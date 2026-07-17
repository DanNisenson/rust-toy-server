

use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex, mpsc};
use std::thread::{self, JoinHandle};

struct Worker {
    id: String,
    handle: Option<JoinHandle<()>>,
}

type Job = Box<dyn FnOnce() + Send>;

impl Worker {
    pub fn new(id: String, r: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let id2 = id.clone();

        let handle = thread::spawn(move || {
            loop {
                match r.lock().unwrap().recv() {
                    Ok(job) => {
                        println!("Thread {id2}: run job");
                        job();
                    }
                    Err(_) => {
                        println!("Worker {id2} disconnected; shutting down.");
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

pub struct ThreadPool {
    workers: Vec<Worker>,
    tx: Sender<Job>,
}

impl ThreadPool {
    pub fn new() -> ThreadPool {
        let (tx, rx) = mpsc::channel::<Job>();
        let arc_rx = Arc::new(Mutex::new(rx));
        let mut workers = Vec::new();

        for i in 0..=3 {
            workers.push(Worker::new(i.to_string(), Arc::clone(&arc_rx)));
        }

        tx.send(Box::new(|| println!("Hello from channel")))
            .unwrap();
        tx.send(Box::new(|| println!("Hello from channel")))
            .unwrap();
        tx.send(Box::new(|| println!("Hello from channel")))
            .unwrap();
        tx.send(Box::new(|| println!("Hello from channel")))
            .unwrap();
        tx.send(Box::new(|| println!("Hello from channel")))
            .unwrap();
        tx.send(Box::new(|| println!("Hello from channel")))
            .unwrap();
        tx.send(Box::new(|| println!("Hello from channel")))
            .unwrap();
        tx.send(Box::new(|| println!("Hello from channel")))
            .unwrap();
        tx.send(Box::new(|| println!("Hello from channel")))
            .unwrap();
        tx.send(Box::new(|| println!("Hello from channel")))
            .unwrap();
        tx.send(Box::new(|| println!("Hello from channel")))
            .unwrap();
        tx.send(Box::new(|| println!("Hello from channel")))
            .unwrap();
        tx.send(Box::new(|| println!("Hello from channel")))
            .unwrap();
        tx.send(Box::new(|| println!("Hello from channel")))
            .unwrap();

        ThreadPool { workers, tx }
    }
}