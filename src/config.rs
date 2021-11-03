//! Configuration for the application.

/// This module initializes the application's `Config` struct at startup time.
/// The user-defined configuration is parsed using a combination of file
/// sources and environment variables.
///
/// Source files for the `Config` struct are located, by default, in the
/// __config__ directory. These files use YAML format and are parsed using
/// the derived
/// [`serde::Deserialize`](https://).
use std::fs::File;
use std::io::Read;

use serde::{Deserialize, Serialize};

use crate::environment::Environment;

type Error = Box<dyn std::error::Error + 'static>;
type Result<T, E = Error> = std::result::Result<T, E>;

/// Application server configuration
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct App {
    /// Bind address host
    pub hostname: String,
    /// Port for the service to listen on
    pub port: u16,
}

// TODO: Change `branch` field to `branches`, allowing multiple branches
/// GitHub API configuration
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct GitHub {
    /// Name of git repository to watch
    pub branch: String,
    /// GitHub webhooks API token
    pub api_key: String,
}

/// Top-level configuration struct.
///
/// The `Config` struct contains all required application config.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Config {
    pub app: App,
    pub github: GitHub,
    pub scripts: String,
}

impl Config {
    /// Construct a new `Config` instance from configuration sources.
    pub fn new() -> Result<Self> {
        // If `APP_ENVIRONMENT` is set, use the provided value.
        let environment: Environment = std::env::var("APP_ENVIRONMENT")
            .unwrap_or_else(|_| "default".into())
            .try_into()?;

        println!("environment: {}", environment.as_str());

        Self::parse(environment)
    }

    /// Create a path to a config file based on the `Environment` value.
    fn parse(value: Environment) -> Result<Self> {
        // TODO: Allow both `yml` and `yaml` extension variants
        // TODO: Resolve and validate config file path
        let filename = &*format!("config/{}.yml", value.as_str());
        Self::from_file(filename)
    }

    /// Populate config values based on the type of environment
    fn from_file(filename: &str) -> Result<Self> {
        let mut file = File::open(filename)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        Ok(serde_yaml::from_str::<Self>(&buffer)?)
    }
}
