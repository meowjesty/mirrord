pub mod context;
pub mod deprecated;
pub mod from_env;
pub mod source;
pub mod unstable;

use std::error::Error;

pub use context::ConfigContext;
use thiserror::Error;

use crate::feature::split_queues::QueueSplittingVerificationError;

/// <!--${internal}-->
/// Error that would be returned from [MirrordConfig::generate_config]
#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("invalid target provided `{0}`!")]
    InvalidTarget(String),

    #[error("value for {1:?} not provided in {0:?} (env override {2:?})")]
    ValueNotProvided(&'static str, &'static str, Option<&'static str>),

    #[error("invalid {} value `{}`: {}", .name, .provided, .error)]
    InvalidValue {
        // Name of parsed env var or field path in the config.
        name: &'static str,
        // Value provided by the user.
        provided: String,
        // Error that occurred when processing the value.
        error: Box<dyn Error + Send + Sync>,
    },

    #[error("mirrord-config: IO operation failed with `{0}`")]
    Io(#[from] std::io::Error),

    #[error("mirrord-config: `{0}`!")]
    SerdeJson(#[from] serde_json::Error),

    #[error("mirrord-config: `{0}`!")]
    Toml(#[from] toml::de::Error),

    #[error("mirrord-config: `{0}`!")]
    SerdeYaml(#[from] serde_yaml::Error),

    #[error("mirrord-config: Unsupported configuration file format!")]
    UnsupportedFormat,

    #[error("Invalid FS mode `{0}`!")]
    InvalidFsMode(String),

    #[error("Conflicting configuration found `{0}`")]
    Conflict(String),

    #[error(
        "A target namespace was specified, but no target was specified. If you want to set the \
        namespace in which the agent will be created, please set the agent namespace, not the \
        target namespace. That value can be set with agent.namespace in the configuration file, \
        the -a argument of the CLI, or the MIRRORD_AGENT_NAMESPACE environment variable.

        If you are not trying to run targetless, please specify a target instead."
    )]
    TargetNamespaceWithoutTarget,

    #[error(
        "A Job or CronJob target has been specified, but the feature `copy_target` has not been enabled!

        If you want to target a job or cronjob, please enable `copy_target` feature in the `feature` section.
        "
    )]
    TargetJobWithoutCopyTarget,

    #[error("Template rendering failed with: `{0}`! Please check your config file!")]
    TemplateRenderingFailed(String),

    #[error("Target type requires the mirrord-operator, but operator usage was explicitly disabled. Consider enabling mirrord-operator in your mirrord config.")]
    TargetRequiresOperator,

    #[error("Queue splitting config is invalid: {0}")]
    QueueSplittingVerificationError(#[from] QueueSplittingVerificationError),

    /// When preparing the `EnvVarsRemapper`, regex creation may fail.
    #[error(
        "Regex creation for pattern `{pattern}: {value}` in `config.feature.env.mapping` failed with: `{fail}`"
    )]
    Regex {
        pattern: String,
        value: String,
        fail: Box<fancy_regex::Error>,
    },

    #[error("Decoding resolved config failed: {0}")]
    DecodeError(String),

    #[error("Encoding resolved config failed: {0}")]
    EncodeError(String),
}

impl From<tera::Error> for ConfigError {
    fn from(fail: tera::Error) -> Self {
        let mut fail_message = fail.to_string();
        let mut source = fail.source();

        while let Some(fail_source) = source {
            fail_message.push_str(&format!(" -> {fail_source}"));
            source = fail_source.source();
        }

        Self::TemplateRenderingFailed(fail_message)
    }
}

pub type Result<T, E = ConfigError> = std::result::Result<T, E>;

/// <!--${internal}-->
/// Main configuration creation trait of mirrord-config
pub trait MirrordConfig {
    /// <!--${internal}-->
    /// The resulting struct you plan on using in the rest of your code
    type Generated;

    /// <!--${internal}-->
    /// Load configuration from all sources and output as `Self::Generated`
    /// Pass reference to list of warnings which callee can add warnings into.
    fn generate_config(self, context: &mut ConfigContext) -> Result<Self::Generated>;
}

impl<T> MirrordConfig for Option<T>
where
    T: MirrordConfig + Default,
{
    type Generated = T::Generated;

    fn generate_config(self, context: &mut ConfigContext) -> Result<Self::Generated> {
        self.unwrap_or_default().generate_config(context)
    }
}

/// <!--${internal}-->
/// Lookup trait for accessing type implementing [MirrordConfig] from [MirrordConfig::Generated]
pub trait FromMirrordConfig {
    type Generator: MirrordConfig;
}
