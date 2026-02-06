pub mod email;
pub mod data_processing;
pub mod webhook;

use crate::types::error::TaskError;

pub enum TaskMode {
    Dummy,
    Real,
}

pub trait Executable {
    fn execute(&self) -> Result<String, TaskError>;
    fn task_type(&self) -> &str;
}
