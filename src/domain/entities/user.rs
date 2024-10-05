pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
}

impl User {
    pub fn new(id: i32, username: String, password_hash: String) -> Self {
        User { id, username, password_hash }
    }
}