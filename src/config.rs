//! Configuration for the application.
//!
//! This module populates the application `Config` struct at startup time.
//! The user-defined configuration is parsed using a combination of file
//! sources and environment variables.
//!
//! Source files for the `Config` struct are located, by default, in the
//! __config__ directory. The only currently supported format is YAML.
use std::fs::File;
use std::io::Read;
use std::net::SocketAddr;
use std::path::Path;

use serde::Deserialize;

use crate::environment::Environment;

type Error = Box<dyn std::error::Error + 'static>;
type Result<T, E = Error> = std::result::Result<T, E>;

/// Application server configuration
#[derive(Debug, Default, Deserialize, PartialEq)]
pub struct App {
    /// Bind address host
    pub hostname: String,
    /// Port for the service to listen on
    pub port: u16,
}

// TODO: Change `branch` field to `branches`, allowing multiple branches
/// GitHub API configuration
#[derive(Debug, Default, Deserialize, PartialEq)]
pub struct GitHub {
    /// Name of git repository to watch
    pub branch: String,
    /// GitHub webhooks API token
    pub api_key: String,
}

/// Top-level configuration struct.
///
/// The `Config` struct contains all required application config.
#[derive(Debug, Deserialize, PartialEq)]
pub struct Config {
    pub app: App,
    pub github: GitHub,
    pub scripts: String,
}

impl Config {
    /// Construct a new `Config` instance from configuration sources.
    pub fn new() -> Result<Self> {
        let environment: Environment = std::env::var("APP_ENV")
            .unwrap_or_else(|_| "default".into())
            .try_into()?;
        Self::parse(environment)
    }

    /// Create a path to a config file based on the `Environment` value.
    fn parse(value: Environment) -> Result<Self> {
        let filename = Path::new(value.as_str()).with_extension("yml");
        let current_dir = std::env::current_dir()?;
        let config_dir = current_dir.join("config");

        Self::from_path(&config_dir.join(filename))
    }

    /// Populate config values based on the type of environment
    fn from_path(filename: &Path) -> Result<Self> {
        let mut file = File::open(filename)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        Ok(serde_yaml::from_str::<Self>(&buffer)?)
    }

    pub fn bind_address(&self) -> SocketAddr {
        let socket_addr = format!("{}:{}", self.app.hostname, self.app.port);
        socket_addr
            .parse()
            .expect("Failed to parse hostname and port into valid bind address")
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::parse(Environment::Default).expect("Failed to load default config")
    }
}
