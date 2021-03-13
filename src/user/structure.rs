use crate::schema::users::dsl as user_dsl;
use crate::util::{
    db::{can_connect, get_database},
    gen_random,
};
use crate::{error::StratError, schema::users};
use argon2::{self, Config};
use chrono::{DateTime, NaiveDateTime, Utc};
use diesel::{
    result::Error as dsl_err, ExpressionMethods, PgConnection, QueryDsl, QueryResult, RunQueryDsl,
};

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug, AsChangeset, Clone)]
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
    // Creates a new User Struct but doesn't save it.
    pub fn new(nickname: String, email: String, password: String) -> Self {
        let time: DateTime<Utc> = Utc::now();
        Self {
            id: gen_random(23),
            nickname,
            email,
            password: Self::hash_pass(&password),
            rank: 0,
            is_priv: false,
            updated_at: time.naive_utc(),
            created_at: time.naive_utc(),
        }
    }

    // Gets a reference to the user's ID
    pub fn get_id(&self) -> &str {
        &self.id
    }

    // Gets an instance of the user using their ID
    pub fn get_user(id: &str) -> Result<Self, StratError> {
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

    // Find a user using the username and password combination
    pub fn get_by_login(email: &str, password: &str) -> Result<Self, StratError> {
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

    pub fn set_nickname(&mut self, nickname: &str) {
        self.nickname = nickname.to_owned();
    }

    pub fn set_password(&mut self, password: &str) {
        self.password = Self::hash_pass(password);
    }

    // Saves our user back into the database.
    pub fn save_user(&mut self) -> Result<bool, StratError> {
        let time: DateTime<Utc> = Utc::now();
        self.updated_at = time.naive_utc();
        let db: &PgConnection = &get_database();
        if can_connect() {
            let rslt = match Self::get_user(&self.id) {
                Ok(_u) => diesel::update(users::table)
                    .set(&*self)
                    .filter(user_dsl::id.eq(&self.id))
                    .execute(db),
                Err(_e) => diesel::insert_into(users::table).values(&*self).execute(db),
            };
            match rslt {
                Ok(_) => return Ok(true),
                Err(e) => return Err(Self::match_errors(e)),
            }
        }
        Err(StratError::DbFailed)
    }

    // util
    // Turns Diesel Error strings into our custom error types.
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

    // Function for hasing passwords (duh)
    fn hash_pass(password: &str) -> String {
        let salt = gen_random(30);
        let config = Config::default();
        argon2::hash_encoded(password.as_ref(), salt.as_ref(), &config).unwrap()
    }

    // Checks if the submitted password matches our hash
    fn verify_pass(password: &str, encoded: &str) -> bool {
        return argon2::verify_encoded(encoded, password.as_ref()).unwrap();
    }

    pub fn get_rank(&self) -> i32 {
        self.rank
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
