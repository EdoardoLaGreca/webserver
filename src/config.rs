extern crate toml;
use once_cell::sync::Lazy;
use serde_derive::Deserialize;

use crate::io_ops;
use crate::printing::print_err;

pub const CONFIG: Lazy<Config> = Lazy::new(|| Config::new());

#[derive(Deserialize)]
pub struct Server {
	pub address: String,
	pub threads: usize,
	pub err404_path: String,
	pub title: String,
    pub www_path: String
}

#[derive(Deserialize)]
pub struct Config {
	pub server: Server,
}

impl Config {
	// Returns a new Config instance
	// Call this function only once in the whole program
	fn new() -> Config {
		let config_file_content = io_ops::get_config_file();

		if let Err(_) = config_file_content {
			print_err("Couldn't open/read config.toml");
			std::process::exit(1);
		}

		toml::from_str(&config_file_content.unwrap())
			.expect(&format!("Couldn't parse config.toml: bad syntax."))
	}
}


