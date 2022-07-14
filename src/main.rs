use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;

struct ConnectionInfo {
    host: &'static str,
    port: &'static str
}

struct ConnectionHandler<'a> {
    stream: &'a TcpStream,
    status: &'static str,
    message: &'static str
}

impl ConnectionInfo {
    fn to_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

impl<'a> ConnectionHandler<'a> {
    fn new(stream: &TcpStream, message: &'static str, status: &'static str) {
        let mut handler = ConnectionHandler {
            stream: &stream,
            status: &status,
            message: &message
        };

        handler.handle_connection();
        handler.handle_response();
    }

    fn handle_connection(&mut self) {
        let mut buffer = [0; 1024];
        
        self.stream.read(&mut buffer).unwrap();

        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
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

fn main() {
    let connection_info = ConnectionInfo {
        host: "127.0.0.1",
        port: "8080"
    };

    let listener = TcpListener::bind(connection_info.to_address()).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let message = "Hello, World";
        let status = "HTTP/1.1 200 OK\r\n\r\n";
        let handle = ConnectionHandler::new(&stream, &message, &status);
    }
}
