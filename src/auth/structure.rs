use std::ops::Add;

use crate::{error::StratError, schema::auths::dsl as auth_dsl};
use crate::util::db::{can_connect, get_database};
use crate::{schema::auths, util::gen_random};
use chrono::{Duration, NaiveDateTime};
use diesel::{
    result::Error as dsl_err, ExpressionMethods, PgConnection, QueryDsl, QueryResult, RunQueryDsl,
};


#[derive(Queryable, Insertable, Serialize, Deserialize, Debug, AsChangeset)]
pub struct Auth {
    token: String,
    refresh: String,
    owner: String,
    expiry: NaiveDateTime,
    created: NaiveDateTime,
}

impl Auth {
    //creators
    pub fn new(user: String) -> Self {
        Self {
            token: gen_random(25),
            refresh: gen_random(33),
            owner: user,
            expiry: chrono::Local::now().naive_local() + Duration::weeks(1),
            created: chrono::Local::now().naive_local(),
        }
    }

    pub fn get_by_token(id: String) -> Result<Self, StratError> {
        if can_connect() {
            let db: &PgConnection = &get_database();
            let auth: QueryResult<Self> = auth_dsl::auths
                .filter(auths::token.eq(id))
                .first::<Self>(db);
            match auth {
                Ok(u) => match u.has_expired() {
                    Some(e) => Err(e),
                    None => Ok(u),
                },
                Err(_e) => Err(StratError::UnknownToken),
            }
        } else {
            Err(StratError::DbFailed)
        }
    }

    pub fn get_by_refresh(id: String) -> Result<Self, StratError> {
        if can_connect() {
            let db: &PgConnection = &get_database();
            let auth: QueryResult<Self> = auth_dsl::auths
                .filter(auths::refresh.eq(id))
                .first::<Self>(db);
            match auth {
                Ok(u) => match u.has_expired() {
                    Some(e) => Err(e),
                    None => Ok(u),
                },
                Err(_e) => Err(StratError::UnknownRefresh),
            }
        } else {
            Err(StratError::DbFailed)
        }
    }

    //getters
    pub fn get_token(&self) -> String {
        self.token.clone()
    }

    pub fn get_refresh(&self) -> String {
        self.refresh.clone()
    }

    //savers
    pub fn save_auth(&mut self) -> Option<StratError> {
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
                Ok(_) => return None,
                Err(e) => return Some(Self::match_errors(e)),
            }
        }
        Some(StratError::DbFailed)
    }

    pub fn refresh(&self) -> Result<Self, StratError> {
        if can_connect() {
            let db: &PgConnection = &get_database();
            let auth: QueryResult<Self> = auth_dsl::auths
                .filter(auths::refresh.eq(self.refresh.clone()))
                .first::<Self>(db);
            let mut u = match auth {
                Ok(mut u) => {
                    u.token = gen_random(25);
                    u.expiry = chrono::Local::now().naive_local() + Duration::weeks(1);
                    match u.has_expired() {
                        Some(e) => return Err(e),
                        None => u,
                    }
                }
                Err(_e) => return Err(StratError::UnknownRefresh),
            };
            u.save_auth().unwrap();
            Ok(u)
        } else {
            Err(StratError::DbFailed)
        }
    }

    //Utils
    pub fn has_expired(&self) -> Option<StratError> {
        if self.expiry < chrono::Local::now().naive_local() {
            return Some(StratError::TokenExpired);
        } else if self.created.add(Duration::days(50)) < chrono::Local::now().naive_local() {
            return Some(StratError::AuthExpired);
        }
        None
    }

    fn match_errors(e: dsl_err) -> StratError {
        match e.to_string().as_str() {
            "duplicate key value violates unique constraint \"auths_token_key\"" => {
                StratError::UniqueExists
            }
            _ => StratError::Unknown,
        }
    }
}

pub struct AuthRefresh {
    refresh: String,
}

impl AuthRefresh {
    pub fn new(refresh: String) -> Self {
        Self { refresh }
    }

    pub fn is_valid(&self) -> bool {
        self.to_auth().is_ok()
    }

    pub fn to_auth(&self) -> Result<Auth, StratError> {
        Auth::get_by_refresh(self.refresh.clone())
    }
}

pub struct AuthToken {
    token: String,
}
impl AuthToken {
    pub fn new(token: String) -> Self {
        Self { token }
    }

    pub fn is_valid(&self) -> bool {
        self.to_auth().is_ok()
    }

    pub fn to_auth(&self) -> Result<Auth, StratError> {
        Auth::get_by_token(self.token.clone())
    }
}
