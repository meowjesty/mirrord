use std::str::FromStr;

use schemars::JsonSchema;
use serde::Deserialize;
use thiserror::Error;

use self::steal::StealModeConfig;
use crate::config::{ConfigError, MirrordConfig};

pub mod steal;

/// Controls the mode of operation for incoming traffic.
///
/// Defaults to [`IncomingConfig::Mirror`].
///
/// See the incoming [reference](https://mirrord.dev/docs/reference/traffic/#incoming) for more
/// details.
///
/// Incoming traffic supports 2 modes of operation:
///
/// 1. Mirror: Sniffs the TCP data from a port, and forwards a copy to the interested listeners;
///
/// 2. Steal: Captures the TCP data from a port, and forwards it (depending on how it's configured,
/// see [`StealModeConfig`]);
///
/// ## Examples
///
/// - Mirror any incoming traffic:
///
/// ```toml
/// # mirrord-config.toml
///
/// [feature.network]
/// incoming = "mirror"    # for illustration purporses, it's the default
/// ```
///
/// - Steal incoming HTTP traffic, if the HTTP header matches "token.*" (supports regex):
///
/// ```yaml
/// # mirrord-config.yaml
///
/// [feature.network,incoming]
/// http_filter = "token.*"
/// ```
#[derive(Deserialize, PartialEq, Eq, Clone, Debug, JsonSchema, Default)]
#[serde(rename_all = "lowercase")]
pub enum IncomingFileConfig {
    /// Sniffs on TCP port, and send a copy of the data to listeners.
    #[default]
    Mirror,

    /// Stealer supports 2 modes of operation:
    ///
    /// 1. Port traffic stealing: Steals all TCP data from a port, which is selected whenever the
    /// user listens in a TCP socket (enabling the feature is enough to make this work, no
    /// additional configuration is needed);
    ///
    /// 2. HTTP traffic stealing: Steals only HTTP traffic, mirrord tries to detect if the incoming
    /// data on a port is HTTP (in a best-effort kind of way, not guaranteed to be HTTP), and
    /// steals the traffic on the port if it is HTTP;
    Steal(Option<String>),
}

#[derive(Error, Debug)]
#[error("could not parse IncomingConfig from string, values must be bool or mirror/steal")]
pub struct IncomingConfigParseError;

impl MirrordConfig for IncomingFileConfig {
    type Generated = IncomingFileConfig;

    fn generate_config(self) -> Result<Self::Generated, ConfigError> {
        Ok(self)
    }
}

impl IncomingFileConfig {
    /// Helper function.
    ///
    /// Used by mirrord-layer to identify the incoming network configuration as steal or not.
    pub fn is_steal(&self) -> bool {
        matches!(self, IncomingFileConfig::Steal(_))
    }
}
