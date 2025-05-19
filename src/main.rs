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
/// 1. Initialise le logging
/// 2. Initialise la base de données
/// 3. Configure les routes et les middlewares
/// 4. Démarre le serveur HTTP
#[tokio::main]
async fn main() {
    // Load environment variables
    dotenvy::dotenv().ok();

    // Initialize configuration
    let config = config::Config::from_env().expect("Failed to load configuration");

    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::filter::EnvFilter::new(
            &config.log_level,
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

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
        .server_address
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
