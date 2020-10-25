extern crate http as libhttp;

use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;

mod http;
mod io_ops;
mod requests_handler;

fn handle_stream(mut stream: TcpStream) {
    let mut buffer: [u8; 2048] = [0; 2048];

    stream.read(&mut buffer).unwrap();

    // Elaborate the request
    let packet_content = &String::from_utf8(buffer.to_vec()).unwrap();
    let parsed_request = http::parse_request(packet_content);

    if let Ok(request) = parsed_request {
        let response = requests_handler::response_builder(request);

        http::send_response(stream, response);
    }
}

fn main() {
    // Listen on this address
    let address = "0.0.0.0:7878";

    let listener = TcpListener::bind(address)
        .expect(&format!("Cannot bind {}", address));

    listener.set_nonblocking(true).unwrap();

    for stream_res in listener.incoming() {

        if let Ok(stream) = stream_res {
            handle_stream(stream);
        }
    }
}