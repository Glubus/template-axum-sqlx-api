// Re-export all handler modules here
// Example:
// pub mod user;
// pub mod product;

pub mod common;

// You can also define common handler utilities here
pub mod common {
    use axum::{
        http::StatusCode,
        response::{IntoResponse, Response},
        Json,
    };
    use serde_json::json;

    pub struct ApiResponse<T> {
        pub status: StatusCode,
        pub data: Option<T>,
        pub message: Option<String>,
    }

    impl<T> IntoResponse for ApiResponse<T>
    where
        T: serde::Serialize,
    {
        fn into_response(self) -> Response {
            let body = json!({
                "data": self.data,
                "message": self.message,
            });

            (self.status, Json(body)).into_response()
        }
    }
} 