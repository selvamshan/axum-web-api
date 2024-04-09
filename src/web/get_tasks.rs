use axum::{extract::{Path, Query}, http::StatusCode, Extension, Json};
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

use crate::database::tasks::{self, Entity as Tasks};

#[derive(Serialize)]
pub struct ResponseTask {
    id : i32,
    title: String,
    priority: Option<String>,
    description: Option<String>
}

pub async fn get_one_task(
    Path(task_id):Path<i32>,
    Extension(database):Extension<DatabaseConnection>
) -> Result<Json<ResponseTask>, StatusCode> {
    let task = Tasks::find_by_id(task_id)
    .one(&database)
    .await
    .unwrap();

    if let Some(task) = task {
        Ok(Json(ResponseTask{
            id: task.id,
            title: task.title,
            priority: task.priority,
            description: task.description,
        }))
    } else {
        Err(
        StatusCode::NOT_FOUND,    
        )
    }
    
}

#[derive(Deserialize)]
pub struct GetTasksQueryParams {
    priority: Option<String>
}

pub async fn get_all_tasks(
    Extension(database):Extension<DatabaseConnection>,
    Query(query_parms): Query<GetTasksQueryParams>
) -> Result<Json<Vec<ResponseTask>>, StatusCode>{
    let mut priority_filter = Condition::all();
    if let Some(priority) = query_parms.priority {
        
        priority_filter = if priority.is_empty() {
            priority_filter.add(tasks::Column::Priority.is_null())
        } else {
            priority_filter.add(tasks::Column::Priority.eq(priority))
        };
    }
    let tasks = Tasks::find()
    .filter(priority_filter)
    .all(&database)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .into_iter().map(|task| ResponseTask{
        id: task.id,
        title: task.title,
        priority: task.priority,
        description: task.description})
     .collect();
    Ok(Json(tasks))
}