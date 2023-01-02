use mirrord_config_derive::MirrordConfig;
use schemars::JsonSchema;
use serde::Deserialize;

use crate::{
    config::{from_env::FromEnv, source::MirrordConfigSource, ConfigError, MirrordConfig},
    util::MirrordToggleableConfig,
};

const FILTER_ENV_VAR: &str = "MIRRORD_HTTP_TRAFFIC_FILTER";

#[derive(Deserialize, PartialEq, Eq, Clone, Debug, JsonSchema)]
#[serde(untagged, rename_all = "lowercase")]
pub enum StealUserConfig {
    Simple,
    Advanced(AdvancedStealUserConfig),
}

impl Default for StealUserConfig {
    fn default() -> Self {
        StealUserConfig::Simple
    }
}

impl MirrordConfig for StealUserConfig {
    type Generated = StealConfig;

    fn generate_config(self) -> Result<Self::Generated, ConfigError> {
        let config = match self {
            StealUserConfig::Simple => StealConfig { filter: None },
            StealUserConfig::Advanced(advanced) => advanced.generate_config()?,
        };

        Ok(config)
    }
}

#[derive(MirrordConfig, Default, Clone, PartialEq, Eq, Debug)]
#[config(
    map_to = "AdvancedStealUserConfig",
    derive = "PartialEq,Eq,JsonSchema",
    generator = "StealUserConfig"
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
