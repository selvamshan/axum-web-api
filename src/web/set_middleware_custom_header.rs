use axum::{
    body::Body, 
    extract::Request, 
    http::StatusCode, 
    middleware::Next,
    response::Response};

use super::middleware_custom_header::MiddleWareMessage;



pub async fn set_middleware_custom_header(
    mut req: Request<Body>,
    next: Next
) -> Result<Response, StatusCode> {
    let headers = req.headers();
    let message = headers
        .get("message")
        .ok_or_else(|| StatusCode::BAD_REQUEST)?;
    let message = message
        .to_str()
        .map_err(|_| StatusCode::BAD_REQUEST )?
        .to_owned();

    let extentions = req.extensions_mut();

    extentions.insert(MiddleWareMessage{message});

    Ok(next.run(req).await)
}