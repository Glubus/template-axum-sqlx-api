
use crate::fixtures::common::FixtureManager;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use fake::{Dummy as FakeDummy, Fake, Faker};

// this is just a dummy model you can delete it 
// this is just a dummy model you can delete it 
// this is just a dummy model you can delete it 
// this is just a dummy model you can delete it 
// this is just a dummy model you can delete it 
// this is just a dummy model you can delete it 
// this is just a dummy model you can delete it 
#[derive(Debug, Serialize, Deserialize, FakeDummy)]
pub struct Dummy {
    pub name: String
}

pub fn create_dummy_from_fake(number: u32) -> Vec<Dummy> {
    let mut dummies = Vec::new();
    for _ in 0..number {
        let dummy : Dummy = Faker.fake();
        dummies.push(dummy);
    }
    dummies
}

pub async fn create_dummy(pool: &Pool<Postgres>, name: String) -> Result<(), sqlx::Error> {
    let dummies = create_dummy_from_fake(100);
    let fixture_manager = FixtureManager::new(pool.clone());
    fixture_manager.submit_fixtures(dummies, "dummy").await?;
    Ok(())
}

