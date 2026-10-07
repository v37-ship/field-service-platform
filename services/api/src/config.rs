use std::collections::HashMap;
use std::env;
use std::net::SocketAddr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppConfig {
    pub bind_addr: SocketAddr,
    pub database_url: String,
    pub environment: String,
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ConfigError {
    #[error("missing required environment variable: {0}")]
    MissingVar(&'static str),
    #[error("invalid bind address `{value}`: {source}")]
    InvalidBindAddress {
        value: String,
        source: std::net::AddrParseError,
    },
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        Self::from_vars(env::vars())
    }

    pub fn from_vars<I, K, V>(vars: I) -> Result<Self, ConfigError>
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        let env_map: HashMap<String, String> = vars
            .into_iter()
            .map(|(key, value)| (key.into(), value.into()))
            .collect();

        let bind_addr_raw = env_map
            .get("API_BIND_ADDR")
            .cloned()
            .unwrap_or_else(|| "0.0.0.0:3000".to_string());
        let bind_addr =
            bind_addr_raw
                .parse()
                .map_err(|source| ConfigError::InvalidBindAddress {
                    value: bind_addr_raw,
                    source,
                })?;

        let database_url = env_map
            .get("DATABASE_URL")
            .cloned()
            .ok_or(ConfigError::MissingVar("DATABASE_URL"))?;

        let environment = env_map
            .get("APP_ENV")
            .cloned()
            .unwrap_or_else(|| "development".to_string());

        Ok(Self {
            bind_addr,
            database_url,
            environment,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{AppConfig, ConfigError};

    #[test]
    fn parses_config_with_defaults() {
        let config =
            AppConfig::from_vars([("DATABASE_URL", "postgres://localhost:5432/field_service")])
                .expect("config should parse");

        assert_eq!(config.bind_addr.to_string(), "0.0.0.0:3000");
        assert_eq!(config.environment, "development");
    }

    #[test]
    fn validates_required_database_url() {
        let err = AppConfig::from_vars([("API_BIND_ADDR", "127.0.0.1:4000")])
            .expect_err("missing database url should fail");

        assert_eq!(err, ConfigError::MissingVar("DATABASE_URL"));
    }

    #[test]
    fn validates_bind_addr() {
        let err = AppConfig::from_vars([
            ("API_BIND_ADDR", "bad-address"),
            ("DATABASE_URL", "postgres://localhost/field_service"),
        ])
        .expect_err("invalid bind address should fail");

        assert!(matches!(err, ConfigError::InvalidBindAddress { .. }));
    }
}
