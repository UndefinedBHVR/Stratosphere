use std::ops::Add;

use crate::{schema::auths, util::gen_random};
use crate::schema::auths::dsl as auth_dsl;
use crate::util::{
    db::{can_connect, get_database},
};
use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use diesel::{
    result::Error as dsl_err, ExpressionMethods, PgConnection, QueryDsl, QueryResult, RunQueryDsl,
};
#[derive(Queryable, Insertable, Serialize, Deserialize, Debug, AsChangeset)]
pub struct Auth{
    token: String,
    refresh: String,
    owner: String,
    expiry: NaiveDateTime,
    created: NaiveDateTime,
}

impl Auth{
    //Creators
    pub fn new(user: String) -> Self{
        Self{
            token: gen_random(25),
            refresh: gen_random(33),
            owner: user,
            expiry: chrono::Local::now().naive_local() + Duration::weeks(1),
            created: chrono::Local::now().naive_local()
        }
    }

    //Getters
    pub fn get_by_token(id: String) -> Result<Self, AuthError> {
        if can_connect() {
            let db: &PgConnection = &get_database();
            let auth: QueryResult<Self> = auth_dsl::auths.filter(auths::token.eq(id)).first::<Self>(db);
            match auth {
                Ok(u) => {
                    match u.has_expired(){
                        Some(e) => { return Err(e) },
                        None => return Ok(u)
                    }
                },
                Err(_e) => return Err(AuthError::UnknownToken),
            }
        } else {
            Err(AuthError::DbFailed)
        }
    }

    //Setters
    pub fn refresh(id: String) -> Result<Self, AuthError> {
        if can_connect() {
            let db: &PgConnection = &get_database();
            let auth: QueryResult<Self> = auth_dsl::auths.filter(auths::refresh.eq(id)).first::<Self>(db);
            let mut u = match auth {
                Ok(mut u) => {
                    u.token = gen_random(25);
                    u.expiry = chrono::Local::now().naive_local() + Duration::weeks(1);
                    match u.has_expired(){
                        Some(e) => return Err(e),
                        None => u
                    }
                },
                Err(_e) => return Err(AuthError::UnknwonRefresh),
            };
            u.save_auth().unwrap();
            return Ok(u);
        } else {
            Err(AuthError::DbFailed)
        }
    }

    //Utils
    pub fn has_expired(&self) -> Option<AuthError>{
        if self.expiry < chrono::Local::now().naive_local(){
            return Some(AuthError::TokenExpired);
        }
        else if self.created.add(Duration::days(50)) < chrono::Local::now().naive_local(){
            return Some(AuthError::AuthExpired)
        }
        return None
    }

    pub fn save_auth(&mut self) -> Result<bool, String>{
        let _time: DateTime<Utc> = Utc::now();
        let db: &PgConnection = &get_database();
        if can_connect() {
            let rslt = match Self::get_by_token(self.token.clone()) {
                Ok(_u) => diesel::update(auths::table)
                    .set(&*self)
                    .filter(auth_dsl::token.eq(&self.token))
                    .execute(db),
                Err(_e) => diesel::insert_into(auths::table).values(&*self).execute(db),
            };
            match rslt {
                Ok(_) => return Ok(true),
                Err(e) => return Err(format!("{:?}", Self::match_errors(e))),
            }
        }
        return Err(format!("{:?}", AuthError::DbFailed));
    }

    fn match_errors(e: dsl_err) -> AuthError {
        match e.to_string().as_str() {
            "duplicate key value violates unique constraint \"auths_token_key\"" => {
                AuthError::TokenExists
            }
            _ => AuthError::UnknownError,
        }
    }
}
#[derive(Debug)]
pub enum AuthError{
    DbFailed,
    UnknownToken,
    TokenExpired,
    AuthExpired,
    UnknwonRefresh,
    UnknownError,
    TokenExists
}