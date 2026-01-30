pub mod email;

use crate::types::error::TaskError;

pub trait Executable {
    fn execute(&self) -> Result<String, TaskError>;
    fn task_type(&self) -> &str;
}
