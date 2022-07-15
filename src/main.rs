mod connection;

use connection::handler::{ConnectionHandler};
use connection::information::{ConnectionInformation};
use std::net::TcpListener;

fn main() {
    let connection_info = ConnectionInformation {
        host: "127.0.0.1",
        port: "8080"
    };

    let listener = TcpListener::bind(connection_info.to_address()).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let message = "Hello, World";
        let status = "HTTP/1.1 200 OK";

        let response = ConnectionHandler::new(&stream, &message, &status);
        println!("Status: {:?}\n Headers: {:?}\n Body: {:?}", response.status, response.headers, response.body);
    }
}
