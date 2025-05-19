//! # Routes Module
//!
//! Ce module gère la configuration des routes de l'API.
//! Il permet d'organiser les routes par domaine fonctionnel et de les combiner
//! dans un routeur Axum unique.
//!
//! ## Utilisation
//!
//! Pour ajouter de nouvelles routes :
//! 1. Créez un nouveau module dans le dossier `routes/`
//! 2. Implémentez une fonction `router()` qui retourne un `Router`
//! 3. Ajoutez le module dans ce fichier
//! 4. Utilisez `merge()` pour combiner les routes

use crate::db::DatabaseManager;
use axum::Router;

// Re-export all route modules here
// Example:
// pub mod user;
// pub mod product;

/// Crée le routeur principal de l'application.
///
/// Cette fonction :
/// 1. Crée un nouveau routeur
/// 2. Combine toutes les routes des différents modules
/// 3. Ajoute l'état de la base de données
///
/// # Arguments
///
/// * `db` - Le gestionnaire de base de données
///
/// # Returns
///
/// * `Router` - Le routeur configuré
pub fn create_router(db: DatabaseManager) -> Router {
    Router::new()
        // Add your route modules here
        // Example:
        // .merge(user::router())
        // .merge(product::router())
        .with_state(db)
}
