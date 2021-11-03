use std::fs::File;
use std::io::Read;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

pub type StdError = Box<dyn std::error::Error + 'static>;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct App {
    /// Bind address host [default: 127.0.0.1]
    pub hostname: String,
    /// Port for service to listen on [default: 16353]
    pub port: u16,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct GitHub {
    /// Name of git repository to watch [default: main]
    pub branch: String,
    /// GitHub webhooks API token [default: ""]
    pub api_key: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Config {
    pub app: App,
    pub github: GitHub,
    /// Directory containing loadable scripts
    pub scripts: String,
}

impl Config {
    /// Construct a new `Config` instance from configuration sources.
    pub fn new() -> Result<Self, StdError> {
        let environment: Environment = std::env::var("APP_ENVIRONMENT")
            .unwrap_or_else(|_| "default".into())
            .parse()?;
        Self::from_env(environment)
    }

    pub fn from_env(value: Environment) -> Result<Config, StdError> {
        let filename = &*format!("config/{}.yml", value.as_str());
        Config::parse_file(filename)
    }

    /// Populate config values based on the type of environment
    fn parse_file(filename: &str) -> Result<Self, StdError> {
        let mut file = File::open(filename)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        Ok(serde_yaml::from_str::<Config>(&buffer)?)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            scripts: "scripts".to_string(),
            app: App {
                hostname: "127.0.0.1".to_string(),
                port: 15550,
            },
            github: GitHub {
                branch: "main".to_string(),
                api_key: "".to_string(),
            },
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Environment {
    Default,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Default => "default",
            Environment::Production => "production",
        }
    }
}

impl FromStr for Environment {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "default" => Ok(Self::Default),
            "production" => Ok(Self::Production),
            _ => {
                eprintln!("Environment type '{}' not supported.", s);
                Err("default")
            }
        }
    }
}
