use super::structure::{Auth, AuthRefreshable};
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

pub async fn refresh(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let headers = req.headers();
    let token = match headers.get("X-AUTH-TOKEN") {
        Some(h) => match h.to_str() {
            Ok(t) => t.to_string(),
            Err(_e) => {
                return Ok(json_response(
                    json!({"status": 200, "response": "Auth token Header appears to be malformed!"}),
                ))
            }
        },
        None => {
            return Ok(json_response(
                json!({"status": 200, "response": "An Auth token must be supplied."}),
            ))
        }
    };
    let refresh = match headers.get("X-AUTH-REFRESH") {
        Some(h) => match h.to_str() {
            Ok(t) => t.to_string(),
            Err(_e) => {
                return Ok(json_response(
                    json!({"status": 200, "response": "Refresh token Header appears to be malformed!"}),
                ))
            }
        },
        None => {
            return Ok(json_response(
                json!({"status": 200, "response": "A Refresh token must be supplied."}),
            ))
        }
    };
    let a: AuthRefreshable = AuthRefreshable::new(token, refresh);
    let auth = match a.is_valid() {
        true => a.to_auth().unwrap(),
        false => {
            return Ok(json_response(
                json!({"status": 200, "response": "The Refresh token supplied appears to be invalid."}),
            ))
        }
    };
    match auth.refresh() {
        Ok(_s) => Ok(json_response(
            json!({"status": 200, "response": "Authorization updated!", "token": auth.get_token(), "refresh": auth.get_refresh()}),
        )),
        Err(e) => Ok(json_response(json!({"status": 500, "response": e}))),
    }
}
