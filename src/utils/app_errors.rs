
use axum::http::StatusCode;
use axum::response::{Response,IntoResponse};
use serde::Serialize;
use axum::Json;


pub struct AppError {
    code: StatusCode,
    message:String
}

impl AppError {
    pub fn new(code: StatusCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message:message.into(),
        }
    }
}


#[derive(Serialize)]
struct ResponseMessage {
    message: String
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            self.code,
            Json(
            ResponseMessage{
                    message: self.message
                }),
        )
        .into_response()
    }
}