pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
}

pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
}

impl Task {
    pub fn new(id: i32, title: String, description: Option<String>) -> Self {
        Task {
            id,
            title,
            description,
            status: TaskStatus::Pending,
        }
    }

    pub fn set_status(&mut self, status: TaskStatus) {
        self.status = status;
    }
}