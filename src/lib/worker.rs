use std::sync::{Arc, Mutex, mpsc};
use std::thread::{self, JoinHandle};

use super::thread_pool::Job;

pub struct Worker {
    id: String,
    handle: Option<JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: String, rx: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let id2 = id.clone();

        let handle = thread::spawn(move || {
            loop {
                match rx.lock().unwrap().recv() {
                    Ok(job) => {
                        println!("Thread {id2}: run job");
                        if let Err(e) = job() {
                            eprintln!("Thread {id2}: failed job. {}", e)
                        }
                    }
                    Err(_) => {
                        eprintln!("Worker {id2} disconnected; shutting down.");
                        break;
                    }
                }
            }
        });

        Worker {
            id,
            handle: Some(handle),
        }
    }
}
