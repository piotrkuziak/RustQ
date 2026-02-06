use crate::task::{Executable, TaskMode};
use crate::types::error::TaskError;
use std::thread;
use std::time::Duration;

pub struct EmailTask {
    pub to: String,
    pub subject: String,
    pub body: String,
    pub mode: TaskMode,
}

impl Executable for EmailTask {
    fn execute(&self) -> Result<String, TaskError> {
        if !self.to.contains("@") {
            return Err(TaskError::InvalidPayload);
        }
        
        match self.mode {
            TaskMode::Dummy => {
                dbg!("[DUMMY] Sending email to: {}", &self.to);
                thread::sleep(Duration::from_millis(100));
                Ok(format!("Email sent to: {}", self.to))
            }
            TaskMode::Real => todo!()
        }
    }

    fn task_type(&self) -> &str {
        "send_email"
    }
}