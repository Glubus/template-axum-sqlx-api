//! # Common Models Module
//!
//! Ce module contient les structures de base communes à tous les modèles.
//! Il fournit des traits et des structures de base pour la sérialisation et la désérialisation.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseModel {
    /// Identifiant unique du modèle
    pub id: i32,
    /// Date et heure de création
    pub created_at: DateTime<Utc>,
    /// Date et heure de dernière modification
    pub updated_at: DateTime<Utc>,
}


