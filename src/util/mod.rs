use hyper::{header, Body, Response, StatusCode};
use serde_json::Value;
use rand::{Rng, distributions::Alphanumeric, thread_rng};
use std::iter;

pub mod db;
pub fn json_response(json: Value) -> Response<Body> {
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(json.to_string()))
        .expect("Unable to create response.")
}

pub fn gen_random(length: usize) -> String{
    let mut rng = thread_rng();
    let result: String = iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(length)
        .collect();
    result
}