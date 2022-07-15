use std::net::TcpStream;
use std::io::prelude::*;

pub struct ConnectionHandler<'a> {
    stream: &'a TcpStream,
    status: &'static str,
    message: &'static str
}

#[derive(Debug)]
pub struct ConnectionResponse {
    pub status: &'static str,
    pub headers: &'static str,
    pub body: &'static str
}

impl<'a> ConnectionHandler<'a> {
    pub fn new(stream: &TcpStream, message: &'static str, status: &'static str) -> ConnectionResponse {
        let mut handler = ConnectionHandler {
            stream: &stream,
            status: &status,
            message: &message
        };

        let response = handler.handle_connection();
        handler.handle_response();

        response
    }

    fn handle_connection(&mut self) -> ConnectionResponse {
        let mut buffer = [0; 1024];
        
        self.stream.read(&mut buffer).unwrap();

        let _incoming_request = String::from_utf8_lossy(&buffer[..]);

        // TODO: Write a pattern matching for 404 and other stuff in here.
        ConnectionResponse { status: "", headers: "", body: "" }
    }

    fn handle_response(&mut self)  {

        let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}", 
            self.status, 
            self.message.len(), 
            self.message
        );

        self.stream.write(response.as_bytes()).unwrap();
        self.stream.flush().unwrap();
    }
}
