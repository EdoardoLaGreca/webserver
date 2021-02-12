extern crate toml;
use once_cell::sync::Lazy;
use serde_derive::Deserialize;

use crate::io_ops;

pub const CONFIG: Lazy<Config> = Lazy::new(|| Config::new());

#[derive(Deserialize)]
pub struct Server {
	pub address: String,
	pub threads: usize,
	pub err404_path: Path,
	pub title: String,
    pub www_path: Path
}

#[derive(Deserialize)]
pub struct Config {
	pub server: Server,
}

impl Config {
	// Returns a new Config instance
	// Call this function only once in the whole program
	fn new() -> Config {
		toml::from_str(get_config_file)
			.expect(&format!("Couldn't load config.toml: missing/corrupted file or bad syntax."))
	}
}


