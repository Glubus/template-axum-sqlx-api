use serde::{Deserialize, Serialize};
use std::env;
use tracing::{info, warn, debug};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

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
    /// Initialise le système de logging
    fn init_logging(level: &str, format: &str) {
        let env_filter = EnvFilter::try_from_default_env()
            .or_else(|_| EnvFilter::try_new(level))
            .unwrap_or_else(|_| EnvFilter::new("info"));

        tracing_subscriber::registry()
            .with(env_filter)
            .with(tracing_subscriber::fmt::layer())
            .init();

        info!("Logging initialized with level: {}", level);
    }

    /// Charge la configuration depuis config.toml avec possibilité d'override par les variables d'environnement
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        // Charger la configuration de base pour initialiser le logging
        let config_content = include_str!("../assets/config.toml");
        let base_config = toml::from_str::<Config>(config_content)?;
        
        // Initialiser le logging avec la configuration de base
        Self::init_logging(&base_config.logging.level, &base_config.logging.format);

        info!("Loading configuration from config.toml");
        
        // Charger le fichier TOML
        let mut config = base_config;
        debug!("Base configuration loaded: {:?}", config);

        // Override avec les variables d'environnement si elles existent
        if let Ok(host) = env::var("HOST") {
            info!("Overriding host with environment variable: {}", host);
            config.server.host = host;
        }
        if let Ok(port) = env::var("PORT") {
            info!("Overriding port with environment variable: {}", port);
            config.server.port = port.parse().unwrap_or(config.server.port);
        }
        if let Ok(db_url) = env::var("DATABASE_URL") {
            info!("Overriding database URL with environment variable");
            config.database.url = db_url;
        }
        if let Ok(log_level) = env::var("RUST_LOG") {
            info!("Overriding log level with environment variable: {}", log_level);
            config.logging.level = log_level;
        }

        info!("Configuration loaded successfully. Server will bind to: {}", config.server_address());
        Ok(config)
    }

    /// Retourne l'adresse complète du serveur
    pub fn server_address(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }

    /// Configuration par défaut
    pub fn default() -> Self {
        warn!("Using default configuration as no config.toml was found");
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
