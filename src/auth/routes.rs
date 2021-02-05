use super::structure::Auth;
use crate::{
    user::structure::{User, UserLoginable},
    util::{json_response, parse_body},
};
use hyper::{Body, Request, Response};
use std::convert::Infallible;

pub async fn login(mut req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let u: UserLoginable = match parse_body::<UserLoginable>(&mut req).await {
        Ok(val) => val,
        Err(e) => return Ok(json_response(json!({"status": 500, "response": e}))),
    };
    let user = match User::get_by_login(u.email, u.password) {
        Ok(u) => u,
        Err(e) => return Ok(json_response(json!({"status": 500, "response": e}))),
    };
    let mut auth = Auth::new(user.get_id());
    match auth.save_auth() {
        None => Ok(json_response(
            json!({"status": 200, "response": "Authorization created", "token": auth.get_token(), "refresh": auth.get_refresh()}),
        )),
        Some(e) => Ok(json_response(json!({"status": 500, "response": e}))),
    }
}
