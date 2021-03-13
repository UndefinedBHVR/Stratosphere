use super::structure::{Auth, AuthRefresh, AuthToken};
use crate::{
    error::StratError,
    user::structure::{User, UserLoginable},
    util::{json_response, parse_body, parse_cookies},
};
use hyper::{header::HeaderValue, Body, Request, Response};
use routerify::ext::RequestExt;

// Authenticates an account and returns the refresh and token
// Takes a UserLoginable struct as the request body ex: {"email": "johndoe@example.com", "password": "password"}
pub async fn login(mut req: Request<Body>) -> Result<Response<Body>, StratError> {
    let u: UserLoginable = match parse_body::<UserLoginable>(&mut req).await {
        Ok(val) => val,
        Err(e) => return Ok(json_response(json!({"status": 500, "response": e}))),
    };
    let user = match User::get_by_login(&u.email, &u.password) {
        Ok(u) => u,
        Err(e) => return Err(e),
    };
    let mut auth = Auth::new(user.get_id().to_owned());
    match auth.save_auth() {
        None => {
            let mut response =
                json_response(json!({"status": 200, "response": "Authorization created"}));
            // Append Refresh Token as a cookie, limiting it to "/auth/refresh"
            response.headers_mut().append(
                "Set-Cookie",
                HeaderValue::from_str(&format!(
                    "X-AUTH-REFRESH={};Path=/auth/refresh;HttpOnly",
                    auth.get_refresh()
                ))
                .unwrap(),
            );
            // Append the Auth Token as a cookie
            response.headers_mut().append(
                "Set-Cookie",
                HeaderValue::from_str(&format!(
                    "X-AUTH-TOKEN={};Path=/;HttpOnly",
                    auth.get_token()
                ))
                .unwrap(),
            );
            return Ok(response);
        }
        Some(e) => Err(e),
    }
}

// Refreshes an Auth and returns the token
// Takes a AuthRefresh struct as the request body ex: {"refresh": "ABCDEFGHIJKLMNOPQRSTUVWXY"}
pub async fn refresh(req: Request<Body>) -> Result<Response<Body>, StratError> {
    let cookies = parse_cookies(req.headers());
    let refresh = if let Some(token) = cookies.get("X-AUTH-REFRESH") {
        AuthRefresh::new(token.value().to_owned())
    } else {
        return Err(StratError::InvalidRefresh);
    };
    let auth = match refresh.is_valid() {
        true => refresh.to_auth().unwrap(),
        false => return Err(StratError::UnknownRefresh),
    };
    match auth.refresh() {
        Ok(_s) => Ok(json_response(
            json!({"status": 200, "response": "Authorization updated!", "token": auth.get_token()}),
        )),
        Err(e) => Err(e),
    }
}

//Authenticates am account
pub async fn auth_middleware(req: Request<Body>) -> Result<Request<Body>, StratError> {
    let cookies = parse_cookies(req.headers());
    let token = if let Some(token) = cookies.get("X-AUTH-TOKEN") {
        AuthToken::new(token.value().to_owned())
    } else {
        return Err(StratError::InvalidToken);
    };
    if !token.is_valid() {
        return Err(StratError::InvalidToken);
    }
    let user = match User::get_user(token.to_auth().unwrap().get_owner()) {
        Ok(u) => u,
        Err(e) => return Err(e),
    };
    req.set_context(user);
    Ok(req)
}
