pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub name: Option<String>,
    pub surname: Option<String>,
    pub email: Option<String>,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub followers: Option<Vec<i32>>,
    pub following: Option<Vec<i32>>,
    pub created_at: Option<i64>,
    pub updated_at: Option<i64>,
    pub archived_at: Option<i64>,
    pub deleted_at: Option<i64>,
    pub is_admin: Option<bool>,
    pub is_verified: Option<bool>,
}

impl User {
    pub fn new(id: i32, username: String, password_hash: String) -> Self {
        User {
            id,
            username,
            password_hash,
            name: None,
            surname: None,
            email: None,
            bio: None,
            image: None,
            followers: None,
            following: None,
            created_at: None,
            updated_at: None,
            archived_at: None,
            deleted_at: None,
            is_admin: None,
            is_verified: None,
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn set_surname(&mut self, surname: String) {
        self.surname = Some(surname);
    }

    pub fn set_email(&mut self, email: String) {
        self.email = Some(email);
    }

    pub fn set_bio(&mut self, bio: String) {
        self.bio = Some(bio);
    }

    pub fn set_image(&mut self, image: String) {
        self.image = Some(image);
    }

    pub fn add_follower(&mut self, follower_id: i32) {
        if let Some(ref mut followers) = self.followers {
            followers.push(follower_id);
        } else {
            self.followers = Some(vec![follower_id]);
        }
    }

    pub fn add_following(&mut self, following_id: i32) {
        if let Some(ref mut following) = self.following {
            following.push(following_id);
        } else {
            self.following = Some(vec![following_id]);
        }
    }

    pub fn remove_follower(&mut self, follower_id: i32) {
        if let Some(ref mut followers) = self.followers {
            followers.retain(|&id| id != follower_id);
        }
    }

    pub fn remove_following(&mut self, following_id: i32) {
        if let Some(ref mut following) = self.following {
            following.retain(|&id| id != following_id);
        }
    }

    pub fn update_password(&mut self, new_password_hash: String) {
        self.password_hash = new_password_hash;
    }

    pub fn set_admin_status(&mut self, is_admin: bool) {
        self.is_admin = Some(is_admin);
    }

    pub fn set_verification_status(&mut self, is_verified: bool) {
        self.is_verified = Some(is_verified);
    }

    pub fn archive_user(&mut self, timestamp: i64) {
        self.archived_at = Some(timestamp);
    }

    pub fn delete_user(&mut self, timestamp: i64) {
        self.deleted_at = Some(timestamp);
    }

    pub fn update_timestamp(&mut self, timestamp: i64) {
        self.updated_at = Some(timestamp);
    }
}