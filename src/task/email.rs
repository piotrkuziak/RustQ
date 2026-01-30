use crate::task::Executable;
use crate::types::error::TaskError;

pub struct EmailTask {
    pub to: String,
    pub body: String,
}

impl Executable for EmailTask {
    fn execute(&self) -> Result<String, TaskError> {
        if !self.to.contains("@") {
            return Err(TaskError::InvalidPayload);
        }
        Ok(format!("Email sent to {}", self.to))
    }

    fn task_type(&self) -> &str {
        "send_email"
    }
}