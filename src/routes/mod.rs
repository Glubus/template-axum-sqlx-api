use axum::Router;
use crate::db::DatabaseManager;

// Re-export all route modules here
// Example:
// pub mod user;
// pub mod product;

pub fn create_router(db: DatabaseManager) -> Router {
    Router::new()
        // Add your route modules here
        // Example:
        // .merge(user::router())
        // .merge(product::router())
        .with_state(db)
} 