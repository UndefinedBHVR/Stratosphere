use super::structure::{Auth, AuthRefresh};
use crate::{error::StratError, user::structure::{User, UserLoginable}, util::{json_response, parse_body}};
use hyper::{Body, Request, Response};


//Authenticates an account and returns the refresh and token
//Takes a UserLoginable struct as the request body ex: {"email": "johndoe@example.com", "password": "password"}
pub async fn login(mut req: Request<Body>) -> Result<Response<Body>, StratError> {
    let u: UserLoginable = match parse_body::<UserLoginable>(&mut req).await {
        Ok(val) => val,
        Err(e) => return Ok(json_response(json!({"status": 500, "response": e}))),
    };
    let user = match User::get_by_login(u.email, u.password) {
        Ok(u) => u,
        Err(e) => return Err(e),
    };
    let mut auth = Auth::new(user.get_id());
    match auth.save_auth() {
        None => Ok(json_response(
            json!({"status": 200, "response": "Authorization created", "token": auth.get_token(), "refresh": auth.get_refresh()}),
        )),
        Some(e) => Err(e),
    }
}

//Refreshes an Auth and returns the token
//Takes a AuthRefresh struct as the request body ex: {"refresh": "ABCDEFGHIJKLMNOPQRSTUVWXY"}
pub async fn refresh(req: Request<Body>) -> Result<Response<Body>, StratError> {
    let headers = req.headers();
    let refresh = match headers.get("X-AUTH-REFRESH") {
        Some(h) => match h.to_str() {
            Ok(t) => t.to_string(),
            Err(_e) => {
                return Err(StratError::InvalidRefresh)
            }
        },
        None => {
            return Err(StratError::InvalidRefresh)
        }
    };
    let a: AuthRefresh = AuthRefresh::new(refresh);
    let auth = match a.is_valid() {
        true => a.to_auth().unwrap(),
        false => {
            return Err(StratError::UnknownRefresh)
        }
    };
    match auth.refresh() {
        Ok(_s) => Ok(json_response(
            json!({"status": 200, "response": "Authorization updated!", "token": auth.get_token()}),
        )),
        Err(e) => Err(e),
    }
}

//Authenticates am account
pub async fn auth_middleware(_req: Request<Body>) -> Result<Request<Body>, StratError> {
    return Err(StratError::AuthFailed);
}
