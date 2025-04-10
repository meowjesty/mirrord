use std::{
    borrow::Cow,
    collections::BTreeMap,
    convert::Infallible,
    fmt::{self, Display, Formatter},
    future::Future,
    net::IpAddr,
    ops::FromResidual,
    str::FromStr,
};

use k8s_openapi::{
    api::core::v1::{Container, ContainerPort, Node, Pod, Probe},
    apimachinery::pkg::util::intstr::IntOrString,
    NamespaceResourceScope,
};
use kube::{api::ListParams, Api, Client, Resource};
use mirrord_agent_env::mesh::MeshVendor;
use mirrord_config::target::Target;
use serde::de::DeserializeOwned;
use thiserror::Error;
use tracing::Level;

use crate::{
    api::{
        container::{check_mesh_vendor, choose_container},
        kubernetes::get_k8s_resource_api,
    },
    error::{KubeApiError, Result},
    resolved::ResolvedTarget,
};

pub mod cron_job;
pub mod deployment;
pub mod job;
pub mod pod;
pub mod replica_set;
pub mod rollout;
pub mod service;
pub mod stateful_set;

#[derive(Debug)]
pub enum ContainerRuntime {
    Docker,
    Containerd,
    CriO,
}

#[derive(Error, Debug)]
#[error("invalid container runtime name: {0}")]
pub struct ContainerRuntimeParseError(String);

impl FromStr for ContainerRuntime {
    type Err = ContainerRuntimeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "docker" => Ok(Self::Docker),
            "containerd" => Ok(Self::Containerd),
            "cri-o" => Ok(Self::CriO),
            _ => Err(ContainerRuntimeParseError(s.to_string())),
        }
    }
}

impl Display for ContainerRuntime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ContainerRuntime::Docker => write!(f, "docker"),
            ContainerRuntime::Containerd => write!(f, "containerd"),
            ContainerRuntime::CriO => write!(f, "cri-o"),
        }
    }
}

#[derive(Debug)]
pub struct RuntimeData {
    pub pod_name: String,
    pub pod_ips: Vec<IpAddr>,
    pub pod_namespace: String,
    pub node_name: String,
    pub container_id: String,
    pub container_runtime: ContainerRuntime,
    pub container_name: String,
    /// True when no container was specified by the user, but there are multiple containers,
    /// so mirrord chose one of them for the user.
    pub guessed_container: bool,

    /// Used to check if we're running with a mesh/sidecar in `detect_mesh_mirror_mode`.
    pub mesh: Option<MeshVendor>,

    pub share_process_namespace: bool,

    /// Ports where HTTP/gRPC probes are configured
    /// in the target pod.
    pub containers_probe_ports: Vec<u16>,
}

