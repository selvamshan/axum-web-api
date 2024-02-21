use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Data {
    message: String,
    count: i32,
    username: String
}

pub async fn return_json() -> Json<Data>{
    let data = Data{
        message: "I am in data".to_string(),
        count: 2,
        username: "selva".to_string()
    };

    Json(data)
}