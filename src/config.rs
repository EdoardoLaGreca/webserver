extern crate toml;
use serde_derive::Deserialize;

use crate::io_ops;
use crate::args;

pub const DEFAULT_ADDRESS: &str = "127.0.0.1:80";
pub const DEFAULT_THREADS: usize = 4;
pub const DEFAULT_VERB: u8 = 2;
pub const DEFAULT_404_PAGE_CONTENT: &str = "ERROR 404: Not found.";
pub const DEFAULT_USE_TLS: bool = true;

// Root path: repo root
pub const DEFAULT_CONFIG_PATH: &str = "config.toml";
pub const DEFAULT_WWW: &str = "www/";

// Root path: WWW
pub const DEFAULT_PAGE_404_PATH: &str = "404.md";

// Root path: WWW/style/
pub const DEFAULT_MD_STYLE: &str = "default.scss";

// Program's internal configuration loaded on server start
lazy_static! {
	pub static ref CONFIG: Config = Config::init(args::parse_args());
}


// Parsed+<smth> means it is used as an intermediate sturct between the parsed input data and the
// final Config constant.
// ParsedConfig and ParsedServer structs are used to parse the config.toml file.
// ParsedArgs struct is used to parse the command line arguments.


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

pub struct ParsedArgs {
	pub verbosity: u8,
	pub use_tls: bool,
}

#[derive(Clone, Debug)]
pub struct Config {
	pub server: Server,
	pub printing: Printing,
}

#[derive(Clone, Debug)]
pub struct Server {
	pub address: String,
	pub threads: usize,
	pub err404_path: String,
	pub title: String,
    pub www_path: String,
	pub use_tls: bool,
	
}

#[derive(Clone, Debug)]
pub struct Printing {
	pub verbosity: u8,
}

impl Config {
	pub fn init(args_config: ParsedArgs) -> Self {

		let toml_file_config = Config::parse_pers_config();

		// Take a ParsedConfig instance and put default values on Nones
		// ParsedConfig -> Config
		Config {
			server: {
				let server = toml_file_config.server.unwrap();

				Server {
					address: server.address.unwrap_or(DEFAULT_ADDRESS.into()),
					threads: server.threads.unwrap_or(DEFAULT_THREADS),
					err404_path: server.err404_path.unwrap_or(DEFAULT_PAGE_404_PATH.into()),
					title: server.title.unwrap_or("".into()),
					www_path: server.www_path.unwrap_or(DEFAULT_WWW.into()),
					use_tls: args_config.use_tls
				}
			},
			printing: Printing {
				verbosity: args_config.verbosity
			}
		}
	}

	// Returns parsed config.toml (persistent config)
	// Call this function only once in the whole program
	fn parse_pers_config() -> ParsedConfig {
		let config_file_content = io_ops::get_config_file();

		let config_file_content = config_file_content.expect(&format!("Couldn't open/read {}", DEFAULT_CONFIG_PATH));

		let config: ParsedConfig = toml::from_str(&config_file_content)
			.expect(&format!("Couldn't parse {}: bad syntax.", DEFAULT_CONFIG_PATH));

		config
	}
}
