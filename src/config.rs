//! # Configuration Module
//!
//! Ce module gère la configuration de l'application.
//! Il charge la configuration depuis config.toml et permet des overrides via les variables d'environnement.
//!
//! ## Utilisation
//! ```rust
//! use crate::config::Config;
//!
//! let config = Config::load()?;
//! println!("Server address: {}", config.server_address());
//! ```

use serde::{Deserialize, Serialize};
use std::env;
use std::path::Path;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CorsConfig {
    pub allowed_origins: Vec<String>,
    pub allowed_methods: Vec<String>,
    pub allowed_headers: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub logging: LoggingConfig,
    pub cors: CorsConfig,
}

impl Config {
    /// Charge la configuration depuis config.toml avec possibilité d'override par les variables d'environnement
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = "config.toml";
        
        // Charger le fichier TOML
        let mut config = if Path::new(config_path).exists() {
            let content = std::fs::read_to_string(config_path)?;
            toml::from_str::<Config>(&content)?
        } else {
            // Configuration par défaut si le fichier n'existe pas
            Config::default()
        };

        // Override avec les variables d'environnement si elles existent
        if let Ok(host) = env::var("HOST") {
            config.server.host = host;
        }
        if let Ok(port) = env::var("PORT") {
            config.server.port = port.parse().unwrap_or(config.server.port);
        }
        if let Ok(db_url) = env::var("DATABASE_URL") {
            config.database.url = db_url;
        }
        if let Ok(log_level) = env::var("RUST_LOG") {
            config.logging.level = log_level;
        }

        Ok(config)
    }

    /// Retourne l'adresse complète du serveur
    pub fn server_address(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }

    /// Configuration par défaut
    pub fn default() -> Self {
        Config {
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 3000,
            },
            database: DatabaseConfig {
                url: "postgres://postgres:postgres@localhost:5432/template_db".to_string(),
                max_connections: 10,
                min_connections: 1,
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                format: "json".to_string(),
            },
            cors: CorsConfig {
                allowed_origins: vec![
                    "http://localhost:3000".to_string(),
                    "http://127.0.0.1:3000".to_string(),
                ],
                allowed_methods: vec![
                    "GET".to_string(),
                    "POST".to_string(),
                    "PUT".to_string(),
                    "DELETE".to_string(),
                    "OPTIONS".to_string(),
                ],
                allowed_headers: vec![
                    "content-type".to_string(),
                    "authorization".to_string(),
                ],
            },
        }
    }
}
