use super::structure::{User, UserCreatable};
use crate::util::{json_response, parse_body};
use hyper::{Body, Request, Response};
use std::convert::Infallible;

/*
* The Routes module contains the various routing for each directory.
* Each Route MUST return a JSON result containing the status followed by the response.
* IE: `{"status": 200, "result": "Welcome to Stratosphere!"}
*/

pub async fn create_user(mut req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let u: UserCreatable = match parse_body::<UserCreatable>(&mut req).await {
        Ok(val) => val,
        Err(e) => return Ok(json_response(json!({"status": 500, "response": e}))),
    };
    let mut user = User::new(u.nickname, u.email, u.password);
    match user.save_user() {
        Ok(_) => Ok(json_response(
            json!({"status": 200, "response": "Successfully created user!"}),
        )),
        Err(e) => Ok(json_response(json!({"status": 500, "response": e}))),
    }
}
