mod lib;

use lib::server;

fn main() {
    if let Err(e) = server::run() {
        eprintln!("FATAL: {e}");
        std::process::exit(1);
    }
}
