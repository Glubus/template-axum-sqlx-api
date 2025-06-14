mod dummy;
mod common;
use sqlx::{Pool, Postgres};
use tracing::{info, warn};
use dummy::create_dummy;

/// Structure pour gérer les fixtures de test
pub async fn run_fixtures(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    info!("Running fixtures...");
    let create_dummy = create_dummy(pool, "John Doe".to_string()).await;
    if let Err(e) = create_dummy {
        warn!("Error running fixtures: {}", e);
    }
    info!("Fixtures run successfully");
    Ok(())
}