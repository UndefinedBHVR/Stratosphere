use hyper::{Body, Request, Response, Server};
use routerify::prelude::*;
use routerify::{Middleware, Router, RouterService};
use std::{convert::Infallible, net::SocketAddr};
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
pub mod schema;
pub mod user;
pub mod util;
/*
* The main section of the program contains the basic application as well as uncategorized route handlers (IE: temporary testing handlers)
* This directory should NOT contain route handlers nor middleware handlers for specific modules.
*/
const MODE: &str = "Debug";
#[cfg(debug)]
const MODE: &str = "Debug";
#[cfg(release)]
const MODE: &str = "Release";

async fn index_handler(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    let json = json_response(json!({"status": 200, "response": "Welcome to Stratosphere!"}));
    Ok(json)
}

//Log all requests if we're not in Release mode.
async fn logger(req: Request<Body>) -> Result<Request<Body>, Infallible> {
    #[cfg(not(release))]
    println!(
        "{} {} {}",
        req.remote_addr(),
        req.method(),
        req.uri().path()
    );
    Ok(req)
}

fn create_router() -> Router<Body, Infallible> {
    Router::builder()
        .middleware(Middleware::pre(logger))
        .get("/", index_handler)
        .post("/user/create", create_user)
        .build()
        .unwrap()
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
