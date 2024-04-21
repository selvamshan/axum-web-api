use axum::{extract::{Path, Query, State}, http::StatusCode, Extension, Json};
use sea_orm::{prelude::DateTimeWithTimeZone, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

use crate::database::tasks::{self, Entity as Tasks};
use crate::database::users::Model;

#[derive(Serialize)]
pub struct ResponseTask {
    id : i32,
    title: String,
    priority: Option<String>,
    description: Option<String>,
    deleted_at: Option<DateTimeWithTimeZone>,
    user_id: Option<i32>
}

pub async fn get_one_task(
    Path(task_id):Path<i32>,
    State(database): State<DatabaseConnection>,
    Extension(user): Extension<Model>, 
) -> Result<Json<ResponseTask>, StatusCode> {
    let task = Tasks::find_by_id(task_id)
        .filter(tasks::Column::DeletedAt.is_null())
        .filter(tasks::Column::UserId.eq(user.id))
        .one(&database)
        .await
        .unwrap();
    
    if let Some(task) = task {
        Ok(Json(ResponseTask{
            id: task.id,
            title: task.title,
            priority: task.priority,
            description: task.description,
            deleted_at: task.deleted_at,
            user_id: task.user_id
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
    State(database): State<DatabaseConnection>,
    Extension(user): Extension<Model>,
    Query(query_parms): Query<GetTasksQueryParams>
) -> Result<Json<Vec<ResponseTask>>, StatusCode> {
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
    .filter(tasks::Column::DeletedAt.is_null())
    .filter(tasks::Column::UserId.eq(user.id))
    .all(&database)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .into_iter().map(|task| ResponseTask{
        id: task.id,
        title: task.title,
        priority: task.priority,
        description: task.description,
        deleted_at: task.deleted_at,
        user_id:task.user_id
    })
     .collect();
    Ok(Json(tasks))
}