use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Role {
    Admin,
    Regular,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub name: Option<String>,
    pub surname: Option<String>,
    pub email: Option<String>,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub followers: Vec<i32>,
    pub following: Vec<i32>,
    pub created_at: i64,
    pub updated_at: i64,
    pub archived_at: Option<i64>,
    pub deleted_at: Option<i64>,
    pub role: Role,
    pub is_verified: bool,
    pub activity_log: Vec<String>,
    pub custom_fields: HashMap<String, String>,
}

impl User {
    pub fn new(id: i32, username: String, password_hash: String, role: Role) -> Self {
        User {
            id,
            username,
            password_hash,
            name: None,
            surname: None,
            email: None,
            bio: None,
            image: None,
            followers: Vec::new(),
            following: Vec::new(),
            created_at: chrono::Utc::now().timestamp_millis(),
            updated_at: chrono::Utc::now().timestamp_millis(),
            archived_at: None,
            deleted_at: None,
            role,
            is_verified: false,
            activity_log: Vec::new(),
            custom_fields: HashMap::new(),
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
        self.update_timestamp();
        self.log_activity(format!("Name set to {}", name));
    }

    pub fn set_surname(&mut self, surname: String) {
        self.surname = Some(surname);
        self.update_timestamp();
        self.log_activity(format!("Surname set to {}", surname));
    }

    pub fn set_email(&mut self, email: String) {
        self.email = Some(email);
        self.update_timestamp();
        self.log_activity(format!("Email set to {}", email));
    }

    pub fn set_bio(&mut self, bio: String) {
        self.bio = Some(bio);
        self.update_timestamp();
        self.log_activity("Bio updated".to_string());
    }

    pub fn set_image(&mut self, image: String) {
        self.image = Some(image);
        self.update_timestamp();
        self.log_activity("Profile image updated".to_string());
    }

    pub fn add_follower(&mut self, follower_id: i32) {
        if !self.followers.contains(&follower_id) {
            self.followers.push(follower_id);
            self.log_activity(format!("User {} followed", follower_id));
        }
    }

    pub fn remove_follower(&mut self, follower_id: i32) {
        if self.followers.contains(&follower_id) {
            self.followers.retain(|&id| id != follower_id);
            self.log_activity(format!("User {} unfollowed", follower_id));
        }
    }

    pub fn add_following(&mut self, following_id: i32) {
        if !self.following.contains(&following_id) {
            self.following.push(following_id);
            self.log_activity(format!("Started following user {}", following_id));
        }
    }

    pub fn remove_following(&mut self, following_id: i32) {
        if self.following.contains(&following_id) {
            self.following.retain(|&id| id != following_id);
            self.log_activity(format!("Stopped following user {}", following_id));
        }
    }

    pub fn update_password(&mut self, new_password_hash: String) {
        self.password_hash = new_password_hash;
        self.update_timestamp();
        self.log_activity("Password updated".to_string());
    }

    pub fn set_role(&mut self, role: Role) {
        self.role = role;
        self.update_timestamp();
        self.log_activity(format!("Role set to {:?}", role));
    }

    pub fn set_verification_status(&mut self, is_verified: bool) {
        self.is_verified = is_verified;
        self.update_timestamp();
        self.log_activity(format!("Verification status set to {}", is_verified));
    }

    pub fn archive_user(&mut self) {
        self.archived_at = Some(chrono::Utc::now().timestamp_millis());
        self.update_timestamp();
        self.log_activity("User archived".to_string());
    }

    pub fn delete_user(&mut self) {
        self.deleted_at = Some(chrono::Utc::now().timestamp_millis());
        self.update_timestamp();
        self.log_activity("User deleted".to_string());
    }

    pub fn add_custom_field(&mut self, key: String, value: String) {
        self.custom_fields.insert(key.clone(), value.clone());
        self.log_activity(format!("Custom field '{}' set to '{}'", key, value));
    }

    pub fn get_custom_field(&self, key: &str) -> Option<&String> {
        self.custom_fields.get(key)
    }

    pub fn log_activity(&mut self, activity: String) {
        self.activity_log.push(format!(
            "{} - {}",
            chrono::Utc::now().to_rfc3339(),
            activity
        ));
    }

    pub fn update_timestamp(&mut self) {
        self.updated_at = chrono::Utc::now().timestamp_millis();
    }
}