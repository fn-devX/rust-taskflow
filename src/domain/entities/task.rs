use std::cmp::PartialEq;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Recurrence {
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Role {
    Leader,
    Contributor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collaborator {
    pub user_id: i32,
    pub role: Role,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub user_id: i32,
    pub comment: String,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub created_at: i64,
    pub updated_at: i64,
    pub due_date: Option<i64>,
    pub priority: Option<i32>,
    pub tags: Vec<String>,
    pub subtasks: Vec<Task>,
    pub parent_task: Option<i32>,
    pub assigned_to: Option<i32>,
    pub assigned_by: Option<i32>,
    pub completed_at: Option<i64>,
    pub archived_at: Option<i64>,
    pub deleted_at: Option<i64>,
    pub recurrence: Option<Recurrence>,
    pub recurrence_end: Option<i64>,
    pub dependencies: Vec<i32>,
    pub collaborators: Vec<Collaborator>,
    pub progress: Option<u8>,
    pub comments: Vec<Comment>,
    pub activity_log: Vec<String>,
    pub custom_fields: HashMap<String, String>,
}

impl PartialEq for TaskStatus {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl Task {
    // Constructor for a new task
    pub fn new(title: String, description: Option<String>) -> Self {
        Task {
            id: 0,
            title,
            description,
            status: TaskStatus::Pending,
            created_at: chrono::Utc::now().timestamp_millis(),
            updated_at: chrono::Utc::now().timestamp_millis(),
            due_date: None,
            priority: None,
            tags: Vec::new(),
            subtasks: Vec::new(),
            parent_task: None,
            assigned_to: None,
            assigned_by: None,
            completed_at: None,
            archived_at: None,
            deleted_at: None,
            recurrence: None,
            recurrence_end: None,
            dependencies: Vec::new(),
            collaborators: Vec::new(),
            progress: None,
            comments: Vec::new(),
            activity_log: Vec::new(),
            custom_fields: HashMap::new(),
        }
    }

    // Setters for various task attributes
    pub fn set_status(&mut self, status: TaskStatus) {
        self.status = status;
        self.update_timestamp();
        self.log_activity(format!("Status set to {:?}", status));
    }

    pub fn set_due_date(&mut self, due_date: Option<i64>) {
        self.due_date = due_date;
        self.update_timestamp();
        self.log_activity(format!("Due date set to {:?}", due_date));
    }

    pub fn set_priority(&mut self, priority: Option<i32>) {
        self.priority = priority;
        self.update_timestamp();
        self.log_activity(format!("Priority set to {:?}", priority));
    }

    pub fn add_tag(&mut self, tag: String) {
        self.tags.push(tag);
        self.log_activity(format!("Tag '{}' added", tag));
    }

    pub fn add_subtask(&mut self, subtask: Task) {
        self.subtasks.push(subtask);
        self.update_progress();
        self.log_activity("Subtask added".to_string());
    }

    pub fn complete(&mut self) {
        self.status = TaskStatus::Completed;
        self.completed_at = Some(chrono::Utc::now().timestamp_millis());
        self.update_timestamp();
        self.log_activity("Task completed".to_string());
    }

    pub fn archive(&mut self) {
        self.archived_at = Some(chrono::Utc::now().timestamp_millis());
        self.update_timestamp();
        self.log_activity("Task archived".to_string());
    }

    pub fn delete(&mut self) {
        self.deleted_at = Some(chrono::Utc::now().timestamp_millis());
        self.update_timestamp();
        self.log_activity("Task deleted".to_string());
    }

    pub fn set_recurrence(&mut self, recurrence: Option<Recurrence>, recurrence_end: Option<i64>) {
        self.recurrence = recurrence;
        self.recurrence_end = recurrence_end;
        self.log_activity(format!("Recurrence set to {:?}, ends at {:?}", recurrence, recurrence_end));
    }

    pub fn add_dependency(&mut self, task_id: i32) {
        self.dependencies.push(task_id);
        self.log_activity(format!("Dependency on task {} added", task_id));
    }

    pub fn can_be_completed(&self, other_tasks: &Vec<Task>) -> bool {
        for dep_id in &self.dependencies {
            if let Some(dep_task) = other_tasks.iter().find(|t| t.id == *dep_id) {
                if dep_task.status != TaskStatus::Completed {
                    return false;
                }
            }
        }
        true
    }

    pub fn add_collaborator(&mut self, user_id: i32, role: Role) {
        self.collaborators.push(Collaborator { user_id, role });
        self.log_activity(format!("Collaborator {} added with role {:?}", user_id, role));
    }

    pub fn remove_collaborator(&mut self, user_id: i32) {
        self.collaborators.retain(|c| c.user_id != user_id);
        self.log_activity(format!("Collaborator {} removed", user_id));
    }

    pub fn change_role(&mut self, user_id: i32, new_role: Role) {
        if let Some(collaborator) = self.collaborators.iter_mut().find(|c| c.user_id == user_id) {
            collaborator.role = new_role;
            self.log_activity(format!("Role of collaborator {} changed to {:?}", user_id, new_role));
        }
    }

    pub fn update_progress(&mut self) {
        let total = self.subtasks.len();
        if total > 0 {
            let completed = self.subtasks.iter().filter(|t| t.status == TaskStatus::Completed).count();
            self.progress = Some((completed as u8 * 100) / total as u8);
            self.log_activity(format!("Progress updated to {}%", self.progress.unwrap()));
        }
    }

    pub fn set_progress(&mut self, progress: u8) {
        if progress <= 100 {
            self.progress = Some(progress);
            self.log_activity(format!("Progress set to {}%", progress));
        }
    }

    pub fn add_comment(&mut self, user_id: i32, comment: String) {
        self.comments.push(Comment {
            user_id,
            comment: comment.clone(),
            timestamp: chrono::Utc::now().timestamp_millis(),
        });
        self.log_activity(format!("Comment added by user {}: {}", user_id, comment));
    }

    pub fn log_activity(&mut self, activity: String) {
        self.activity_log.push(format!(
            "{} - {}",
            chrono::Utc::now().to_rfc3339(),
            activity
        ));
    }

    pub fn add_custom_field(&mut self, key: String, value: String) {
        self.custom_fields.insert(key.clone(), value.clone());
        self.log_activity(format!("Custom field '{}' set to '{}'", key, value));
    }

    pub fn get_custom_field(&self, key: &str) -> Option<&String> {
        self.custom_fields.get(key)
    }

    fn update_timestamp(&mut self) {
        self.updated_at = chrono::Utc::now().timestamp_millis();
    }
}
