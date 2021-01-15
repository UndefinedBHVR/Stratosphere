use hyper::{Body, Request, Response, Server};
use routerify::prelude::*;
use routerify::{Middleware, Router, RouterService};
use std::{convert::Infallible, net::SocketAddr};
use util::json_response;
//Macro Use
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_json;

pub mod util;
pub mod user;
//A simple reponse to verify everything works as intended.
async fn index_handler(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    let json = json_response(json!({"response": "Welcome to Stratosphere!"}));
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
        .build()
        .unwrap()
}
//Returns current mode
fn determine_mode() -> &'static str {
    #[cfg(debug)]
    return "Debug";
    #[cfg(release)]
    return "Release";
    return "Development";
}
#[tokio::main]
async fn main() {
    let router = create_router();
    let service = RouterService::new(router).unwrap();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    let server = Server::bind(&addr).serve(service);
    println!(
        "Stratosphere initialized in {} mode and running on: {}",
        determine_mode(),
        addr
    );
    if let Err(err) = server.await {
        eprintln!("Server error: {}", err);
    }
}
