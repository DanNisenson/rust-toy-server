use std::sync::{Arc, Mutex, mpsc};

use super::server::ServerResult;
use super::worker::Worker;

pub type Job = Box<dyn FnOnce() -> ServerResult<()> + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    tx: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new() -> ThreadPool {
        let (tx, rx) = mpsc::channel::<Job>();
        let arc_rx = Arc::new(Mutex::new(rx));
        let mut workers = Vec::new();

        for i in 0..=3 {
            workers.push(Worker::new(i.to_string(), Arc::clone(&arc_rx)));
        }

        ThreadPool { workers, tx }
    }

    pub fn exec(&self, job: Job) {
        if let Err(e) = self.tx.send(job) {
            eprintln!("Error sending job to thread. {}", e);
        }
    }
}
