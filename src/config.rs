use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub server_address: String,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = env::var("PORT").unwrap_or_else(|_| "3001".to_string());
        
        Ok(Config {
            server_address: format!("{}:{}", host, port),
        })
    }
} 