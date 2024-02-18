
use axum::routing::{get, post};
use axum::Router;

mod routes_hello;
pub mod routes_static;
mod mirror_body_strings;
mod mirror_body_json;
mod path_variable;

use routes_hello::hello_handler;
use mirror_body_strings::mirro_body_strings;
use mirror_body_json::mirror_body_json;
use path_variable::path_variable;



pub fn routes() -> Router {
    Router::new()
    .route("/hello", get(hello_handler))
    .route("/mirror_body_string", post(mirro_body_strings))
    .route("/mirror_body_json", post(mirror_body_json))
    .route("/path_variable/:id", get(path_variable))
}
