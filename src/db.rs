//! # Database Module
//! 
//! Ce module gère la connexion et les opérations avec la base de données PostgreSQL.
//! Il utilise SQLx pour les requêtes asynchrones et la gestion du pool de connexions.
//! 
//! ## Utilisation
//! ```rust
//! use crate::db::DatabaseManager;
//! 
//! let mut db = DatabaseManager::new();
//! db.connect().await?;
//! let pool = db.get_pool();
//! ```

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;

/// Gestionnaire de base de données.
/// 
/// Cette structure gère la connexion à la base de données PostgreSQL
/// et fournit un pool de connexions pour les requêtes.
pub struct DatabaseManager {
    /// Pool de connexions à la base de données
    pool: Option<PgPool>,
}

impl DatabaseManager {
    /// Crée une nouvelle instance de DatabaseManager.
    /// 
    /// # Returns
    /// 
    /// * `DatabaseManager` - Une nouvelle instance non connectée
    pub fn new() -> Self {
        Self { pool: None }
    }

    /// Établit la connexion à la base de données.
    /// 
    /// Cette méthode :
    /// 1. Lit l'URL de la base de données depuis les variables d'environnement
    /// 2. Crée un pool de connexions
    /// 3. Stocke le pool dans l'instance
    /// 
    /// # Returns
    /// 
    /// * `Result<(), sqlx::Error>` - Succès ou erreur de connexion
    /// 
    /// # Panics
    /// 
    /// Cette méthode panique si la variable d'environnement `DATABASE_URL` n'est pas définie.
    pub async fn connect(&mut self) -> Result<(), sqlx::Error> {
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;

        self.pool = Some(pool);
        Ok(())
    }

    /// Récupère le pool de connexions.
    /// 
    /// # Returns
    /// 
    /// * `&PgPool` - Référence au pool de connexions
    /// 
    /// # Panics
    /// 
    /// Cette méthode panique si la base de données n'a pas été initialisée.
    pub fn get_pool(&self) -> &PgPool {
        self.pool.as_ref().expect("Database not initialized")
    }
} 