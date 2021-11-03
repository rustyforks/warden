//! Environment

/// `Environment` contains all possible runtime environment types.
/// The current value is derived from the `APP_ENVIRONMENT` variable.
///
/// NOTE: `APP_ENVIRONMENT` must be either set to a valid value or left unset.
///   If the parsed value is invalid, the application will not start.
#[derive(Clone, Copy, Debug)]
pub enum Environment {
    Default,
    Staging,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Default => "default",
            Environment::Staging => "staging",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = value.to_lowercase();
        match value.as_str() {
            "default" => Ok(Self::Default),
            "staging" => Ok(Self::Staging),
            "production" => Ok(Self::Production),
            _ => Err("Environment type not supported. \
                Valid values include: 'default', 'staging' and 'production'."),
        }
    }
}
