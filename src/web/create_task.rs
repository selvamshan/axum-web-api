use axum::http::StatusCode;
use axum::Extension;
use axum::Json;
use serde::Deserialize;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};

use crate::database::tasks;
use crate::database::users::Model;

#[derive(Deserialize)]
pub struct TaskCreate {
    priority: Option<String>,
    title:String,
    description: Option<String>,
}

pub async fn create_task(
    Extension(database): Extension<DatabaseConnection>,
    Extension(user): Extension<Model>,   
    Json(task): Json<TaskCreate>,    
) -> Result<(), StatusCode> {
    
    let new_task = tasks::ActiveModel { 
        priority: Set(task.priority),
        title: Set(task.title), 
        description: Set(task.description), 
        user_id: Set(Some(user.id)),    
       ..Default::default()
    };

    let _result = new_task.save(&database).await.unwrap();

    //dbg!(result);
    Ok(())

}