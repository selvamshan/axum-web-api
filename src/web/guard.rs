use axum::{body::Body, extract::{Request, State}, http::StatusCode, middleware::Next};
use axum::response::Response;
use axum_extra::TypedHeader;
use headers::{authorization::Bearer, Authorization, HeaderMapExt};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait,  QueryFilter};

use crate::database::users;
use crate::database::users::Entity as Users;
use crate::utils::jwt::is_valid_jwt;
use crate::utils::app_errors::AppError;

pub async fn guard(
    State(database): State<DatabaseConnection>,
    TypedHeader(token): TypedHeader<Authorization<Bearer>>,
    mut request:Request<Body>,
    next: Next
) -> Result<Response, AppError>{    
    let token =  token.token().to_owned();     
    let  user = if let Some(user) = Users::find()
        .filter(users::Column::Token.eq(token.clone()))
        .one(&database)
        .await
        .map_err(|_| AppError::new(StatusCode::INTERNAL_SERVER_ERROR,"Internal Server Error"))?
        {
            user
        } else {
            return Err(AppError::new(StatusCode::UNAUTHORIZED, "You are not authorized, please loging again"))
        };
    is_valid_jwt(&token)?;
    request.extensions_mut().insert(user);
   
    Ok(next.run(request).await)
}