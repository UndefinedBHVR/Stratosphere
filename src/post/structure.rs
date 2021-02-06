use std::fmt::Display;

use crate::util::gen_random;
use crate::{
    schema::{posts, posts::dsl as post_dsl},
    util::db::{can_connect, get_database},
};
use chrono::NaiveDateTime;
use diesel::{
    result::Error as dsl_err, ExpressionMethods, PgConnection, QueryDsl, QueryResult, RunQueryDsl,
};
#[derive(Queryable, Insertable, Serialize, Deserialize, Debug, AsChangeset)]
pub struct Post {
    id: String,
    owner: String,
    public: bool,
    content: String,
    created: NaiveDateTime,
    edited: NaiveDateTime,
    //todo: Implement media
    //media: Vec<Media>
}

impl Post {
    pub fn new(content: String, owner: String, public: bool) -> Self {
        //todo: Sanitize Inputs
        //content = html_sanitize(content);
        Self {
            id: gen_random(27),
            owner,
            public,
            content,
            created: chrono::Local::now().naive_local(),
            edited: chrono::Local::now().naive_local(),
        }
    }

    pub fn edit(&mut self, content: String) {
        //content = html_sanitize(content);
        self.content = content;
        self.edited = chrono::Local::now().naive_local();
    }

    pub fn get_by_id(id: String) -> Result<Self, PostError> {
        if can_connect() {
            let db: &PgConnection = &get_database();
            let auth: QueryResult<Self> =
                post_dsl::posts.filter(posts::id.eq(id)).first::<Self>(db);
            match auth {
                Ok(u) => Ok(u),
                Err(_e) => Err(PostError::UnknownPost),
            }
        } else {
            Err(PostError::DbFailed)
        }
    }

    pub fn save_post(&self) -> Option<PostError> {
        let db: &PgConnection = &get_database();
        if can_connect() {
            let rslt = match Self::get_by_id(self.id.clone()) {
                Ok(_u) => diesel::update(posts::table)
                    .set(&*self)
                    .filter(post_dsl::id.eq(&self.id))
                    .execute(db),
                Err(_e) => diesel::insert_into(posts::table).values(&*self).execute(db),
            };
            match rslt {
                Ok(_) => return None,
                Err(e) => return Some(Self::match_errors(e)),
            }
        }
        Some(PostError::DbFailed)
    }

    fn match_errors(_e: dsl_err) -> PostError {
        PostError::UnknownError
    }
}

pub enum PostError {
    DbFailed,
    UnknownPost,
    UnknownError,
}

impl Display for PostError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::UnknownPost => write!(f, "The requested Post could not be found!"),
            Self::DbFailed => write!(f, "The Database appears to be offline!"),
            Self::UnknownError => write!(f, "An unknown error has occured!"),
        }
    }
}
