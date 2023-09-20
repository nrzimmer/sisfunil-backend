use std::env;
use dotenvy::dotenv;
use super::types::*;

pub fn get_pool() -> DBPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    diesel::r2d2::Pool::builder()
        .build(DBManager::new(database_url))
        .expect("Failed to create pool")
}