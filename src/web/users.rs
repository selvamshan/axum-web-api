use axum::{http::StatusCode, Extension, Json};

use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter, Set};
use serde::{Deserialize, Serialize};

use crate::database::users::{Model, Entity as Users};
use crate::database::users;


#[derive(Deserialize)]
pub struct RequestUser {
    username: String,
    password: String
}


#[derive(Serialize)]
pub struct ResponseUser {
    username: String,
    id: i32,
    token: String
}

pub async fn create_users(
    Extension(database): Extension<DatabaseConnection>,
    Json(request_user):Json<RequestUser>,   
) -> Result<Json<ResponseUser>, StatusCode> {
    let new_user = users::ActiveModel {  
        username: Set(request_user.username),
        password: Set(hash_password(request_user.password)?),    
        token: Set(Some("xe45343xrds".to_owned())),
        ..Default::default()
    }.save(&database)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ResponseUser{
        username: new_user.username.unwrap(),
        id: new_user.id.unwrap(),
        token: new_user.token.unwrap().unwrap(),
    }))
 
}


pub async fn login(
    Extension(database): Extension<DatabaseConnection>,
    Json(request_user):Json<RequestUser>,  
) -> Result<Json<ResponseUser>, StatusCode> {
    let db_user = Users::find()
        .filter(users::Column::Username.eq(&request_user.username))
        .one(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(db_user) = db_user {
        if !verify_passord(request_user.password, &db_user.password)? {
            return Err(StatusCode::UNAUTHORIZED);
        }
        let new_token = "1233456abc".to_owned();
        let mut user =  db_user.into_active_model();
        user.token = Set(Some(new_token));
        let saved_user = user.save(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(Json(ResponseUser{
            username: saved_user.username.unwrap(),
            id: saved_user.id.unwrap(),
            token: saved_user.token.unwrap().unwrap(),
        }))
     
    } else {
        return  Err(StatusCode::NOT_FOUND);
    }   
 
}


pub async fn logout(
    Extension(database): Extension<DatabaseConnection>,    
    Extension(user): Extension<Model>
) -> Result<(), StatusCode> {
    let mut user = user.into_active_model();
    user.token = Set(None);
    user.save(&database)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}


fn hash_password(password:String) -> Result<String, StatusCode> {
    bcrypt::hash(password, 14).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

fn verify_passord(password:String, hash: &str) -> Result<bool, StatusCode> {
    bcrypt::verify(password, hash).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}