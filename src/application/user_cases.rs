use crate::application::services::{TaskService, UserService};
use crate::domain::entities::task::{Task, Recurrence};
use crate::domain::entities::user::{User, Role};
use crate::domain::errors::AppError;

pub struct TaskUseCases;
pub struct UserUseCases;

impl TaskUseCases {
    pub async fn create_new_task(
        user_id: i32,
        title: String,
        description: Option<String>,
    ) -> Result<Task, AppError> {
        TaskService::create_task(user_id.to_string(), Option::from(title), description).await
    }

    pub async fn complete_existing_task(task: &mut Task) -> Result<(), AppError> {
        TaskService::complete_task(task).await
    }

    pub async fn archive_existing_task(task: &mut Task) -> Result<(), AppError> {
        TaskService::archive_task(task).await
    }

    pub async fn delete_existing_task(task: &mut Task) -> Result<(), AppError> {
        TaskService::delete_task(task).await
    }

    pub async fn add_comment_to_task(
        task: &mut Task,
        user_id: i32,
        comment: String,
    ) -> Result<(), AppError> {
        TaskService::add_comment_to_task(task, user_id, comment).await
    }

    pub async fn add_subtask_to_task(
        task: &mut Task,
        subtask: Task
    ) -> Result<(), AppError> {
        TaskService::add_subtask(task, subtask).await
    }

    pub async fn set_task_due_date(task: &mut Task, due_date: Option<i64>) -> Result<(), AppError> {
        TaskService::set_task_due_date(task, due_date).await
    }

    pub async fn set_task_priority(task: &mut Task, priority: Option<i32>) -> Result<(), AppError> {
        TaskService::set_task_priority(task, priority).await
    }

    pub async fn set_task_recurrence(
        task: &mut Task,
        recurrence: Option<Recurrence>,
        recurrence_end: Option<i64>
    ) -> Result<(), AppError> {
        TaskService::set_task_recurrence(task, recurrence, recurrence_end).await
    }
}

impl UserUseCases {
    pub async fn create_new_user(
        username: String,
        password_hash: String,
        role: Role,
    ) -> Result<User, AppError> {
        UserService::create_user(username, password_hash, role).await
    }

    pub async fn update_user_info(
        user: &mut User,
        name: Option<String>,
        surname: Option<String>,
        email: Option<String>,
    ) -> Result<(), AppError> {
        UserService::update_user_details(user, name, surname, email).await
    }

    pub async fn follow_another_user(
        follower: &mut User,
        followee_id: i32,
    ) -> Result<(), AppError> {
        UserService::follow_user(follower, followee_id).await
    }

    pub async fn unfollow_another_user(
        follower: &mut User,
        followee_id: i32,
    ) -> Result<(), AppError> {
        UserService::unfollow_user(follower, followee_id).await
    }

    pub async fn archive_existing_user(user: &mut User) -> Result<(), AppError> {
        UserService::archive_user(user).await
    }

    pub async fn delete_existing_user(user: &mut User) -> Result<(), AppError> {
        UserService::delete_user(user).await
    }

    pub async fn change_user_role(user: &mut User, role: Role) -> Result<(), AppError> {
        UserService::set_user_role(user, role).await
    }
}