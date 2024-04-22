use core::fmt;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::FromSplit;
use crate::config::{self, ConfigError};

/// <!--${internal}-->
/// Mirror the cron_job specified by [`CronJobTarget::cron_job`].
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Debug, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct CronJobTarget {
    /// <!--${internal}-->
    /// CronJob to mirror.
    pub cronjob: String,
    pub container: Option<String>,
}

impl fmt::Display for CronJobTarget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}",
            self.container
                .as_ref()
                .map(|c| format!("{c}/"))
                .unwrap_or_default(),
            self.cronjob.clone()
        )
    }
}

impl FromSplit for CronJobTarget {
    fn from_split(split: &mut std::str::Split<char>) -> config::Result<Self> {
        let cron_job = split
            .next()
            .ok_or_else(|| ConfigError::InvalidTarget("CronJob failure 1".to_string()))?;
        match (split.next(), split.next()) {
            (Some("container"), Some(container)) => Ok(Self {
                cronjob: cron_job.to_string(),
                container: Some(container.to_string()),
            }),
            (None, None) => Ok(Self {
                cronjob: cron_job.to_string(),
                container: None,
            }),
            _ => Err(ConfigError::InvalidTarget("CronJob Failure 2".to_string())),
        }
    }
}
