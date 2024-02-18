use axum::Json;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MirrorJson {
    message: String,
}

#[derive(Serialize)]
pub struct MirrorJsonResponse {
    message : String,
    method : String
}

pub async fn mirror_body_json(Json(body):Json<MirrorJson>) -> Json<MirrorJsonResponse> {
    //dbg!(body);
    Json(MirrorJsonResponse{
        message:body.message,
        method:"post".to_string(),
    })
}