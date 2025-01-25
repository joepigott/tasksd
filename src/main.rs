use piglog;
use taskscheduler::vars;
use taskscheduler::TaskQueue;
use taskscheduler::scheduler::Scheduler;
use taskscheduler::server::Server;
use std::fs;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};

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
    let mut scheduler = Scheduler::with_queue(Arc::clone(&queue));
    let mut server = Server::with_queue(Arc::clone(&queue));
    let sigterm = Arc::new(AtomicBool::new(false));

    // handle termination signals
    let ctrlc_sigterm = Arc::clone(&sigterm);
    tokio::spawn(async move {
        let sigint = tokio::signal::ctrl_c();
        let mut sigterm = match tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()) {
            Ok(s) => s,
            Err(e) => {
                piglog::error!("{e}");
                return;
            }
        };

        tokio::select! {
            _ = sigint => {
                piglog::info!("Received SIGINT. Signaling thread to exit...");
            },
            _ = sigterm.recv() => {
                piglog::info!("Received SIGTERM. Signaling thread to exit...");
            }
        }

        ctrlc_sigterm.store(true, Ordering::Relaxed);
    });

    // scheduler thread
    let sched_sigterm = Arc::clone(&sigterm);
    let sched_thread = tokio::spawn(async move {
        if let Err(e) = scheduler.run(Arc::clone(&sched_sigterm)).await {
            piglog::error!("{e}");
            return;
        }
    });

    // server thread
    let serv_thread = tokio::spawn(async move {
        if let Err(e) = server.run().await {
            piglog::error!("{e}");
            return;
        }
    });

    // wait on scheduler thread
    if let Err(e) = sched_thread.await {
        piglog::error!("{e}");
    };

    // server thread doesn't need to exit gracefully, so we can abort it
    serv_thread.abort();

    piglog::info!("Exited.");
}

fn recover_queue(storage: &str) -> Option<TaskQueue> {
    let data = fs::read_to_string(storage).ok()?;
    serde_json::from_str::<TaskQueue>(&data).ok()
}
