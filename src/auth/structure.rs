use std::ops::Add;

use crate::schema::auths::dsl as auth_dsl;
use crate::util::db::{can_connect, get_database};
use crate::{schema::auths, util::gen_random};
use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use diesel::{
    result::Error as dsl_err, ExpressionMethods, PgConnection, QueryDsl, QueryResult, RunQueryDsl,
};
use std::fmt::Display;

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

    pub fn get_by_token(id: String) -> Result<Self, AuthError> {
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
                Err(_e) => Err(AuthError::UnknownToken),
            }
        } else {
            Err(AuthError::DbFailed)
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
    pub fn save_auth(&mut self) -> Option<AuthError> {
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
                Ok(_) => return None,
                Err(e) => return Some(Self::match_errors(e)),
            }
        }
        Some(AuthError::DbFailed)
    }

    pub fn refresh(id: String) -> Result<Self, AuthError> {
        if can_connect() {
            let db: &PgConnection = &get_database();
            let auth: QueryResult<Self> = auth_dsl::auths
                .filter(auths::refresh.eq(id))
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
                Err(_e) => return Err(AuthError::UnknownRefresh),
            };
            u.save_auth().unwrap();
            Ok(u)
        } else {
            Err(AuthError::DbFailed)
        }
    }

    //Utils
    pub fn has_expired(&self) -> Option<AuthError> {
        if self.expiry < chrono::Local::now().naive_local() {
            return Some(AuthError::TokenExpired);
        } else if self.created.add(Duration::days(50)) < chrono::Local::now().naive_local() {
            return Some(AuthError::AuthExpired);
        }
        None
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
#[derive(Serialize, Debug)]
pub enum AuthError {
    DbFailed,
    UnknownToken,
    TokenExpired,
    AuthExpired,
    UnknownRefresh,
    UnknownError,
    TokenExists,
}
impl Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::UnknownToken => write!(f, "The submitted Token could not be found!"),
            Self::TokenExpired => write!(f, "The submitted token is expired!"),
            Self::DbFailed => write!(f, "The Database appears to be offline!"),
            Self::AuthExpired => write!(f, "This Authorization has expired!"),
            Self::UnknownRefresh => write!(f, "The submitted refresh token is expired!"),
            Self::UnknownError => write!(f, "An unknown error has occured!"),
            Self::TokenExists => write!(f, "An issue has occured, please try again."),
        }
    }
}
