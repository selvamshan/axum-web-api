use std::time::Duration;
use dotenvy::dotenv;
use dotenvy_macro::dotenv;
use axum::http::StatusCode;
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use super::app_errors::AppError;



#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    exp: usize,
    iat: usize,    
}


pub fn create_jwt() -> Result<String, StatusCode> {
    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let expires_in = Duration::from_secs(300);
    let exp = (now + expires_in).timestamp() as usize;    
    let claim = Claims{exp: exp, iat: iat };
    let secret = dotenv!("JWT_SECRET").as_bytes();
    let key = EncodingKey::from_secret(secret);
    let token = encode(&Header::default(), &claim, &key)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);
    token
}


pub fn is_valid_jwt(token:&str) -> Result<bool, AppError> {
    let secret = dotenv!("JWT_SECRET").as_bytes();
    let key = DecodingKey::from_secret(secret);
    let _ = decode::<Claims>(token, &key, &Validation::new(jsonwebtoken::Algorithm::HS256))
        .map_err(|error| {
            match error.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => AppError::new(StatusCode::UNAUTHORIZED, "Your sesssion has expired"),
                _ => AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong")
            }
        })?;
   Ok(true)
}