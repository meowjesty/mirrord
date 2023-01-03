use mirrord_config_derive::MirrordConfig;
use schemars::JsonSchema;
use serde::Deserialize;

use crate::{
    config::{from_env::FromEnv, source::MirrordConfigSource, ConfigError, MirrordConfig},
    util::MirrordToggleableConfig,
};

/// Environment variable used to activate the HTTP traffic stealer feature.
const FILTER_ENV_VAR: &str = "MIRRORD_HTTP_TRAFFIC_FILTER";

/// Configuration mode for the stealer feature.
///
/// Defaults to [`StealModeConfig::Simple`].
///
/// See the stealing [reference](https://mirrord.dev/docs/reference/traffic/#stealing) for more
/// details.
///
/// Stealer supports 2 modes of operation:
///
/// 1. Port traffic stealing: Steals all TCP data from a port, which is selected whenever the
/// user listens in a TCP socket (enabling the feature is enough to make this work, no additional
/// configuration is needed);
///
/// 2. HTTP traffic stealing: Steals only HTTP traffic, mirrord tries to detect if the incoming data
/// on a port is HTTP (in a best-effort kind of way, not guaranteed to be HTTP), and steals the
/// traffic on the port if it is HTTP;
#[derive(Deserialize, PartialEq, Eq, Clone, Debug, JsonSchema, Default)]
#[serde(untagged, rename_all = "lowercase")]
pub enum StealModeConfig {
    /// Steal any traffic on TCP ports.
    #[default]
    Simple,

    /// Allows the user more fine-grained control over the kind of traffic to steal on TCP ports.
    ///
    /// Currently only supports HTTP traffic.
    Advanced(AdvancedStealUserConfig),
}

impl MirrordConfig for StealModeConfig {
    type Generated = StealConfig;

    fn generate_config(self) -> Result<Self::Generated, ConfigError> {
        let config = match self {
            StealModeConfig::Simple => StealConfig { filter: None },
            StealModeConfig::Advanced(advanced) => advanced.generate_config()?,
        };

        Ok(config)
    }
}

#[derive(MirrordConfig, Default, Clone, PartialEq, Eq, Debug)]
#[config(
    map_to = "AdvancedStealUserConfig",
    derive = "PartialEq,Eq,JsonSchema",
    generator = "StealModeConfig"
)]
pub struct StealConfig {
    pub filter: Option<String>,
}

impl MirrordToggleableConfig for AdvancedStealUserConfig {
    fn disabled_config() -> Result<Self::Generated, ConfigError> {
        let filter = FromEnv::new(FILTER_ENV_VAR).source_value().transpose()?;

        Ok(Self::Generated { filter })
    }
}
