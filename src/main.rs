use auth::routes::{auth_middleware, login, refresh};
use error::StratError;
use hyper::{Body, Request, Response, Server};
use post::routes::{create_post, delete_post, edit_post};
use routerify::prelude::*;
use routerify::{Middleware, Router, RouterService};
use std::net::SocketAddr;
use user::routes::create_user;
use util::json_response;
//Macro Use
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_json;
//modules
pub mod auth;
pub mod error;
pub mod post;
pub mod schema;
pub mod user;
pub mod util;

// This is just flavour for the server logs.
const MODE: &str = "Debug";
#[cfg(release)]
const MODE: &str = "Release";

// This probably isn't needed
async fn index_handler(_: Request<Body>) -> Result<Response<Body>, StratError> {
    let json = json_response(json!({"status": 200, "response": "Welcome to Stratosphere!"}));
    Ok(json)
}

//Log all requests if we're not in Release mode.
async fn logger(req: Request<Body>) -> Result<Request<Body>, StratError> {
    #[cfg(not(release))]
    println!(
        "{} {} {}",
        req.remote_addr(),
        req.method(),
        req.uri().path()
    );
    Ok(req)
}

// A function that crates our Router.
fn create_router() -> Router<Body, StratError> {
    Router::builder()
        .middleware(Middleware::pre(logger))
        .post("/user/create", create_user)
        .post("/user/login", login)
        .post("/auth/refresh", refresh)
        .get("/", index_handler)
        .scope(
            // set a prefix for all the authorization routes
            "/v1/",
            Router::builder()
                .middleware(Middleware::pre(auth_middleware))
                .get("/", index_handler)
                .post("/post/create", create_post)
                .patch("/post/edit", edit_post)
                .delete("/post/delete", delete_post)
                .err_handler(error_handler)
                .build()
                .unwrap(),
        )
        .err_handler(error_handler)
        .build()
        .unwrap()
}

// Take an Error, if its our type, we return this status and response
pub fn err_to_resp(e: Box<dyn std::error::Error + Sync + Send + 'static>) -> Response<Body> {
    if let Some(e) = e.downcast_ref::<StratError>() {
        json_response(json!({"status": 500, "response": format!("{}", e)}))
    } else {
        json_response(json!({"status": 500, "response": "An internal server error has occured!"}))
    }
}

/// Basically process error types, send stuff to err_to_resp if it is capable of being our error.
async fn error_handler(e: routerify::Error) -> Response<Body> {
    match e {
        routerify::Error::HandlePreMiddlewareRequest(e) => err_to_resp(e),
        routerify::Error::HandleRequest(e, _) => err_to_resp(e),
        routerify::Error::HandlePostMiddlewareWithoutInfoRequest(e) => err_to_resp(e),
        routerify::Error::HandlePostMiddlewareWithInfoRequest(e) => err_to_resp(e),
        _ => json_response(
            json!({"status": 500, "response": "An internal server error has occured!"}),
        ),
    }
}

// Creating our Router and Running it.
#[tokio::main]
async fn main() {
    let router = create_router();
    let service = RouterService::new(router).unwrap();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    let server = Server::bind(&addr).serve(service);
    println!(
        "Stratosphere initialized in {} mode and running on: {}",
        MODE, addr
    );
    if let Err(err) = server.await {
        eprintln!("Server error: {}", err);
    }
}
