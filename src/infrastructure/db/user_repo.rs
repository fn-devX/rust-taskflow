use tokio_postgres::{Client, Error as PgError};
use crate::domain::entities::user::User;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserRepoError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] PgError),
    #[error("User with ID {0} not found")]
    UserNotFound(i32),
}

pub struct UserRepository;

impl UserRepository {
    pub async fn find_user_by_id(client: &Client, user_id: i32) -> Result<User, UserRepoError> {
        let row = client
            .query_opt("SELECT id, username, password_hash FROM users WHERE id = $1", &[&user_id])
            .await
            .map_err(UserRepoError::DatabaseError)?;

        if let Some(row) = row {
            Ok(User {
                id: row.get(0),
                username: row.get(1),
                password_hash: row.get(2),
            })
        } else {
            Err(UserRepoError::UserNotFound(user_id))
        }
    }

    pub async fn create_user(client: &Client, username: &str, password_hash: &str) -> Result<User, UserRepoError> {
        let row = client
            .query_one(
                "INSERT INTO users (username, password_hash) VALUES ($1, $2) RETURNING id, username, password_hash",
                &[&username, &password_hash],
            )
            .await
            .map_err(UserRepoError::DatabaseError)?;

        Ok(User {
            id: row.get(0),
            username: row.get(1),
            password_hash: row.get(2),
        })
    }

    pub async fn delete_user(client: &Client, user_id: i32) -> Result<(), UserRepoError> {
        let result = client
            .execute("DELETE FROM users WHERE id = $1", &[&user_id])
            .await
            .map_err(UserRepoError::DatabaseError)?;

        if result == 0 {
            Err(UserRepoError::UserNotFound(user_id))
        } else {
            Ok(())
        }
    }
}
