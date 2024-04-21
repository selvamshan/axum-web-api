
use axum::extract::FromRef;
use axum::routing::{delete, get, patch, post, put};
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
mod update_task;
mod partial_update_task;
mod delete_task;
mod users;
mod guard;

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
use update_task::atomic_update;
use partial_update_task::partial_update;
use delete_task::delete_task;
use users::{create_users, login, logout};
use guard::guard;

#[derive(Clone)]
pub struct SharedData {
    message: String,    
}

#[derive(Clone, FromRef)]
pub struct AppState {
    pub database: DatabaseConnection,
}


pub fn routes(database:DatabaseConnection) -> Router {
     
     let shared_data = SharedData {
        message: "Hello from shared data".to_string(),
     };
     let app_state = AppState{ database };

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
    .with_state(shared_data)
    .route("/return_json", get(return_json))
    .route("/validate_data", post(validate_data))
    .route("/custom_json_extractor", post(custom_json_extactor)) 
    .route("/task", post(create_task))
    .route("/task", get(get_all_tasks))
    .route("/task/:task_id", get(get_one_task))
    .route("/task/:task_id", put(atomic_update))
    .route("/task/:task_id", patch(partial_update))
    .route("/task/:task_id", delete(delete_task))
    .route("/users/logout", post(logout))
    .route_layer(middleware::from_fn_with_state(app_state.clone(), guard))
    .route("/users/login", post(login))
    .route("/users", post(create_users))    
    .with_state(app_state)
    
}
