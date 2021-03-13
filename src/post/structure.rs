use crate::{error::StratError, util::gen_random};
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
    // Creates a new post.
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

    // Edits a post
    pub fn edit(&mut self, content: String) {
        //content = html_sanitize(content);
        self.content = content;
        self.edited = chrono::Local::now().naive_local();
    }

    // Finds a post using its ID.
    pub fn get_by_id(id: &str) -> Result<Self, StratError> {
        if can_connect() {
            let db: &PgConnection = &get_database();
            let auth: QueryResult<Self> =
                post_dsl::posts.filter(posts::id.eq(id)).first::<Self>(db);
            match auth {
                Ok(u) => Ok(u),
                Err(_e) => Err(StratError::UnknownPost),
            }
        } else {
            Err(StratError::DbFailed)
        }
    }

    // Saves the post instance back into the datbase
    pub fn save_post(&self) -> Option<StratError> {
        let db: &PgConnection = &get_database();
        if can_connect() {
            let rslt = match Self::get_by_id(&self.id) {
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
        Some(StratError::DbFailed)
    }

    // Converts Diesel Errors into regular Errors.
    fn match_errors(_e: dsl_err) -> StratError {
        StratError::Unknown
    }

    // Deletes this post, consuming self.
    pub fn delete_post(self) -> Option<StratError> {
        let db: &PgConnection = &get_database();
        if can_connect() {
            let rslt = match Self::get_by_id(&self.id) {
                Ok(_u) => {
                    diesel::delete(posts::table.filter(post_dsl::id.eq(&self.id))).execute(db)
                }
                Err(_e) => diesel::insert_into(posts::table).values(self).execute(db),
            };
            match rslt {
                Ok(_) => return None,
                Err(e) => return Some(Self::match_errors(e)),
            }
        }
        Some(StratError::DbFailed)
    }

    pub fn get_owner(&self) -> &str {
        &self.owner
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }
}
