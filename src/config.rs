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

/// Structure de configuration de l'application.
/// 
/// Cette structure contient toutes les configurations nécessaires
/// pour le fonctionnement de l'application.
#[derive(Debug, Clone)]
pub struct Config {
    /// Adresse du serveur au format "host:port"
    pub server_address: String,
}

impl Config {
    /// Crée une nouvelle instance de Config à partir des variables d'environnement.
    /// 
    /// # Arguments
    /// 
    /// * `env` - Les variables d'environnement à utiliser (par défaut: les variables système)
    /// 
    /// # Returns
    /// 
    /// * `Result<Config, env::VarError>` - La configuration ou une erreur
    /// 
    /// # Exemple
    /// 
    /// ```rust
    /// let config = Config::from_env()?;
    /// ```
    pub fn from_env() -> Result<Self, env::VarError> {
        let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = env::var("PORT").unwrap_or_else(|_| "3001".to_string());
        
        Ok(Config {
            server_address: format!("{}:{}", host, port),
        })
    }
} 