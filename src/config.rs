//! # Configuration Module
//!
//! Ce module gère la configuration de l'application.
//! Il charge les variables d'environnement et fournit une structure de configuration typée.
//!
//! ## Utilisation
//! ```rust
//! use crate::config::Config;
//!
//! let config = Config::from_env()?;
//! println!("Server address: {}", config.server_address);
//! ```

use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub server_address: String,
    pub database_url: String,
    pub log_level: String,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
        let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@localhost:5432/template_db".to_string()
        });
        let log_level = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());

        Ok(Config {
            server_address: format!("{}:{}", host, port),
            database_url,
            log_level,
        })
    }
}
