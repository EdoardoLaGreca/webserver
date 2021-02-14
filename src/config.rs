extern crate toml;
use once_cell::sync::Lazy;
use serde_derive::Deserialize;

use crate::io_ops;
use crate::defaults;

pub const CONFIG: Lazy<Config> = Lazy::new(|| Config::parse());

// ParsedConfig and ParsedServer structs are used to parse the config.toml file
// The real config is kept through Config and Server structs

#[derive(Deserialize)]
struct ParsedServer {
	address: Option<String>,
	threads: Option<usize>,
	err404_path: Option<String>,
	title: Option<String>,
    www_path: Option<String>
}

#[derive(Deserialize)]
struct ParsedConfig {
	server: Option<ParsedServer>,
}

pub struct Server {
	pub address: String,
	pub threads: usize,
	pub err404_path: String,
	pub title: String,
    pub www_path: String
}

pub struct Config {
	pub server: Server,
}

impl Config {
	// Returns a new Config instance
	// Call this function only once in the whole program
	pub fn parse() -> Config {
		let config_file_content = io_ops::get_config_file();

		let config_file_content = config_file_content.expect(&format!("Couldn't open/read {}", defaults::DEFAULT_CONFIG_PATH));

		let config: ParsedConfig = toml::from_str(&config_file_content)
			.expect(&format!("Couldn't parse {}: bad syntax.", defaults::DEFAULT_CONFIG_PATH));

		// Take a ParsedConfig instance and put default values on Nones
		// ParsedConfig -> Config
		Config {
			server: {
				let server = config.server.unwrap();

				Server {
					address: server.address.unwrap_or(defaults::DEFAULT_ADDRESS.into()),
					threads: server.threads.unwrap_or(defaults::DEFAULT_THREADS),
					err404_path: server.err404_path.unwrap_or(defaults::DEFAULT_PAGE_404_PATH.into()),
					title: server.title.unwrap_or("".into()),
					www_path: server.www_path.unwrap_or(defaults::WWW.into()),
				}
			}
		}
	}
}


