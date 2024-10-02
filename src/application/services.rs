use crate::domain::entities::{task::Task, user::User};

pub struct TaskService;

impl TaskService {
    pub fn create_task(title: String, description: Option<String>) -> Task {
        Task::new(0, title, description)
    }

    pub fn complete_task(task: &mut Task) {
        task.set_status(crate::domain::entities::task::TaskStatus::Completed);
    }
}

pub struct UserService;

impl UserService {
    pub fn create_user(id: i32, username: String, password_hash: String) -> User {
        User::new(id, username, password_hash)
    }
}