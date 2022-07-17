use std::net::TcpStream;
use std::io::prelude::*;
use httparse;

pub struct ConnectionHandler<'a> {
    stream: &'a TcpStream,
    status: &'static str,
    message: &'static str
}

pub enum Method {
    GET,
    POST,
    Unimplemented
}

pub struct ConnectionResponse<'a> {
    pub status: &'static str,
    pub headers: [httparse::Header<'a>; 16],
    pub body: &'static str,
    pub method: Method
}

// impl<'a> ConnectionResponse<'a> {
//     fn get_method(&self) -> &'static str {
//         match &self.method {
//             Method::GET => "GET",
//             Method::POST => "POST",
//             _ => "Unimplemented",
//         }
//     }
// }

impl<'a> ConnectionHandler<'a> {
    pub fn new(stream: &'a TcpStream, message: &'static str, status: &'static str) -> ConnectionResponse<'a> {
        let mut handler = ConnectionHandler {
            stream: &stream,
            status: &status,
            message: &message
        };

        let response = handler.handle_connection();
        handler.handle_response();

        response
    }

    fn handle_connection(&mut self) -> ConnectionResponse<'a> {
        let mut buffer = [0; 1024];
        
        self.stream.read(&mut buffer).unwrap();

        // let _incoming_request = String::from_utf8_lossy(&buffer[..]);
        // let splited_lines = incoming_request.split("\r\n");

        // let method = match splited_lines[0] {
        //     "GET / HTTP/1.1\r\n" => Method::GET,
        //     "POST / HTTP/1.1\r\n" => Method::POST,
        //     _ => Method::Unimplemented,
        // }

        let mut headers = [httparse::EMPTY_HEADER; 16];
        let mut req = httparse::Request::new(&mut headers);
        let _res = req.parse(&buffer).unwrap();

        println!("{:?}", req.method);
        // TODO: Write a pattern matching for 404 and other stuff in here.
        ConnectionResponse { 
            status: "", 
            headers: headers, 
            body: "",
            method: Method::Unimplemented,
        }
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
