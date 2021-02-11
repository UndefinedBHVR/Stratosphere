use auth::routes::{auth_middleware, login};
use error::StratError;
use hyper::{Body, Request, Response, Server};
use routerify::prelude::*;
use routerify::{Middleware, Router, RouterService};
use std::{net::SocketAddr};
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

const MODE: &str = "Debug";
#[cfg(debug)]
const MODE: &str = "Debug";
#[cfg(release)]
const MODE: &str = "Release";

async fn index_handler(_: Request<Body>) -> Result<Response<Body>, std::io::Error> {
    let json = json_response(json!({"status": 200, "response": "Welcome to Stratosphere!"}));
    Ok(json)
}

//Log all requests if we're not in Release mode.
async fn logger(req: Request<Body>) -> Result<Request<Body>, std::io::Error> {
    #[cfg(not(release))]
    println!(
        "{} {} {}",
        req.remote_addr(),
        req.method(),
        req.uri().path()
    );
    Ok(req)
}

fn create_router() -> Router<Body, std::io::Error> {
    Router::builder()
        .middleware(Middleware::pre(logger))
        .post("/user/create", create_user)
        .post("/user/login", login)
        .get("/", index_handler)
        .scope(
            // set a prefix for all the authorization routes
            "/v1",
            Router::builder()
                .middleware(Middleware::pre(auth_middleware))
                .get("/", index_handler)
                .build()
                .unwrap(),
        )
        .err_handler(error_handler)
        .build()
        .unwrap()
}
async fn error_parse(e: impl std::error::Error) -> String{
    if let Some(e) = e.source(){
        return format!("{}", e)
    }
    "Failed to parse Error!".to_string()
}
async fn error_handler(e: routerify::Error) -> Response<Body> {
    let e = match &e{
        routerify::Error::HandlePreMiddlewareRequest(_a) => {
            error_parse(e).await
        }
        _ => format!("{}",StratError::Unknown)
    };
    json_response(json!({"status": 500, "response": e}))
}
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
