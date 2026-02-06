use crate::task::Executable;

pub struct TaskManager {
    tasks: Vec<Box<dyn Executable>>,
}

impl TaskManager {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    pub fn add_task<T: Executable + 'static>(&mut self, task: T) {
        self.tasks.push(Box::new(task));
    }

    pub fn execute_next(&mut self) {
        if let Some(task) = self.tasks.pop() {
            println!("Executing {}", task.task_type());

            match task.execute() {
                Ok(res) => eprintln!("Success: {}", res),
                Err(e) => eprintln!("Error: {}", e),
            }
        }
    }

    pub fn has_tasks(&self) -> bool {
        !self.tasks.is_empty()
    }
}