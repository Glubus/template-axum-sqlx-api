// Re-export all model modules here
// Example:
// pub mod user;
// pub mod product;

pub mod common;

// You can also define common model traits or types here
pub mod common {
    use serde::{Deserialize, Serialize};
    use chrono::{DateTime, Utc};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct BaseModel {
        pub id: i32,
        pub created_at: DateTime<Utc>,
        pub updated_at: DateTime<Utc>,
    }
} 