use http::{Request, Response, request::Builder};

use std::net::TcpStream;
use std::io::prelude::*;

// Split a string to get keys and values out of it
fn split_keyval(string: &str) -> Result<(&str, &str), ()> {
    let keyval: Vec<&str> = string.split(": ").collect();

    if keyval.len() == 2 { return Err(()) }

    return Ok((keyval[0], keyval[1]));
}

// Parse the text got from http into a Request
pub fn parse_request(request: &str) -> Result<Request<String>, ()> {

    // Divide the string in lines
    let mut splitted_request: Vec<&str> = request.lines().collect();

    // Empty request
    if splitted_request.len() == 0 {
        return Err(());
    }

    // Divide the first line by spaces, it contains several info
    let first_line: Vec<&str> = splitted_request[0].split(' ').collect();
    
    // Remove the first item, which is handled by first_line
    splitted_request.remove(0);

    let mut builder: Builder = Builder::new()
        .method(first_line[0])
        .uri(first_line[1]);

    // Count the header lines
    let mut last_valid_line = -1;
    
    for key_val in &splitted_request {
        // Split key_val to get keys and values
        if let Ok((key, val)) = split_keyval(key_val) {
            builder = builder.header(key, val);
            last_valid_line += 1;
        } else {
            break
        }
    }

    // Remove the header
    for _ in 0..last_valid_line {
        splitted_request.remove(0);
    }

    let body_content = splitted_request.join("\n");
    Ok(builder.body(body_content).unwrap())
}

pub fn send_response(mut stream: TcpStream, response: Response<Vec<u8>>) {
    // Get the response as string
    let (response_header, mut response_body) = response.into_parts();

    // Text that will be sent, as bytes
    let mut final_response: Vec<u8> = vec![];

    // Insert version and status code
    {
        let version_str = format!("{:?}", response_header.version);
        let status_code_str = response_header.status.as_str();
        final_response.append(&mut format!("{} {}", version_str, status_code_str).as_bytes().to_owned());
    }

    final_response.push(b'\n');

    // Insert header fields
    for (key, value) in response_header.headers.iter() {
        final_response.append(&mut format!("{}: {}", key, value.to_str().unwrap()).as_bytes().to_owned());
        final_response.push(b'\n');
    }
    
    final_response.push(b'\n');

    // Insert body
    final_response.append(&mut response_body);

    // Send the response
    if let Err(_) = stream.write(final_response.as_slice()) {
        println!("Failed to send response")
    }
}
