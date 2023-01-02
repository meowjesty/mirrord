use mirrord_config_derive::MirrordConfig;
use schemars::JsonSchema;
use serde::Deserialize;

use crate::{
    config::{from_env::FromEnv, source::MirrordConfigSource, ConfigError, MirrordConfig},
    util::{MirrordToggleableConfig, VecOrSingle},
};

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
            StealUserConfig::Simple => StealConfig { filter: todo!() },
            StealUserConfig::Advanced(advanced) => advanced.generate_config()?,
        };

        Ok(config)
    }
}

impl MirrordToggleableConfig for StealUserConfig {
    fn disabled_config() -> Result<Self::Generated, ConfigError> {
        todo!()
    }
}

#[derive(MirrordConfig, Default, Clone, PartialEq, Eq, Debug)]
#[config(
    map_to = "AdvancedStealUserConfig",
    derive = "PartialEq,Eq,JsonSchema",
    generator = "StealUserConfig"
)]
pub struct StealConfig {
    #[config(nested)]
    pub filter: Option<String>,
}

impl MirrordToggleableConfig for AdvancedStealUserConfig {
    fn disabled_config() -> Result<Self::Generated, ConfigError> {
        todo!()
    }
}
