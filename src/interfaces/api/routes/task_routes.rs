use axum::{routing::{post, put}, Router, Json, http::StatusCode};
use crate::{application::services::TaskService, domain::entities::task::{Task, TaskStatus, Recurrence}};
use crate::domain::errors::AppError;
use serde::{Deserialize, Serialize};

pub fn task_routes() -> Router {
    Router::new()
        .route("/tasks", post(create_task))
        .route("/tasks/:id/complete", put(complete_task))
        .route("/tasks/:id/archive", put(archive_task))
        .route("/tasks/:id/delete", put(delete_task))
        .route("/tasks/:id/comment", post(add_comment_to_task))
        .route("/tasks/:id/subtask", post(add_subtask))
}

#[derive(Deserialize)]
struct CreateTaskRequest {
    title: String,
    description: Option<String>,
    description0: Option<String>,
}

async fn create_task(Json(payload): Json<CreateTaskRequest>) -> Result<Json<Task>, StatusCode> {
    TaskService::create_task(payload.title, payload.description, payload.description0)
        .map(Json)
        .map_err(|_| StatusCode::BAD_REQUEST)
}

async fn complete_task() -> Result<StatusCode, StatusCode> {
    let mut task = Task::mock();
    TaskService::complete_task(&mut task).map(|_| StatusCode::OK).map_err(|_| StatusCode::BAD_REQUEST)
}

async fn archive_task() -> Result<StatusCode, StatusCode> {
    let mut task = Task::mock();
    TaskService::archive_task(&mut task).map(|_| StatusCode::OK).map_err(|_| StatusCode::BAD_REQUEST)
}

async fn delete_task() -> Result<StatusCode, StatusCode> {
    let mut task = Task::mock();
    TaskService::delete_task(&mut task).map(|_| StatusCode::OK).map_err(|_| StatusCode::BAD_REQUEST)
}

#[derive(Deserialize)]
struct CommentRequest {
    user_id: i32,
    comment: String,
}

async fn add_comment_to_task(Json(payload): Json<CommentRequest>) -> Result<StatusCode, StatusCode> {
    let mut task = Task::mock();
    TaskService::add_comment_to_task(&mut task, payload.user_id, payload.comment)
        .map(|_| StatusCode::OK)
        .map_err(|_| StatusCode::BAD_REQUEST)
}

#[derive(Deserialize)]
struct SubtaskRequest {
    subtask: Task,
}

async fn add_subtask(Json(payload): Json<SubtaskRequest>) -> Result<StatusCode, StatusCode> {
    let mut task = Task::mock();
    TaskService::add_subtask(&mut task, payload.subtask)
        .map(|_| StatusCode::OK)
        .map_err(|_| StatusCode::BAD_REQUEST)
}
