use std::{collections::HashMap, ops::Not, sync::Arc};

use tokio_util::sync::CancellationToken;
use tracing::{Level, error, warn};

use crate::{
    cli::{Args, CliAndEnvArgs},
    entrypoint::{
        DIRTY_IPTABLES_CLEANUP_WARNING_MESSAGE, DIRTY_IPTABLES_ERROR_MESSAGE, check_existing_rules,
        monitor_main_container, notify_client_about_dirty_iptables,
    },
    error::{AgentError, AgentResult},
    runtime::{Container, ContainerInfo, ContainerRuntime},
    task::BgTaskRuntime,
};

#[derive(Debug)]
struct Inner {
    /// Cached process ID of the container.
    pid: u64,
    /// Cached environment of the container.
    raw_env: HashMap<String, String>,
}

/// Handle to the container targeted by the agent.
/// Exposes some cached info about the container and allows pausing it according to clients'
/// requests.
#[derive(Debug, Clone)]
pub(crate) struct ContainerHandle(Arc<Inner>);

impl ContainerHandle {
    /// Retrieve info about the container and initialize this struct.
    #[tracing::instrument(level = Level::DEBUG, ret, err)]
    pub(crate) async fn new(container: Container) -> AgentResult<Self> {
        let ContainerInfo { pid, env: raw_env } = container.get_info().await?;

        let inner = Inner { pid, raw_env };

        Ok(Self(inner.into()))
    }

    /// Return the process ID of the container.
    pub(crate) fn pid(&self) -> u64 {
        self.0.pid
    }

    /// Return environment variables from the container.
    pub(crate) fn raw_env(&self) -> &HashMap<String, String> {
        &self.0.raw_env
    }
}

#[derive(Clone)]
pub(crate) struct ContainerThingy {
    pub(crate) handle: ContainerHandle,
    pub(crate) bg_task_runtime: Arc<BgTaskRuntime>,
    pub(crate) env: Arc<HashMap<String, String>>,
}

impl ContainerThingy {
    pub(super) async fn check_container_chain_names_for_conflict(
        &self,
        CliAndEnvArgs {
            args:
                Args {
                    ipv6,
                    clean_iptables_on_start,
                    communication_timeout,
                    ..
                },
            ..
        }: &CliAndEnvArgs,
        listener: tokio::net::TcpListener,
        tls_connector: Option<crate::client_connection::AgentTlsConnector>,
        cancellation_token: CancellationToken,
    ) -> AgentResult<()> {
        let _rt = self.bg_task_runtime.handle().enter();

        let leftover_rules =
            tokio::spawn(check_existing_rules(*ipv6, *clean_iptables_on_start, false))
                .await
                .map_err(|error| AgentError::IPTablesSetupError(error.into()))?
                .map_err(|error| AgentError::IPTablesSetupError(error.into()))?;

        if leftover_rules.is_empty().not() {
            if *clean_iptables_on_start {
                warn!(
                    leftover_rules = ?leftover_rules,
                    "{}",
                    DIRTY_IPTABLES_CLEANUP_WARNING_MESSAGE
                );
            } else {
                error!(
                    leftover_rules = ?leftover_rules,
                    "{}",
                    DIRTY_IPTABLES_ERROR_MESSAGE
                );

                let _ = notify_client_about_dirty_iptables(
                    listener,
                    *communication_timeout,
                    tls_connector,
                )
                .await;
                return Err(AgentError::IPTablesDirty);
            }
        }

        // Casting u64 to i32 but linux pids shouldn't exceed 2^22
        let pid = self.handle.pid().try_into().unwrap();
        monitor_main_container(cancellation_token.clone(), pid);

        Ok(())
    }
}
