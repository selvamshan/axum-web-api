use axum::http::StatusCode;
use serde::{Deserialize, Serialize};



#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    exp: usize,
    iat: usize,    
}


pub fn create() -> Result<String, StatusCode> {
    todo!()
}


pub fn verify() -> Result<bool, StatusCode> {
    todo!()
}