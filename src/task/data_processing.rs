use crate::task::{Executable, TaskMode};
use crate::types::error::TaskError;

pub struct DataProcessingTask {
    pub data: Vec<i32>,
    pub mode: TaskMode,
}

impl Executable for DataProcessingTask {
    fn execute(&self) -> Result<String, TaskError> {
        match self.mode {
            TaskMode::Dummy => {
                let sum: i32 = self.data.iter().sum();
                Ok(format!("Data sum: {}", sum))
            }
            TaskMode::Real => todo!()
        }
    }

    fn task_type(&self) -> &str {
        "process_data"
    }
}