impl RuntimeData {
    /// Extracts data needed to create the mirrord-agent targeting the given [`Pod`].
    /// Verifies that the [`Pod`] is ready to be a target:
    /// 1. pod is in "Running" phase,
    /// 2. pod is not in deletion,
    /// 3. target container is ready.
    pub fn from_pod(pod: &Pod, container_name: Option<&str>) -> Result<Self> {
        let pod_name = pod
            .metadata
            .name
            .as_ref()
            .ok_or_else(|| KubeApiError::missing_field(pod, ".metadata.name"))?
            .to_owned();
        let pod_namespace = pod
            .metadata
            .namespace
            .as_ref()
            .ok_or_else(|| KubeApiError::missing_field(pod, ".metadata.namespace"))?
            .to_owned();

        let phase = pod
            .status
            .as_ref()
            .and_then(|status| status.phase.as_ref())
            .ok_or_else(|| KubeApiError::missing_field(pod, ".status.phase"))?;
        if phase != "Running" {
            return Err(KubeApiError::invalid_state(pod, "not in 'Running' phase"));
        }

        if pod.metadata.deletion_timestamp.is_some() {
            return Err(KubeApiError::invalid_state(pod, "in deletion"));
        }

        let node_name = pod
            .spec
            .as_ref()
            .and_then(|spec| spec.node_name.as_ref())
            .ok_or_else(|| KubeApiError::missing_field(pod, ".spec.nodeName"))?
            .to_owned();

        let containers_probe_ports: Vec<u16> = pod
            .spec
            .as_ref()
            .ok_or_else(|| KubeApiError::missing_field(pod, ".spec"))?
            .containers
            .iter()
            .flat_map(Self::probe_ports)
            .collect();

        let pod_ips = pod
            .status
            .as_ref()
            .and_then(|spec| spec.pod_ips.as_ref())
            .ok_or_else(|| KubeApiError::missing_field(pod, ".status.podIPs"))?
            .iter()
            .filter_map(|pod_ip| {
                pod_ip
                    .ip
                    .parse::<IpAddr>()
                    .inspect_err(|e| {
                        tracing::warn!("failed to parse pod IP {ip}: {e:#?}", ip = pod_ip.ip);
                    })
                    .ok()
            })
            .collect();

        let container_statuses = pod
            .status
            .as_ref()
            .and_then(|status| status.container_statuses.as_ref())
            .ok_or_else(|| KubeApiError::missing_field(pod, ".status.containerStatuses"))?;

        if container_name.is_none() && container_statuses.len() > 1 {
            tracing::trace!(
                "Target has multiple containers and no container name was specified.\
                Now filtering out mesh containers etc."
            );
        }

        let (chosen_container, guessed_container) =
            choose_container(container_name, container_statuses.as_ref());

        if guessed_container {
            tracing::warn!("mirrord picked first eligible container out of many");
        }

        let chosen_status = chosen_container.ok_or_else(|| match container_name {
            Some(name) => KubeApiError::invalid_state(
                pod,
                format_args!("target container `{name}` not found"),
            ),
            None => KubeApiError::invalid_state(pod, "no viable target container found"),
        })?;

        if !chosen_status.ready {
            return Err(KubeApiError::invalid_state(
                pod,
                format_args!("target container `{}` is not ready", chosen_status.name),
            ));
        }

        let container_name = chosen_status.name.clone();
        let container_id_full = chosen_status.container_id.as_ref().ok_or_else(|| {
            KubeApiError::missing_field(pod, ".status.containerStatuses.[].containerID")
        })?;

        let mut split = container_id_full.split("://");

        let (container_runtime, container_id) = match (
            split.next().map(ContainerRuntime::from_str),
            split.next(),
        ) {
            (Some(Ok(runtime)), Some(id)) => (runtime, id.to_string()),
            _ => {
                return Err(KubeApiError::invalid_value(
                    pod,
                    ".status.containerStatuses.[].containerID",
                    format_args!("failed to extract container runtime for `{container_name}`: `{container_id_full}`"),
                ));
            }
        };

        let mesh = check_mesh_vendor(pod);

        Ok(RuntimeData {
            pod_ips,
            pod_name,
            pod_namespace,
            node_name,
            container_id,
            container_runtime,
            container_name,
            guessed_container,
            mesh,
            share_process_namespace: pod
                .spec
                .as_ref()
                .and_then(|spec| spec.share_process_namespace)
                .unwrap_or_default(),
            containers_probe_ports,
        })
    }

    #[tracing::instrument(level = Level::TRACE, skip(client), ret)]
    pub async fn check_node(&self, client: &kube::Client) -> NodeCheck {
        let node_api: Api<Node> = Api::all(client.clone());
        let pod_api: Api<Pod> = Api::all(client.clone());

        let node = node_api.get(&self.node_name).await?;

        let allowed = node
            .status
            .as_ref()
            .and_then(|status| status.allocatable.as_ref())
            .and_then(|allocatable| allocatable.get("pods"))
            .ok_or_else(|| KubeApiError::missing_field(&node, ".status.allocatable.pods"))?
            .0
            .parse::<usize>()
            .map_err(|e| KubeApiError::invalid_value(&node, ".status.allocatable.pods", e))?;

        let mut pod_count = 0;
        let mut list_params = ListParams {
            field_selector: Some(format!(
                "status.phase=Running,spec.nodeName={}",
                self.node_name
            )),
            ..Default::default()
        };

        loop {
            let pods_on_node = pod_api.list(&list_params).await?;

            pod_count += pods_on_node.items.len();

            match pods_on_node.metadata.continue_ {
                Some(next) => {
                    list_params = list_params.continue_token(&next);
                }
                None => break,
            }
        }

        if allowed <= pod_count {
            NodeCheck::Failed(node, pod_count)
        } else {
            NodeCheck::Success
        }
    }

