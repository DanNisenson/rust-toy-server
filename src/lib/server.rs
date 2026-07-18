use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

use super::errors::ServerError;
use super::thread_pool::ThreadPool;

pub type ServerResult<T> = std::result::Result<T, ServerError>;

pub fn run() -> ServerResult<()> {
    let pool = ThreadPool::new();

    let listener = listen("127.0.0.1:8080")?;

    for stream in listener.incoming() {
        match stream {
            Ok(s) => pool.exec(Box::new(|| handle_client(s))),
            Err(e) => eprintln!("{}", ServerError::accept_stream(e)),
        }
    }

    Ok(())
}

fn listen(addr: &str) -> ServerResult<TcpListener> {
    let listener = TcpListener::bind(addr).map_err(|e| ServerError::tcp_bind(addr, e))?;
    println!(
        "Listening on {:?}",
        listener
            .local_addr()
            .map_err(|_| ServerError::new("Couldn't find local addr"))?
    );
    Ok(listener)
}

fn handle_client(mut stream: TcpStream) -> ServerResult<()> {
    let reader = BufReader::new(&stream);

    let header = match reader.lines().next() {
        Some(Ok(line)) => line,
        Some(Err(e)) => return Err(ServerError::parse_req(e)),
        None => return Err(ServerError::new("No data found in request")),
    };

    println!("{}", &header);

    let res = get_res(&header)?;

    stream
        .write_all(res.as_bytes())
        .map_err(|_| ServerError::new("Error writing response"))?;
    Ok(())
}

fn get_res(header: &str) -> ServerResult<String> {
    let sliced: Vec<&str> = header.split(" ").collect();

    match sliced.as_slice() {
        ["GET", "/", ..] => {
            let path = "static/index.html";
            let html = fs::read_to_string(path).map_err(|_| ServerError::read_file(path))?;
            Ok(format!("HTTP/1.1 200 OK\r\n\r\n{html}\r\n"))
        }
        _ => Ok("HTTP/1.1 404 KO\r\n\r\nNOT FOUND\r\n".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_res() {
        assert_eq!(
            get_res(""),
            Ok("HTTP/1.1 400 KO\r\n\r\nBAD REQUEST\r\n".to_string())
        );
        assert_eq!(
            get_res("GET / HTTP/1.1"),
            Ok(
                "HTTP/1.1 200 OK\r\n\r\n<!doctype html>\n<html lang=\"en\">\n  <head>\n    <meta charset=\"UTF-8\" />\n    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\" />\n    <title>Response HTML</title>\n  </head>\n  <body>\n    <h1>Good job!</h1>\n  </body>\n</html>\n\r\n".to_string()
            )
        );
    }
}
