use std::time::Duration;
use std::thread;
use rand::random;

use crate::task::{Executable, TaskMode};
use crate::types::error::TaskError;

pub struct WebhookTask {
    pub url: String,
    pub payload: String,
    pub mode: TaskMode,
}

impl Executable for WebhookTask {
    fn execute(&self) -> Result<String, TaskError> {
        match self.mode {
            TaskMode::Dummy => {
                println!("Calling webhook: {}", self.url);
                thread::sleep(Duration::from_millis(200));

                if rand::random::<f32>() < 0.1 {
                    return Err(TaskError::NetworkError("Connection timeout".to_string()));
                }

                Ok("Webook delivered".to_string())
            }
            TaskMode::Real => todo!()
        }
    }

    fn task_type(&self) -> &str {
        "webhook"
    }
}
