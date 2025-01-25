use piglog;
use taskscheduler::vars;
use taskscheduler::TaskQueue;
use taskscheduler::scheduler::Scheduler;
use taskscheduler::server::Server;
use std::fs;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    let storage = match vars::storage_path() {
        Ok(s) => s,
        Err(e) => {
            piglog::error!("{e}");
            return;
        }
    };

    let queue = Arc::new(Mutex::new(recover_queue(&storage).unwrap_or(TaskQueue::new())));
    let scheduler = Scheduler::with_queue(Arc::clone(&queue));
    let server = Server::with_queue(Arc::clone(&queue));
}

fn recover_queue(storage: &str) -> Option<TaskQueue> {
    let data = fs::read_to_string(storage).ok()?;
    serde_json::from_str::<TaskQueue>(&data).ok()
}
