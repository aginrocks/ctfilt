use color_eyre::{Section as _, eyre::Context as _};
use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumString, IntoStaticStr};
use tracing::warn;

const ENV_PREFIX: &str = "ASSASSIN";
const ENV_SEPARATOR: &str = "__";

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum KubernetesMode {
    InCluster,
    Kubeconfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Kubernetes {
    pub mode: KubernetesMode,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Labels {
    pub prefix: String,
    pub delete_related: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Settings {
    pub kubernetes: Kubernetes,
    pub labels: Labels,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let custom_config_file = env_var("CONFIG_FILE").ok();
        let environment_type = get_environment_type();

        if environment_type == EnvironmentType::Development {
            warn!("Running in development mode");
        }

        let mut settings = Config::builder()
            .add_source(File::with_name("config").required(custom_config_file.is_none()));

        if let Some(file) = custom_config_file {
            settings = settings.add_source(File::with_name(&file));
        }

        settings = settings
            .add_source(File::with_name(&format!("config-{environment_type}")).required(false))
            .add_source(File::with_name("config-local").required(false))
            .add_source(Environment::with_prefix(ENV_PREFIX).separator(ENV_SEPARATOR));

        settings.build()?.try_deserialize()
    }

    pub fn try_load() -> color_eyre::Result<Self> {
        let res = Settings::new();

        let add_suggestion = matches!(
            &res,
            Err(ConfigError::Foreign(foreign_error))
                if matches!(
                    foreign_error.downcast_ref::<std::io::Error>(),
                    Some(io_error)
                        if io_error.kind() == std::io::ErrorKind::NotFound
                            && io_error.get_ref().is_some_and(|custom_error| {
                                let custom_error = custom_error.to_string();
                                custom_error.starts_with("configuration file \"")
                                    && custom_error.ends_with("\" not found")
                            })
                )
        );

        let mut res = res.wrap_err("failed to load settings");

        if add_suggestion && !std::path::Path::new("config.toml").exists() {
            let example_settings = Settings::example();
            let example_settings = toml::to_string_pretty(&example_settings)?;

            std::fs::write("config.toml", example_settings)?;

            res = res.suggestion("An example configuration file has been created at `config.toml` in the current directory.");
        }

        res
    }

    pub fn example() -> Self {
        Self {
            kubernetes: Kubernetes {
                mode: KubernetesMode::Kubeconfig,
            },
            labels: Labels {
                prefix: "pod-assassin.agin.rocks".to_string(),
                delete_related: true,
            },
        }
    }
}

#[derive(Debug, Eq, PartialEq, EnumString, Display, AsRefStr, IntoStaticStr)]
#[strum(ascii_case_insensitive, serialize_all = "snake_case")]
enum EnvironmentType {
    #[strum(serialize = "development", serialize = "dev", serialize = "d")]
    Development,

    #[strum(serialize = "production", serialize = "prod", serialize = "p")]
    Production,
}

fn get_environment_type() -> EnvironmentType {
    let from_env = env_var("ENVIRONMENT")
        .inspect_err(|err| {
            if let std::env::VarError::NotUnicode(_) = err {
                warn!(
                    "Environment variable '{}' is not valid unicode",
                    env_name("ENVIRONMENT")
                );
            }
        })
        .ok()
        .map(|val| val.trim().to_string());

    let from_env = from_env.and_then(|env| env.parse::<EnvironmentType>().inspect_err(|err| {
        warn!(error = ?err, "Environment variable '{}' is not a valid environment type", env_name("ENVIRONMENT"));
    }).ok());

    from_env.unwrap_or({
        if cfg!(debug_assertions) {
            EnvironmentType::Development
        } else {
            EnvironmentType::Production
        }
    })
}

pub fn env_name(name: &str) -> String {
    format!("{}{}{}", ENV_PREFIX, ENV_SEPARATOR, name.to_uppercase())
}

pub fn env_var(name: &str) -> Result<String, std::env::VarError> {
    std::env::var(env_name(name))
}
