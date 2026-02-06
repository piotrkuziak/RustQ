use rustq::executor::manager::TaskManager;
use rustq::task::data_processing::DataProcessingTask;
use rustq::task::email::EmailTask;
use rustq::task::webhook::WebhookTask;
use rustq::task::TaskMode;

fn main() {
    let mut manager = TaskManager::new();

    manager.add_task(EmailTask {
        to: "test@email.com".to_string(),
        subject: "HUJ".to_string(),
        body: "Hello".to_string(),
        mode: TaskMode::Dummy,
    });

    while manager.has_tasks() {
        manager.execute_next();
    }
}
