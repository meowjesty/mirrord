use std::{ops::Not, sync::LazyLock};

use futures::{AsyncBufReadExt, TryStreamExt};
use k8s_openapi::api::core::v1::{EnvVar, Pod, Toleration};
use kube::{api::LogParams, Api};
use mirrord_agent_env::envs;
use mirrord_config::agent::{AgentConfig, LinuxCapability};
use regex::Regex;
use tracing::warn;

use crate::{api::container::ContainerParams, error::Result};

static AGENT_READY_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new("agent ready( - version (\\S+))?").expect("failed to create regex")
});

pub(super) static DEFAULT_TOLERATIONS: LazyLock<Vec<Toleration>> = LazyLock::new(|| {
    vec![Toleration {
        operator: Some("Exists".to_owned()),
        ..Default::default()
    }]
});

/// Retrieve a list of Linux capabilities for the agent container.
pub(super) fn get_capabilities(agent: &AgentConfig) -> Vec<LinuxCapability> {
    let disabled = agent.disabled_capabilities.clone().unwrap_or_default();

    LinuxCapability::all()
        .iter()
        .copied()
        .filter(|c| disabled.contains(c).not())
        .filter(|c| {
            matches!(c, LinuxCapability::NetRaw if agent.passthrough_mirroring.unwrap_or_default())
                .not()
        })
        .collect()
}

/// Builds mirrord agent environment variables.
pub(super) fn agent_env(agent: &AgentConfig, params: &ContainerParams) -> Vec<EnvVar> {
    let mut env = vec![
        envs::LOG_LEVEL.as_k8s_spec(&agent.log_level),
        envs::STEALER_FLUSH_CONNECTIONS.as_k8s_spec(&agent.flush_connections),
        envs::JSON_LOG.as_k8s_spec(&agent.json_log),
        envs::IPV6_SUPPORT.as_k8s_spec(&params.support_ipv6),
        envs::PASSTHROUGH_MIRRORING.as_k8s_spec(&agent.passthrough_mirroring.unwrap_or_default()),
    ];

    if let Some(nftables) = agent.nftables {
        env.push(envs::NFTABLES.as_k8s_spec(&nftables));
    }

    if let Some(attempts) = agent.dns.attempts {
        env.push(envs::DNS_ATTEMPTS.as_k8s_spec(&attempts));
    }

    if let Some(interface) = agent.network_interface.as_ref() {
        env.push(envs::NETWORK_INTERFACE.as_k8s_spec(interface));
    }

    if let Some(timeout) = agent.dns.timeout {
        env.push(envs::DNS_TIMEOUT.as_k8s_spec(&timeout));
    };

    if let Some(pod_ips) = &params.pod_ips {
        env.push(envs::POD_IPS.as_k8s_spec(pod_ips));
    }

    if let Some(metrics_address) = agent.metrics.as_ref() {
        env.push(envs::METRICS.as_k8s_spec(metrics_address));
    }

    if let Some(cert) = &params.tls_cert {
        env.push(envs::OPERATOR_CERT.as_k8s_spec(cert));
    }

    if params.steal_tls_config.is_empty().not() {
        env.push(envs::STEAL_TLS_CONFIG.as_k8s_spec(&params.steal_tls_config));
    }

    env
}

pub(super) fn base_command_line(agent: &AgentConfig, params: &ContainerParams) -> Vec<String> {
    let mut command_line = vec![
        "./mirrord-agent".to_owned(),
        "-l".to_owned(),
        params.port.to_string(),
    ];
    if let Some(timeout) = agent.communication_timeout {
        command_line.push("-t".to_owned());
        command_line.push(timeout.to_string());
    }

    #[cfg(debug_assertions)]
    if agent.test_error {
        command_line.push("--test-error".to_owned());
    }

    command_line
}

/**
 * Wait until the agent prints the "agent ready" message.
 * Return agent version extracted from the message (if found).
 */
#[tracing::instrument(level = "trace", skip(pod_api), ret)]
pub(super) async fn wait_for_agent_startup(
    pod_api: &Api<Pod>,
    pod_name: &str,
    container_name: String,
) -> Result<Option<String>> {
    let logs = pod_api
        .log_stream(
            pod_name,
            &LogParams {
                follow: true,
                container: Some(container_name),
                ..LogParams::default()
            },
        )
        .await?;

    let mut lines = logs.lines();
    while let Some(line) = lines.try_next().await? {
        let Some(captures) = AGENT_READY_REGEX.captures(&line) else {
            continue;
        };

        let version = captures.get(2).map(|m| m.as_str().to_string());
        return Ok(version);
    }

    warn!("Agent did not print 'agent ready' message");
    Ok(None)
}

#[cfg(test)]
mod test {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("agent ready", None)]
    #[case("agent ready - version 3.56.0", Some("3.56.0"))]
    fn agent_version_regex(#[case] agent_message: &str, #[case] version: Option<&str>) {
        let captures = AGENT_READY_REGEX.captures(agent_message).unwrap();

        assert_eq!(captures.get(2).map(|c| c.as_str()), version);
    }
}