    /// Returns a list of ports used by the container's readiness and liveness probes.
    ///
    /// * Startup probes are ignored: target needs to be ready to start a new session anyway
    /// * TCP probes are ignored: the agent will accept the connection anyway
    fn probe_ports(container: &Container) -> Vec<u16> {
        fn port_to_int(port: &IntOrString, container_ports: &[ContainerPort]) -> Option<u16> {
            match port {
                IntOrString::Int(p) => Some(*p as u16),
                IntOrString::String(port_name) => container_ports
                    .iter()
                    .find(|p| p.name.as_ref() == Some(port_name))
                    .and_then(|p| p.container_port.try_into().ok()),
            }
        }

        fn get_probe_ports(probe: &Probe, container_ports: &[ContainerPort]) -> Vec<u16> {
            let mut ports = Vec::new();

            if let Some(port) = probe
                .http_get
                .as_ref()
                .and_then(|get| port_to_int(&get.port, container_ports))
            {
                ports.push(port);
            }

            if let Some(port) = probe
                .grpc
                .as_ref()
                .and_then(|grpc| grpc.port.try_into().ok())
            {
                ports.push(port);
            }

            ports
        }

        let mut ports = Vec::new();

        if let Some(liveness) = &container.liveness_probe {
            ports.extend(get_probe_ports(
                liveness,
                container.ports.as_deref().unwrap_or_default(),
            ));
        }

        if let Some(readiness) = &container.readiness_probe {
            ports.extend(get_probe_ports(
                readiness,
                container.ports.as_deref().unwrap_or_default(),
            ));
        }

        ports
    }
}

#[derive(Debug)]
pub enum NodeCheck {
    Success,
    Failed(Node, usize),
    Error(KubeApiError),
}

impl<E> FromResidual<Result<Infallible, E>> for NodeCheck
where
    E: Into<KubeApiError>,
{
    fn from_residual(Err(err): Result<Infallible, E>) -> Self {
        NodeCheck::Error(err.into())
    }
}

pub trait RuntimeDataProvider {
    fn runtime_data(
        &self,
        client: &Client,
        namespace: Option<&str>,
    ) -> impl Future<Output = Result<RuntimeData>>;
}

/// Trait for resources that abstract a set of pods
/// defined by a label selector.
///
/// Implementors are provided with an implementation of [`RuntimeDataProvider`].
/// When resolving [`RuntimeData`], the set of pods is fetched and [`RuntimeData`] is extracted from
/// the first pod on the list. If the set is empty, resolution fails.
pub trait RuntimeDataFromLabels {
    type Resource: Resource<DynamicType = (), Scope = NamespaceResourceScope>
        + Clone
        + DeserializeOwned
        + fmt::Debug;

    fn get_selector_match_labels(resource: &Self::Resource) -> Result<BTreeMap<String, String>>;

    /// Returns a list of pods matching the selector of the given `resource`.
    fn get_pods(
        resource: &Self::Resource,
        client: &Client,
    ) -> impl Future<Output = Result<Vec<Pod>>> {
        async {
            let api: Api<<Self as RuntimeDataFromLabels>::Resource> =
                get_k8s_resource_api(client, resource.meta().namespace.as_deref());
            let name = resource
                .meta()
                .name
                .as_deref()
                .ok_or_else(|| KubeApiError::missing_field(resource, ".metadata.name"))?;
            let resource = api.get(name).await?;

            let labels = Self::get_selector_match_labels(&resource)?;

            let formatted_labels = labels
                .iter()
                .map(|(key, value)| format!("{key}={value}"))
                .collect::<Vec<String>>()
                .join(",");
            let list_params = ListParams {
                label_selector: Some(formatted_labels),
                ..Default::default()
            };

            let pod_api: Api<Pod> =
                get_k8s_resource_api(client, resource.meta().namespace.as_deref());
            let pods = pod_api.list(&list_params).await?;

            Ok(pods.items)
        }
    }

    fn name(&self) -> Cow<str>;

    fn container(&self) -> Option<&str>;
}

