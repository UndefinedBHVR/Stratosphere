use diesel::{
    r2d2::{Pool, ConnectionManager},
    pg::PgConnection
};
use dotenv::dotenv;
use lazy_static::lazy_static;
use std::env;


type DbPool = Pool<ConnectionManager<PgConnection>>;
pub struct Database {
    pub db_connection: DbPool,
}

lazy_static! {
    pub static ref DATABASE: Database = {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
        Database {
            db_connection: DbPool::builder()
                .max_size(8)
                .build(ConnectionManager::new(&database_url))
                .expect("failed to create db connection_pool")
        }
    };
}