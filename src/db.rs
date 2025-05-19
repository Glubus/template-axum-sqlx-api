use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;

pub struct DatabaseManager {
    pool: Option<PgPool>,
}

impl DatabaseManager {
    pub fn new() -> Self {
        Self { pool: None }
    }

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

    pub fn get_pool(&self) -> &PgPool {
        self.pool.as_ref().expect("Database not initialized")
    }
} 