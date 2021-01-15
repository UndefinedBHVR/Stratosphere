use std::time::SystemTime;
use argon2::{self, Config};
use crate::util::gen_random;
#[derive(Debug, Queryable, Insertable, Deserialize)]
pub struct User{
    id: String,
    nickname: String,
    email: String,
    password: String,
    rank: i32,
    is_priv: bool,
    updated_at: Option<SystemTime>,
    created_at: SystemTime
}

impl User{
    pub fn new(nickname: String, email: String, password: String) -> Self{
        Self{
            id: gen_random(23),
            nickname,
            email,
            password: Self::hash_pass(password),
            rank: 0,
            is_priv: false,
            updated_at: None,
            created_at: SystemTime::now()
        }
    }
    //getters

    //setters
    pub fn set_nickname(&mut self, nickname: String){
        self.nickname = nickname;
    }
    pub fn set_password(&mut self, password: String){
        self.password = Self::hash_pass(password);
    }

    //utilities
    fn hash_pass(password: String) -> String{
        let salt = gen_random(30);
        let config = Config::default();
        argon2::hash_encoded(password.as_ref(), salt.as_ref(), &config).unwrap()
    }
    
}