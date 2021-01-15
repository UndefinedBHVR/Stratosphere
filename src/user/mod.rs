use diesel::PgConnection;
use structure::User;

pub mod structure;
pub mod routes;

pub fn create_user(conn: &PgConnection, user: &User){
    
}