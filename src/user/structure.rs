use crate::schema::users;
use crate::schema::users::dsl as user_dsl;
use crate::util::{
    db::{can_connect, get_database},
    gen_random,
};
use argon2::{self, Config};
use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::{ExpressionMethods, PgConnection, QueryDsl, QueryResult, RunQueryDsl};

/*
* The Structure module contains a Database accessing Struct and its implementation as well as an enum containing possible Errors.
* All functions should be contained in the Implementation and all fucntions MUST relate to the table or struct.
*/

#[derive(Queryable, Insertable, Serialize, Deserialize, Clone, Debug, AsChangeset)]
pub struct User {
    id: String,
    nickname: String,
    email: String,
    password: String,
    rank: i32,
    is_priv: bool,
    updated_at: NaiveDateTime,
    created_at: NaiveDateTime,
}

pub enum UserError {
    NotFound,
    DbFailed,
    SavingError,
}

impl User {
    pub fn new(nickname: String, email: String, password: String) -> Self {
        let time: DateTime<Utc> = Utc::now();
        Self {
            id: gen_random(23),
            nickname,
            email,
            password: Self::hash_pass(password),
            rank: 0,
            is_priv: false,
            updated_at: time.naive_utc(),
            created_at: time.naive_utc(),
        }
    }
    //getters
    pub fn get_user(id: String) -> Result<Self, UserError> {
        if can_connect() {
            let db: &PgConnection = &get_database();
            let user: QueryResult<User> = user_dsl::users.find(id).first::<User>(db);
            match user {
                Ok(u) => return Ok(u),
                Err(_e) => return Err(UserError::NotFound),
            }
        } else {
            Err(UserError::DbFailed)
        }
    }

    //setters
    pub fn set_nickname(&mut self, nickname: String) {
        self.nickname = nickname;
    }
    pub fn set_password(&mut self, password: String) {
        self.password = Self::hash_pass(password);
    }
    pub fn save_user(&mut self) -> Result<bool, UserError> {
        let time: DateTime<Utc> = Utc::now();
        self.updated_at = time.naive_utc();
        let db: &PgConnection = &get_database();
        let usr = self.clone();
        if can_connect() {
            let rslt = match Self::get_user(self.id.clone()) {
                Ok(_u) => diesel::update(users::table)
                    .set(usr)
                    .filter(user_dsl::id.eq(&self.id))
                    .execute(db),
                Err(e) => diesel::insert_into(users::table).values(usr).execute(db),
            };
            if rslt.unwrap() > 0 {
                return Ok(true);
            }
            return Err(UserError::SavingError);
        }
        return Err(UserError::DbFailed);
    }

    //utilities
    fn hash_pass(password: String) -> String {
        let salt = gen_random(30);
        let config = Config::default();
        argon2::hash_encoded(password.as_ref(), salt.as_ref(), &config).unwrap()
    }
}
