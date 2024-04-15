use axum::{body::Body, extract::Request, http::StatusCode, middleware::Next};
use axum::response::Response;
use headers::{authorization::Bearer, Authorization, HeaderMapExt};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter};

use crate::database::users;
use crate::database::users::Entity as Users;


pub async fn guard(
    mut request:Request<Body>,
    next: Next
) -> Result<Response, StatusCode>{
    let token = request.headers().typed_get::<Authorization<Bearer>>()
        .ok_or(StatusCode::BAD_REQUEST)?
        .token()
        .to_owned();
    let database = request.extensions().get::<DatabaseConnection>()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    let  user = if let Some(user) = Users::find()
        .filter(users::Column::Token.eq(token))
        .one(database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        {
            user
        } else {
            return Err(StatusCode::UNAUTHORIZED)
        };
    request.extensions_mut().insert(user);
   
    Ok(next.run(request).await)
}