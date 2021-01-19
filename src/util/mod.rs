use hyper::{header, Body, Response, StatusCode};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde_json::Value;
use std::iter;

pub mod db;

/*
* The Utilities module contains various utility functions that would be used accross the applicaiton.
* If multiple functions are groupable (IE: Database, HTML Sanitizing), they should be moved to their own sub-module.
*/

pub fn json_response(json: Value) -> Response<Body> {
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(json.to_string()))
        .expect("Unable to create response.")
}

pub fn gen_random(length: usize) -> String {
    let mut rng = thread_rng();
    let result: String = iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(length)
        .collect();
    result
}
