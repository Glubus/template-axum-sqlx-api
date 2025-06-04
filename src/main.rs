//! # Template Axum SQLx API
//!
//! Ce module est le point d'entrée principal de l'application.
//! Il configure et démarre le serveur HTTP avec Axum.
//!
//! ## Fonctionnalités
//! - Configuration depuis config.toml
//! - Initialisation de la base de données
//! - Configuration du logging
//! - Configuration CORS
//! - Gestion des erreurs

mod config;
mod db;
mod handlers;
mod models;
mod routes;

use axum::Router;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Point d'entrée principal de l'application.
///
/// Cette fonction :
/// 1. Charge la configuration depuis config.toml
/// 2. Initialise le logging
/// 3. Initialise la base de données
/// 4. Configure les routes et les middlewares
/// 5. Démarre le serveur HTTP
#[tokio::main]
async fn main() {
    // Load configuration from config.toml
    let config = config::Config::load().expect("Failed to load configuration");

    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::filter::EnvFilter::new(
            &config.logging.level,
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting server with configuration loaded from config.toml");
    tracing::debug!("Server will bind to: {}", config.server_address());

    // Initialize database
    let mut db = db::DatabaseManager::new();
    db.connect(&config)
        .await
        .expect("Failed to connect to database");

    // Build our application with a route
    let app = Router::new()
        .merge(routes::create_router(db))
        .layer(CorsLayer::permissive());

    // Run it
    let addr: SocketAddr = config
        .server_address()
        .parse()
        .expect("Invalid server address");
    tracing::info!("listening on {}", addr);
    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app.into_make_service(),
    )
    .await
    .unwrap();
}
