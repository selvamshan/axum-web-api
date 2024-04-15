use axum::http::StatusCode;
use axum::{extract::Path, Extension, Json};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set};
use sea_orm::{prelude::DateTimeWithTimeZone, DatabaseConnection};
use serde::Deserialize;

use crate::database::tasks::Entity as Task;
use crate::database::tasks;


#[derive(Deserialize)]
pub struct RequestTask {   
    pub id: Option<i32>,
    pub priority: Option<String>,
    pub title: String,
    pub completed_at: Option<DateTimeWithTimeZone>,  
    pub description: Option<String>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
    pub user_id: Option<i32>,
    pub is_default: Option<bool>,
}


pub async fn atomic_update(
    Path(task_id): Path<i32>,
    Extension(database):Extension<DatabaseConnection>,
    Json(request_task): Json<RequestTask>
) -> Result<(), StatusCode> {
    let update_task = tasks::ActiveModel {
        id: Set(task_id),
        priority: Set(request_task.priority),
        title: Set(request_task.title),
        completed_at:Set(request_task.completed_at),
        description:Set(request_task.description),
        deleted_at:Set(request_task.deleted_at),
        user_id:Set(request_task.user_id),
        is_default:Set(request_task.is_default),
    };

    Task::update(update_task)
        .filter(tasks::Column::Id.eq(task_id))
        .exec(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}