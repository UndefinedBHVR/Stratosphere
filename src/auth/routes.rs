use super::structure::{Auth, AuthRefresh};
use crate::{error::StratError, user::structure::{User, UserLoginable}, util::{json_response, parse_body}};
use hyper::{Body, Request, Response};
use std::{io};

//Authenticates an account and returns the refresh and token
//Takes a UserLoginable struct as the request body ex: {"email": "johndoe@example.com", "password": "password"}
pub async fn login(mut req: Request<Body>) -> Result<Response<Body>, std::io::Error> {
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

//Refreshes an Auth and returns the token
//Takes a AuthRefresh struct as the request body ex: {"refresh": "ABCDEFGHIJKLMNOPQRSTUVWXY"}
pub async fn refresh(req: Request<Body>) -> Result<Response<Body>, std::io::Error> {
    let headers = req.headers();
    let refresh = match headers.get("X-AUTH-REFRESH") {
        Some(h) => match h.to_str() {
            Ok(t) => t.to_string(),
            Err(_e) => {
                return Ok(json_response(
                    json!({"status": 500, "response": "Refresh token Header appears to be malformed!"}),
                ))
            }
        },
        None => {
            return Ok(json_response(
                json!({"status": 500, "response": "A Refresh token must be supplied."}),
            ))
        }
    };
    let a: AuthRefresh = AuthRefresh::new(refresh);
    let auth = match a.is_valid() {
        true => a.to_auth().unwrap(),
        false => {
            return Ok(json_response(
                json!({"status": 500, "response": "The Refresh token supplied appears to be invalid."}),
            ))
        }
    };
    match auth.refresh() {
        Ok(_s) => Ok(json_response(
            json!({"status": 200, "response": "Authorization updated!", "token": auth.get_token()}),
        )),
        Err(e) => Ok(json_response(json!({"status": 500, "response": e}))),
    }
}

//Authenticates am account
pub async fn auth_middleware(_req: Request<Body>) -> Result<Request<Body>, std::io::Error> {
    return Err(std::io::Error::new(io::ErrorKind::Other, StratError::AuthFailed));
}
