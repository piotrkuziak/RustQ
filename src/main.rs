mod executor;
mod task;
mod types;

use executor::manager::TaskManager;
use task::email::EmailTask;

fn main() {
    let mut manager = TaskManager::new();

    manager.add_task(EmailTask {
        to: "test@email.com".into(),
        body: "Hello".into(),
    });

    while manager.has_tasks() {
        manager.execute_next();
    }
}
