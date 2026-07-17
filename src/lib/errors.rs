use std::fmt;
use std::io;

#[derive(Debug, PartialEq)]
pub struct ServerError(String);

impl ServerError {
    pub fn new(msg: &str) -> ServerError {
        ServerError(msg.to_string())
    }

    pub fn tcp_bind(addr: &str, error: io::Error) -> ServerError {
        ServerError::new(&format!(
            "Failed to bind TCP listener. addr: {}. {}",
            addr,
            error.to_string()
        ))
    }

    pub fn accept_stream(error: io::Error) -> ServerError {
        ServerError::new(&format!(
            "Failed to accept incoming stream. {}",
            error.to_string()
        ))
    }

    pub fn parse_req(error: io::Error) -> ServerError {
        ServerError::new(&format!("Failed to parse request. {}", error.to_string()))
    }

    pub fn read_file(path: &str) -> ServerError {
        ServerError::new(&format!("Error reading file. Path: {}", path))
    }
}

impl fmt::Display for ServerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}