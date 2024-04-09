
use axum::routing::{get, post};
use axum::{middleware, Extension, Router};
use sea_orm::DatabaseConnection;

mod routes_hello;
pub mod routes_static;
mod mirror_body_strings;
mod mirror_body_json;
mod path_variable;
mod query_params;
mod mirror_user_agent;
mod mirror_custom_header;
mod middleware_message;
mod middleware_custom_header;
mod set_middleware_custom_header;
mod return_json;
mod validate_data;
mod custom_json_extactor;
mod create_task;
mod get_tasks;

use routes_hello::hello_handler;
use mirror_body_strings::mirro_body_strings;
use mirror_body_json::mirror_body_json;
use path_variable::path_variable;
use query_params::query_params;
use mirror_user_agent::mirror_user_agent;
use mirror_custom_header::mirror_custom_header;
use middleware_message::middleware_message;
use middleware_custom_header::middleware_custom_header;
use set_middleware_custom_header::set_middleware_custom_header;
use  return_json::return_json;
use validate_data::validate_data;
use custom_json_extactor::custom_json_extactor;
use create_task::create_task;
use get_tasks::{get_one_task,get_all_tasks};

#[derive(Clone)]
pub struct SharedData {
    message: String,    
}


pub fn routes(database:DatabaseConnection) -> Router {
     
     let shared_data = SharedData {
        message: "Hello from shared data".to_string(),
     };

    Router::new()
    .route("/middleware_custom_header", get(middleware_custom_header))
    .route_layer(middleware::from_fn(set_middleware_custom_header))
    .route("/hello", get(hello_handler))
    .route("/mirror_body_string", post(mirro_body_strings))
    .route("/mirror_body_json", post(mirror_body_json))
    .route("/path_variable/:id", get(path_variable))
    .route("/query_params", get(query_params))
    .route("/mirror_user_agent", get(mirror_user_agent))
    .route("/mirror_custom_header", get(mirror_custom_header))
    .route("/middleware_message", get(middleware_message))
    .layer(Extension(shared_data))
    .route("/return_json", get(return_json))
    .route("/validate_data", post(validate_data))
    .route("/custom_json_extractor", post(custom_json_extactor))
    .route("/task", post(create_task))
    .route("/task", get(get_all_tasks))
    .route("/task/:task_id", get(get_one_task))
    .layer(Extension(database))
    
}
