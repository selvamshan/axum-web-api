use axum::Extension;
use axum::Json;
use serde::Deserialize;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};

use crate::database::tasks;

#[derive(Deserialize)]
pub struct TaskCreate {
    priority: Option<String>,
    title:String,
    description: Option<String>,
}

pub async fn create_task(
    Extension(database): Extension<DatabaseConnection>,
    Json(task): Json<TaskCreate>
) {
    let new_task = tasks::ActiveModel { 
        priority: Set(task.priority),
        title: Set(task.title), 
        description: Set(task.description),         
       ..Default::default()
    };

    let result = new_task.save(&database).await.unwrap();

    dbg!(result);

}