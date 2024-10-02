#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    ValidationError(String),
    DatabaseError(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::NotFound(msg) => write!(f, "Not Found: {}", msg),
            AppError::ValidationError(msg) => write!(f, "Validation Error: {}", msg),
            AppError::DatabaseError(msg) => write!(f, "Database Error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}