impl<T> RuntimeDataProvider for T
where
    T: RuntimeDataFromLabels,
{
    async fn runtime_data(&self, client: &Client, namespace: Option<&str>) -> Result<RuntimeData> {
        let api: Api<<Self as RuntimeDataFromLabels>::Resource> =
            get_k8s_resource_api(client, namespace);
        let resource = api.get(&self.name()).await?;
        let pods = Self::get_pods(&resource, client).await?;

        if pods.is_empty() {
            return Err(KubeApiError::invalid_state(
                &resource,
                "no pods matching the labels were found",
            ));
        }

        pods.iter()
            .filter_map(|pod| RuntimeData::from_pod(pod, self.container()).ok())
            .next()
            .ok_or_else(|| {
                KubeApiError::invalid_state(
                    &resource,
                    "no pod matching the labels is ready to be targeted",
                )
            })
    }
}

impl RuntimeDataProvider for Target {
    async fn runtime_data(&self, client: &Client, namespace: Option<&str>) -> Result<RuntimeData> {
        match self {
            Target::Deployment(deployment) => deployment.runtime_data(client, namespace).await,
            Target::Pod(target) => target.runtime_data(client, namespace).await,
            Target::Rollout(target) => target.runtime_data(client, namespace).await,
            Target::Job(target) => target.runtime_data(client, namespace).await,
            Target::CronJob(target) => target.runtime_data(client, namespace).await,
            Target::StatefulSet(target) => target.runtime_data(client, namespace).await,
            Target::Service(target) => target.runtime_data(client, namespace).await,
            Target::ReplicaSet(target) => target.runtime_data(client, namespace).await,
            Target::Targetless => Err(KubeApiError::MissingRuntimeData),
        }
    }
}

impl RuntimeDataProvider for ResolvedTarget<true> {
    async fn runtime_data(&self, client: &Client, namespace: Option<&str>) -> Result<RuntimeData> {
        match self {
            Self::Deployment(target) => target.runtime_data(client, namespace).await,
            Self::Pod(target) => target.runtime_data().await,
            Self::Rollout(target) => target.runtime_data(client, namespace).await,
            Self::Job(target) => target.runtime_data(client, namespace).await,
            Self::CronJob(target) => target.runtime_data(client, namespace).await,
            Self::StatefulSet(target) => target.runtime_data(client, namespace).await,
            Self::Service(target) => target.runtime_data(client, namespace).await,
            Self::ReplicaSet(target) => target.runtime_data(client, namespace).await,
            Self::Targetless(_) => Err(KubeApiError::MissingRuntimeData),
        }
    }
}

#[cfg(test)]
mod tests {
    use mirrord_config::target::{
        deployment::DeploymentTarget, job::JobTarget, pod::PodTarget, service::ServiceTarget,
    };
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("pod/foobaz", Target::Pod(PodTarget {pod: "foobaz".to_string(), container: None}))]
    #[case("deployment/foobaz", Target::Deployment(DeploymentTarget {deployment: "foobaz".to_string(), container: None}))]
    #[case("deployment/nginx-deployment", Target::Deployment(DeploymentTarget {deployment: "nginx-deployment".to_string(), container: None}))]
    #[case("pod/foo/container/baz", Target::Pod(PodTarget { pod: "foo".to_string(), container: Some("baz".to_string()) }))]
    #[case("deployment/nginx-deployment/container/container-name", Target::Deployment(DeploymentTarget {deployment: "nginx-deployment".to_string(), container: Some("container-name".to_string())}))]
    #[case("job/foo", Target::Job(JobTarget { job: "foo".to_string(), container: None }))]
    #[case("job/foo/container/baz", Target::Job(JobTarget { job: "foo".to_string(), container: Some("baz".to_string()) }))]
    #[case("service/foo", Target::Service(ServiceTarget { service: "foo".into(), container: None }))]
    #[case("service/foo/container/baz", Target::Service(ServiceTarget { service: "foo".into(), container: Some("baz".into()) }))]
    fn target_parses(#[case] target: &str, #[case] expected: Target) {
        let target = target.parse::<Target>().unwrap();
        assert_eq!(target, expected)
    }

    #[allow(clippy::duplicated_attributes)]
    #[rstest]
    #[should_panic(expected = "InvalidTarget")]
    #[case::panic("deployment/foobaz/blah")]
    #[should_panic(expected = "InvalidTarget")]
    #[case::panic("pod/foo/baz")]
    fn target_parse_fails(#[case] target: &str) {
        let target = target.parse::<Target>().unwrap();
        assert_eq!(
            target,
            Target::Deployment(DeploymentTarget {
                deployment: "foobaz".to_string(),
                container: None
            })
        )
    }
}
