use std::fmt;

#[derive(Debug)]
pub enum TaskError {
    InvalidPayload,
    NetworkError(String),
    Unknown(String),
}

impl fmt::Display for TaskError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskError::InvalidPayload => write!(f, "Invalid payload"),
            TaskError::NetworkError(msg) => write!(f, "{}", msg),
            TaskError::Unknown(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for TaskError {}
