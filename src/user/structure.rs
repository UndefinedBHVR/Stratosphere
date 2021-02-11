use crate::{error::StratError, schema::users};
use crate::schema::users::dsl as user_dsl;
use crate::util::{
    db::{can_connect, get_database},
    gen_random,
};
use argon2::{self, Config};
use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::{
    result::Error as dsl_err, ExpressionMethods, PgConnection, QueryDsl, QueryResult, RunQueryDsl,
};

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug, AsChangeset)]
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
    pub fn get_id(&self) -> String {
        self.id.clone()
    }

    //creators
    pub fn get_user(id: String) -> Result<Self, StratError> {
        if can_connect() {
            let db: &PgConnection = &get_database();
            let user: QueryResult<User> = user_dsl::users.find(id).first::<User>(db);
            match user {
                Ok(u) => Ok(u),
                Err(_e) => Err(StratError::UserNotFound),
            }
        } else {
            Err(StratError::DbFailed)
        }
    }

    pub fn get_by_login(email: String, password: String) -> Result<Self, StratError> {
        if can_connect() {
            let db: &PgConnection = &get_database();
            let query: QueryResult<User> = user_dsl::users
                .filter(user_dsl::email.eq(email))
                .first::<User>(db);
            let user = match query {
                Ok(u) => u,
                Err(_e) => return Err(StratError::BadLogin),
            };
            if Self::verify_pass(password, &user.password) {
                return Ok(user);
            }
            Err(StratError::BadLogin)
        } else {
            Err(StratError::DbFailed)
        }
    }

    //setters

    pub fn set_nickname(&mut self, nickname: String) {
        self.nickname = nickname;
    }

    pub fn set_password(&mut self, password: String) {
        self.password = Self::hash_pass(password);
    }

    //savers
    pub fn save_user(&mut self) -> Result<bool, String> {
        let time: DateTime<Utc> = Utc::now();
        self.updated_at = time.naive_utc();
        let db: &PgConnection = &get_database();
        if can_connect() {
            let rslt = match Self::get_user(self.id.clone()) {
                Ok(_u) => diesel::update(users::table)
                    .set(&*self)
                    .filter(user_dsl::id.eq(&self.id))
                    .execute(db),
                Err(_e) => diesel::insert_into(users::table).values(&*self).execute(db),
            };
            match rslt {
                Ok(_) => return Ok(true),
                Err(e) => return Err(format!("{:?}", Self::match_errors(e))),
            }
        }
        Err(format!("{:?}", StratError::DbFailed))
    }

    //util
    fn match_errors(e: dsl_err) -> StratError {
        match e.to_string().as_str() {
            "duplicate key value violates unique constraint \"users_email_key\"" => {
                StratError::EmailInUse
            }
            "duplicate key value violates unique constraint \"users_id_key\"" => {
                StratError::UniqueExists
            }
            "duplicate key value violates unique constraint \"users_nickname_key\"" => {
                StratError::NameExists
            }
            _ => StratError::Unknown,
        }
    }

    fn hash_pass(password: String) -> String {
        let salt = gen_random(30);
        let config = Config::default();
        argon2::hash_encoded(password.as_ref(), salt.as_ref(), &config).unwrap()
    }

    fn verify_pass(password: String, encoded: &str) -> bool {
        return argon2::verify_encoded(encoded, password.as_ref()).unwrap();
    }
}

#[derive(Deserialize)]
pub struct UserCreatable {
    pub nickname: String,
    pub email: String,
    pub password: String,
}
#[derive(Deserialize)]
pub struct UserLoginable {
    pub email: String,
    pub password: String,
}
