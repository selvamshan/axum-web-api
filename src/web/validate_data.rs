use axum::Json;
use serde::Deserialize;
use tracing::debug;



#[derive(Deserialize, Debug)]
pub struct RequestUser {
    username: Option<String>,
    password: String,
}

pub async fn validate_data(Json(user):Json<RequestUser>) {
    debug!("{:?}", user);
}