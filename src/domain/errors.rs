use std::fmt;

#[derive(Debug)]
pub enum AppError {
    NotFound { resource: String, id: i32 },
    ValidationError { field: String, message: String },
    DatabaseError { source: Box<dyn std::error::Error> },
    Unauthorized { user_id: i32, action: String },
    Forbidden { user_id: i32, action: String },
    NotImplemented(String),
    InvalidInput(String),
    PermissionDenied(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NotFound { resource, id } => {
                write!(f, "{} with ID {} not found", resource, id)
            }
            AppError::ValidationError { field, message } => {
                write!(f, "Validation Error in '{}': {}", field, message)
            }
            AppError::DatabaseError { source } => {
                write!(f, "Database Error: {}", source)
            }
            AppError::Unauthorized { user_id, action } => {
                write!(f, "User {} is unauthorized to perform action: {}", user_id, action)
            }
            AppError::Forbidden { user_id, action } => {
                write!(f, "User {} is forbidden from performing action: {}", user_id, action)
            }
            AppError::NotImplemented(msg) => {
                write!(f, "Not Implemented: {}", msg)
            }
            AppError::InvalidInput(msg) => {
                write!(f, "Invalid Input: {}", msg)
            }
            AppError::PermissionDenied(msg) => {
                write!(f, "Permission Denied: {}", msg)
            }
        }
    }
}

impl std::error::Error for AppError {}

impl AppError {
    pub fn not_found(resource: &str, id: i32) -> Self {
        AppError::NotFound {
            resource: resource.to_string(),
            id,
        }
    }

    pub fn validation_error(field: &str, message: &str) -> Self {
        AppError::ValidationError {
            field: field.to_string(),
            message: message.to_string(),
        }
    }

    pub fn database_error<E: std::error::Error + 'static>(source: E) -> Self {
        AppError::DatabaseError {
            source: Box::new(source),
        }
    }

    pub fn unauthorized(user_id: i32, action: &str) -> Self {
        AppError::Unauthorized {
            user_id,
            action: action.to_string(),
        }
    }

    pub fn forbidden(user_id: i32, action: &str) -> Self {
        AppError::Forbidden {
            user_id,
            action: action.to_string(),
        }
    }

    pub fn invalid_input(message: &str) -> Self {
        AppError::InvalidInput(message.to_string())
    }

    pub fn permission_denied(message: &str) -> Self {
        AppError::PermissionDenied(message.to_string())
    }
}