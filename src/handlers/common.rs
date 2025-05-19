//! # Common Handlers Module
//!
//! Ce module contient les structures et traits communs pour les gestionnaires de routes.
//! Il fournit des utilitaires pour la gestion des réponses HTTP.

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

/// Structure de réponse API standardisée.
///
/// Cette structure permet de formater les réponses API de manière cohérente
/// avec un format JSON standard incluant :
/// - Un code de statut HTTP
/// - Des données optionnelles
/// - Un message optionnel
///
/// # Exemple
///
/// ```rust
/// use crate::handlers::common::ApiResponse;
/// use axum::http::StatusCode;
///
/// let response = ApiResponse {
///     status: StatusCode::OK,
///     data: Some("Hello, World!"),
///     message: Some("Success"),
/// };
/// ```
pub struct ApiResponse<T> {
    /// Code de statut HTTP
    pub status: StatusCode,
    /// Données de la réponse (optionnelles)
    pub data: Option<T>,
    /// Message de la réponse (optionnel)
    pub message: Option<String>,
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: serde::Serialize,
{
    /// Convertit la réponse en format HTTP.
    ///
    /// Cette implémentation :
    /// 1. Crée un objet JSON avec les données et le message
    /// 2. Combine le statut HTTP avec le corps JSON
    ///
    /// # Returns
    ///
    /// * `Response` - La réponse HTTP formatée
    fn into_response(self) -> Response {
        let body = json!({
            "data": self.data,
            "message": self.message,
        });

        (self.status, Json(body)).into_response()
    }
}
