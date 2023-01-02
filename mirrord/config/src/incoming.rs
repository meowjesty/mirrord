use std::str::FromStr;

use schemars::JsonSchema;
use serde::Deserialize;
use thiserror::Error;

use self::steal::StealModeConfig;

pub mod steal;

#[derive(Deserialize, PartialEq, Eq, Clone, Debug, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum IncomingConfig {
    Mirror,
    Steal(StealModeConfig),
}

impl Default for IncomingConfig {
    fn default() -> Self {
        IncomingConfig::Mirror
    }
}

#[derive(Error, Debug)]
#[error("could not parse IncomingConfig from string, values must be bool or mirror/steal")]
pub struct IncomingConfigParseError;

impl FromStr for IncomingConfig {
    type Err = IncomingConfigParseError;

    fn from_str(val: &str) -> Result<Self, Self::Err> {
        match val.parse::<bool>() {
            Ok(true) => Ok(IncomingConfig::Steal(StealModeConfig::disabled_config()?)),
            Ok(false) => Ok(IncomingConfig::Mirror),
            Err(_) => match val {
                "steal" => Ok(IncomingConfig::Steal(StealModeConfig::disabled_config()?)),
                "mirror" => Ok(IncomingConfig::Mirror),
                _ => Err(IncomingConfigParseError),
            },
        }
    }
}

impl IncomingConfig {
    pub fn is_steal(&self) -> bool {
        matches!(self, IncomingConfig::Steal(_))
    }
}
