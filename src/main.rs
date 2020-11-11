extern crate http as libhttp;
#[macro_use]
extern crate lazy_static;
use chrono::prelude::*;
use colored::Colorize;

use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;

use crate::printing::*;

mod http;
mod css;
mod io_ops;
mod requests_handler;
mod router;
mod html;
mod metadata;
mod printing;
mod args;

fn main() {
	// Listen on this address
	let address = "0.0.0.0:7878";

	// Separator
	println!("\n");

	args::parse_args();

	print_separator();

	// Print legend based on the verbosity level
	match get_verb_lvl() {
		1 => {
			println!("Legend:\n{} <- Error",
				*ERROR_MARKER,
			);
		},
		2 => {
			println!("Legend:\n{} <- Error\n{} <- Warning",
				*ERROR_MARKER,
				*WARNING_MARKER
			);
		},
		3 => {
			println!("Legend:\n{} <- Error\n{} <- Warning\n{} <- Info",
				*ERROR_MARKER,
				*WARNING_MARKER,
				*INFO_MARKER
			);
		}
		0 | _ => ()
	}

	print_separator();

	let listener = TcpListener::bind(address)
		.expect(&format!("Cannot bind {}", address));

	listener.set_nonblocking(true).unwrap();

	print_info(&format!("Server started, listening on {}", listener.local_addr().unwrap()));

	for stream_res in listener.incoming() {

		if let Ok(stream) = stream_res {
			handle_stream(stream);
		}

		// No overhead CPU usage
		sleep(Duration::from_millis(5));
	}
}

fn handle_stream(mut stream: TcpStream) {

	let mut buffer: [u8; 2048] = [0; 2048];

	stream.read(&mut buffer).unwrap();

	let current_time = Local::now().format("%H:%M:%S (UTC%:z)");
	let now = Instant::now();

	print_info(&format!("[{}] New request.", current_time));
	print_info(&format!("Client address: {}", stream.peer_addr().unwrap().to_string().green()));

	// Elaborate the request
	let packet_content = &String::from_utf8(buffer.to_vec()).unwrap();
	let parsed_request = http::parse_request(packet_content);

	if let Ok(request) = parsed_request {
		let response = requests_handler::response_builder(request);

		http::send_response(stream, response);
	}

	let elapsed = now.elapsed();

	print_info(&format!("Completed in {}ms ({})", elapsed.as_millis(), {
		let elapsed_secs = elapsed.as_secs();
		
		if elapsed_secs > 0 {
			format!("{} seconds", elapsed_secs)
		} else {
			let elapsed_micros = elapsed.as_micros();
			format!("{}μs", elapsed_micros)
		}
	}));
}