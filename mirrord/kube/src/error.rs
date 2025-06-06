use std::fmt;

use kube::Resource;
use mirrord_config::target::TargetType;
use thiserror::Error;

pub type Result<T, E = KubeApiError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum KubeApiError {
    #[error(transparent)]
    KubeError(#[from] kube::Error),

    #[error("Connection to agent failed: {0}")]
    KubeConnectionError(#[from] std::io::Error),

    #[error("Failed to infer Kube config: {0}")]
    InferKubeConfigError(#[from] kube::config::InferConfigError),

    #[error("Failed to load Kube config: {0}")]
    KubeConfigPathError(#[from] kube::config::KubeconfigError),

    #[error("Timeout waiting for agent to be ready")]
    AgentReadyTimeout,

    #[error("Port not found in port forward")]
    PortForwardFailed,

    /// This error should never happen, but has to exist if we don't want to unwrap.
    #[error("None runtime data for non-targetless agent. This is a bug.")]
    MissingRuntimeData,

    #[error("Failed to load incluster Kube config: {0}")]
    KubeInclusterError(#[from] kube::config::InClusterError),

    #[error("Path expansion for kubeconfig failed: {0}")]
    ConfigPathExpansionError(String),

    /// We fetched a malformed resource using [`kube`] (should not happen).
    /// Construct with [`Self::missing_field`] or [`Self::invalid_value`] for consistent error
    /// messages.
    #[error("Kube API returned a malformed resource: {0}")]
    MalformedResource(String),

    /// Resource fetched using [`kube`] is in invalid state.
    /// Construct with [`Self::invalid_state`] for consistent error messages.
    #[error("Resource fetched from Kube API is invalid: {0}")]
    InvalidResourceState(String),

    #[error("Agent Job was created, but Pod is not running")]
    AgentPodNotRunning,

    /// Attempted to create an `OperatorTarget` from a resource that cannot be an immediate target.
    ///
    /// Create this variant with the [`KubeApiError::requires_copy`] method.
    #[error("targeting {0} requires the `copy_target` feature")]
    RequiresCopy(
        /// Should be plural name of the resource
        String,
    ),

    /// Attempted to list a specific resource type with `mirrord ls` but was unable to because the
    /// operator was required.
    #[error(
        "The requested resource type `{0}` could not be listed because it requires the operator."
    )]
    TargetTypeRequiresOperator(TargetType),

    /// Attempted to list a specific resource type with `mirrord ls` but was unable to because that
    /// type cannot be listed (for example, cannot list targets of type "targetless".
    #[error("Targets of type `{0}` cannot be listed.")]
    InvalidTargetType(TargetType),

    /// Attempted to list a specific resource type with `mirrord ls` but was unable to because
    /// listing that type has not been implemented.
    #[error("Targets of type `{0}` cannot be listed. This is a bug.")]
    InvalidTargetTypeBug(TargetType),
}

impl KubeApiError {
    /// Use when a resource fetched with [`kube`] is missing some expected field.
    /// Pass full path to the field, e.g. `.spec.selector.matchLabels`.
    ///
    /// Produces [`KubeApiError::MalformedResource`].
    pub fn missing_field<R: Resource<DynamicType = ()>>(resource: &R, field_path: &str) -> Self {
        let kind = R::kind(&());
        let name = resource.meta().name.as_ref();
        let namespace = resource.meta().namespace.as_deref().unwrap_or("default");

        let message = match name {
            Some(name) => format!("{kind} `{namespace}/{name}` is missing field `{field_path}`"),
            None => format!("{kind} is missing field `{field_path}`"),
        };

        Self::MalformedResource(message)
    }

    /// Use when a resource fetched with [`kube`] is has an invalid field (e.g. node IP does not
    /// parse into [`IpAddr`](std::net::IpAddr)). Pass full path to the field, e.g.
    /// `.spec.selector.matchLabels`.
    ///
    /// Produces [`KubeApiError::MalformedResource`].
    pub fn invalid_value<R: Resource<DynamicType = ()>, T: fmt::Display>(
        resource: &R,
        field_path: &str,
        info: T,
    ) -> Self {
        let kind = R::kind(&());
        let name = resource.meta().name.as_ref();
        let namespace = resource.meta().namespace.as_deref().unwrap_or("default");

        let message = match name {
            Some(name) => {
                format!("field `{field_path}` in {kind} `{namespace}/{name}` is invalid: {info}")
            }
            None => format!("field `{field_path}` in a {kind} is invalid: {info}"),
        };

        Self::MalformedResource(message)
    }

    /// Use when a resource fetched with [`kube`] is in invalid state, e.g. node is hosting too many
    /// pods or pod is not running the selected container.
    ///
    /// Produces [`KubeApiError::InvalidResourceState`].
    pub fn invalid_state<R: Resource<DynamicType = ()>, T: fmt::Display>(
        resource: &R,
        info: T,
    ) -> Self {
        let kind = R::kind(&());
        let name = resource.meta().name.as_ref();
        let namespace = resource.meta().namespace.as_deref().unwrap_or("default");

        let message = match name {
            Some(name) => {
                format!("{kind} `{namespace}/{name}` is in invalid state: {info}")
            }
            None => format!("{kind} is in invalid state: {info}"),
        };

        Self::InvalidResourceState(message)
    }

    pub fn requires_copy<R: Resource<DynamicType = ()>>() -> Self {
        Self::RequiresCopy(R::plural(&()).into_owned())
    }
}
