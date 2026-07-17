mod lib;
mod threads;

use std::thread;
use std::time::Duration;

fn main() {
    let mut pool = threads::ThreadPool::new();

    thread::sleep(Duration::from_secs(20000));

    // if let Err(e) = lib::listen("127.0.0.1:8080") {
    //     eprintln!("FATAL: {e}");
    //     std::process::exit(1);
    // }
}


