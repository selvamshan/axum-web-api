use axum::routing::get;
use axum::Router;


pub fn routes() -> Router {
    Router::new()
        .route("/hello", get(hello_handler))

}

pub async fn hello_handler() -> String {
    "Hello World!".to_string()
}