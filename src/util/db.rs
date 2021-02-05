use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};
use dotenv::dotenv;
use lazy_static::lazy_static;
use std::env;

/*
* The Database submodule contains all utilities related to accessing the database.
* This file should NOT contain table specific functions.
*/

type DbPool = Pool<ConnectionManager<PgConnection>>;
type DbPoolConn = PooledConnection<ConnectionManager<PgConnection>>;
pub struct Database {
    pub db_connection: DbPool,
}

lazy_static! {
    pub static ref DATABASE: Database = {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        Database {
            db_connection: DbPool::builder()
                .max_size(8)
                .build(ConnectionManager::new(&database_url))
                .expect("failed to create db connection_pool"),
        }
    };
}
pub fn get_database() -> DbPoolConn {
    DATABASE.db_connection.get().ok().unwrap()
}
pub fn can_connect() -> bool {
    DATABASE.db_connection.get().is_ok()
}
