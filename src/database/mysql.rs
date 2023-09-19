use std::env;

use diesel::mysql::MysqlConnection;
use diesel::r2d2::{self, ConnectionManager, Pool};
use dotenvy::dotenv;

pub fn get_pool() -> Pool<ConnectionManager<MysqlConnection>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}