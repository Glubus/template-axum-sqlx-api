//! # Template Axum SQLx API
//! 
//! Ce module est le point d'entrée principal de l'application.
//! Il configure et démarre le serveur HTTP avec Axum.
//! 
//! ## Fonctionnalités
//! - Configuration du serveur
//! - Initialisation de la base de données
//! - Configuration du logging
//! - Configuration CORS
//! - Gestion des erreurs

mod db;
mod models;
mod routes;
mod handlers;
mod config;

use std::net::SocketAddr;
use dotenvy::dotenv;
use tower_http::cors::{CorsLayer, Any};
use tower_http::trace::TraceLayer;
use crate::db::DatabaseManager;
use crate::routes::create_router;
use crate::config::Config;

/// Point d'entrée principal de l'application.
/// 
/// Cette fonction :
/// 1. Initialise le logging
/// 2. Charge les variables d'environnement
/// 3. Configure le serveur
/// 4. Initialise la base de données
/// 5. Configure les routes et les middlewares
/// 6. Démarre le serveur HTTP
#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_target(false)
        .init();

    // Load environment variables
    if dotenvy::dotenv().is_err() {
        tracing::info!(".env file not found or failed to load, using default configurations or direct env vars.");
    }

    // Load configuration
    let config = match Config::from_env() {
        Ok(cfg) => cfg,
        Err(_e) => {
            tracing::warn!("Failed to load HOST/PORT from env, using default: 127.0.0.1:3001");
            Config { server_address: "127.0.0.1:3001".to_string() } 
        }
    };

    // Initialize database
    let mut db = DatabaseManager::new();
    if let Err(e) = db.connect().await {
        eprintln!("Failed to connect to database: {}", e);
        return;
    }

    // Create router
    let app = create_router(db);

    // Configure CORS and Tracing
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = app
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    // Run server
    let addr_str = config.server_address.clone();
    let addr: SocketAddr = match addr_str.parse() {
        Ok(s_addr) => s_addr,
        Err(e) => {
            eprintln!("Invalid server address format '{}': {}. Falling back to 127.0.0.1:3001", addr_str, e);
            "127.0.0.1:3001".parse().expect("Fallback address should be valid")
        }
    };

    tracing::info!("🚀 Server running on http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
