extern crate toml;
use once_cell::sync::Lazy;
use serde_derive::Deserialize;

pub const CONFIG: Lazy<Config> = Lazy::new(|| 
	Config::new()
);

#[derive(Deserialize)]
pub struct Server {
	pub address: String,
	pub threads: usize,
	pub err404_path: String,
	pub title: String
}

#[derive(Deserialize)]
pub struct Config {
	pub server: Server,
}

impl Config {
	// Returns a new Config instance
	// Call this function only once in the whole program
	fn new() -> Config {
		let config_file_content = include_str!("../config.toml");

		toml::from_str(config_file_content)
			.expect(&format!("Couldn't load config.toml: missing/corrupted file or bad syntax."))
	}
}


