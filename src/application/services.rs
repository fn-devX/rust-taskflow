use crate::domain::entities::{
    task::{Task, TaskStatus, Recurrence},
    user::{User, Role},
};
use crate::domain::errors::AppError;

pub struct TaskService;

impl TaskService {
    pub fn create_task(title: String, description: Option<String>) -> Result<Task, AppError> {
        if title.is_empty() {
            return Err(AppError::ValidationError { field: "".to_string(), message: "Title cannot be empty".to_string() });
        }
        Ok(Task::new(title, description))
    }

    pub fn complete_task(task: &mut Task) -> Result<(), AppError> {
        if task.status == TaskStatus::Completed {
            return Err(AppError::ValidationError { field: "".to_string(), message: "Task is already completed".to_string() });
        }
        task.complete();
        Ok(())
    }

    pub fn archive_task(task: &mut Task) -> Result<(), AppError> {
        if task.archived_at.is_some() {
            return Err(AppError::ValidationError { field: "".to_string(), message: "Task is already archived".to_string() });
        }
        task.archive();
        Ok(())
    }

    pub fn delete_task(task: &mut Task) -> Result<(), AppError> {
        if task.deleted_at.is_some() {
            return Err(AppError::ValidationError { field: "".to_string(), message: "Task is already deleted".to_string() });
        }
        task.delete();
        Ok(())
    }

    pub fn add_comment_to_task(task: &mut Task, user_id: i32, comment: String) -> Result<(), AppError> {
        if task.status == TaskStatus::Completed {
            return Err(AppError::ValidationError { field: "".to_string(), message: "Cannot add comments to completed tasks".to_string() });
        }
        task.add_comment(user_id, comment);
        Ok(())
    }

    pub fn add_subtask(task: &mut Task, subtask: Task) -> Result<(), AppError> {
        if task.status == TaskStatus::Completed {
            return Err(AppError::ValidationError { field: "".to_string(), message: "Cannot add subtasks to completed tasks".to_string() });
        }
        task.add_subtask(subtask);
        Ok(())
    }

    pub fn set_task_due_date(task: &mut Task, due_date: Option<i64>) {
        task.set_due_date(due_date);
    }

    pub fn set_task_priority(task: &mut Task, priority: Option<i32>) {
        task.set_priority(priority);
    }

    pub fn set_task_recurrence(task: &mut Task, recurrence: Option<Recurrence>, recurrence_end: Option<i64>) {
        task.set_recurrence(recurrence, recurrence_end);
    }
}

pub struct UserService;

impl UserService {
    pub fn create_user(id: i32, username: String, password_hash: String, role: Role) -> Result<User, AppError> {
        if username.is_empty() || password_hash.is_empty() {
            return Err(AppError::ValidationError { field: "".to_string(), message: "Username and password cannot be empty".to_string() });
        }
        Ok(User::new(id, username, password_hash, role))
    }

    pub fn update_user_details(user: &mut User, name: Option<String>, surname: Option<String>, email: Option<String>) -> Result<(), AppError> {
        if let Some(n) = name {
            user.set_name(n);
        }
        if let Some(s) = surname {
            user.set_surname(s);
        }
        if let Some(e) = email {
            user.set_email(e);
        }
        Ok(())
    }

    pub fn follow_user(follower: &mut User, followee_id: i32) -> Result<(), AppError> {
        follower.add_following(followee_id);
        Ok(())
    }

    pub fn unfollow_user(follower: &mut User, followee_id: i32) -> Result<(), AppError> {
        follower.remove_following(followee_id);
        Ok(())
    }

    pub fn archive_user(user: &mut User) -> Result<(), AppError> {
        if user.archived_at.is_some() {
            return Err(AppError::ValidationError { field: "".to_string(), message: "User is already archived".to_string() });
        }
        user.archive_user();
        Ok(())
    }

    pub fn delete_user(user: &mut User) -> Result<(), AppError> {
        if user.deleted_at.is_some() {
            return Err(AppError::ValidationError { field: "".to_string(), message: "User is already deleted".to_string() });
        }
        user.delete_user();
        Ok(())
    }

    pub fn set_user_role(user: &mut User, role: Role) {
        user.set_role(role);
    }
}