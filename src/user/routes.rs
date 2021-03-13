use super::structure::{User, UserCreatable};
use crate::{
    error::StratError,
    util::{json_response, parse_body},
};
use hyper::{Body, Request, Response};

// Create an instance of a User, and saves it to the database.
// Takes the UserCreatable struct as the body ex: {"nickname": "testaccount", "email": "johndoe@example.com", "password": "password"}
pub async fn create_user(mut req: Request<Body>) -> Result<Response<Body>, StratError> {
    let u: UserCreatable = match parse_body::<UserCreatable>(&mut req).await {
        Ok(val) => val,
        Err(e) => return Ok(json_response(json!({"status": 500, "response": e}))),
    };
    let mut user = User::new(u.nickname, u.email, u.password);
    match user.save_user() {
        Ok(_) => Ok(json_response(
            json!({"status": 200, "response": "Successfully created user!"}),
        )),
        Err(e) => Err(e),
    }
}
