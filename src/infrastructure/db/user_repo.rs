use tokio_postgres::Client;
use crate::domain::entities::user::User;

pub struct UserRepository;

impl UserRepository {
    pub async fn find_user_by_id(client: &Client, user_id: i32) -> Result<User, tokio_postgres::Error> {
        let row = client
            .query_one("SELECT id, username, password_hash FROM users WHERE id = $1", &[&user_id])
            .await?;

        Ok(User {
            id: row.get(0),
            username: row.get(1),
            password_hash: row.get(2),
        })
    }
}