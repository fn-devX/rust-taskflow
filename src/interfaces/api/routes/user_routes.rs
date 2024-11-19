use axum::{routing::{post, put}, Router, Json, http::StatusCode};
use crate::{application::services::UserService, domain::entities::user::{User, Role}};
use crate::domain::errors::AppError;
use serde::{Deserialize, Serialize};

pub fn user_routes() -> Router {
    Router::new()
        .route("/users", post(create_user))
        .route("/users/:id/update", put(update_user_details))
        .route("/users/:id/follow", post(follow_user))
        .route("/users/:id/unfollow", post(unfollow_user))
        .route("/users/:id/archive", put(archive_user))
        .route("/users/:id/delete", put(delete_user))
        .route("/users/:id/role", put(set_user_role))
}

#[derive(Deserialize)]
struct CreateUserRequest {
    username: String,
    password: String,
    role: Role,
}

async fn create_user(Json(payload): Json<CreateUserRequest>) -> Result<Json<User>, StatusCode> {
    UserService::create_user(payload.username, payload.password, payload.role)
        .map(Json)
        .map_err(|_| StatusCode::BAD_REQUEST)
}

#[derive(Deserialize)]
struct UpdateUserRequest {
    name: Option<String>,
    surname: Option<String>,
    email: Option<String>,
}

async fn update_user_details(Json(payload): Json<UpdateUserRequest>) -> StatusCode {
    let mut user = User::mock();
    UserService::update_user_details(&mut user, payload.name, payload.surname, payload.email)
        .map(|_| StatusCode::OK)
        .unwrap_or(StatusCode::BAD_REQUEST)
}

#[derive(Deserialize)]
struct FollowRequest {
    followee_id: i32,
}

async fn follow_user(Json(payload): Json<FollowRequest>) -> StatusCode {
    let mut user = User::mock();
    UserService::follow_user(&mut user, payload.followee_id)
        .map(|_| StatusCode::OK)
        .unwrap_or(StatusCode::BAD_REQUEST)
}

async fn unfollow_user(Json(payload): Json<FollowRequest>) -> StatusCode {
    let mut user = User::mock();
    UserService::unfollow_user(&mut user, payload.followee_id)
        .map(|_| StatusCode::OK)
        .unwrap_or(StatusCode::BAD_REQUEST)
}

async fn archive_user() -> StatusCode {
    let mut user = User::mock();
    UserService::archive_user(&mut user)
        .map(|_| StatusCode::OK)
        .unwrap_or(StatusCode::BAD_REQUEST)
}

async fn delete_user() -> StatusCode {
    let mut user = User::mock();
    UserService::delete_user(&mut user)
        .map(|_| StatusCode::OK)
        .unwrap_or(StatusCode::BAD_REQUEST)
}

#[derive(Deserialize)]
struct RoleRequest {
    role: Role,
}

async fn set_user_role(Json(payload): Json<RoleRequest>) -> StatusCode {
    let mut user = User::mock();
    UserService::set_user_role(&mut user, payload.role);
    StatusCode::OK
}
