use axum::{extract::{Path, Query, State}, http::StatusCode};
use sea_orm::{ IntoActiveModel, Set};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::Deserialize;

use crate::database::tasks::{self, Entity as Task};

#[derive(Deserialize)]
pub struct QueryParams{
    soft: bool
}

pub async fn delete_task(
    Path(task_id): Path<i32>,
    State(database): State<DatabaseConnection>,
    Query(query_param): Query<QueryParams>
) -> Result<(), StatusCode> {
        // let task = if let Some(task) = Task::find_by_id(task_id)
        //     .one(&database)
        //     .await
        //     .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        // {
        //     task.into_active_model()
        // } else {
        //     return Err(StatusCode::NOT_FOUND)
        // };
        // Task::delete(task)
        // .exec(&database)
        // .await
        // .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        // Task::delete_by_id(task_id)
        // .exec(&database)
        // .await
        // .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        
        if query_param.soft {
            let mut task = if let Some(task) = Task::find_by_id(task_id)
                .one(&database)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            {
                task.into_active_model()
            } else {
                return Err(StatusCode::NOT_FOUND)
            };
            let now = chrono::Utc::now();
            task.deleted_at = Set(Some(now.into()));
            Task::update(task)
            .exec(&database)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        } else {
            Task::delete_many()
            .filter(tasks::Column::Id.eq(task_id))
            .exec(&database)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        }

    Ok(())
}