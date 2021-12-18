use dotenv::dotenv;
use serde::Deserialize;

/// Represents the config
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub database_url: String,
    pub redis_url: String,
    pub jwt_secret: String,
    pub aws_ses_urn: String,
    pub aws_ses_email: String,
}

impl Config {
    /// Gets the environment from .env
    pub fn from_env() -> Self {
        dotenv().ok();

        let mut c = config::Config::new();

        c.merge(config::Environment::default()).unwrap();

        c.try_into()
            .expect("Failed to load configuration from environment")
    }
}
