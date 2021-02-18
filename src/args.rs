use clap::{App, Arg, ArgMatches};

use crate::printing::*;
use crate::config;

pub fn parse_args() -> config::ParsedArgs {

	let matches: ArgMatches = {
		App::new("webserver")
			.version(env!("CARGO_PKG_VERSION"))
			.author(env!("CARGO_PKG_AUTHORS"))
			.about(env!("CARGO_PKG_DESCRIPTION"))
			.arg(Arg::with_name("verbosity")
				.short("v")
				.long("verbosity")
				.long_help(&format!(
"Sets the level of verbosity.
If no verbosity level is set or it's an invalid value,
the default value ({}) will be used.
Possible values:
 1 = Only errors
 2 = Errors and warnings
 3 = Errors, warnings and info
The -s flag gets priority over this option.", config::DEFAULT_VERB))
				.multiple(false)
				.takes_value(true)
				//.default_value("2")
				.value_name("N")
				.conflicts_with("silent"))
			.arg(Arg::with_name("silent")
				.short("s")
				.long("silent")
				.long_help(
"Enables the silent mode: no output gets printed at all.
This flag gets priority over the -v option.")
				.conflicts_with("verbosity")
				.multiple(false)
				.takes_value(false))
			.arg(Arg::with_name("no-tls")
				.long("no-tls")
				.help("No SSL/TLS in HTTP requests.")
				.multiple(false)
				.takes_value(false))
			//.arg(Arg::with_name("config")
			//	.short("c")
			//	.long("config")
			//	.help("Takes the configuration file from the specified path.")
			//	.multiple(false)
			//	.takes_value(true)
			//	.value_name("PATH"))
			.get_matches()
	};


	// Parse CLI args
	let mut args_config = config::ParsedArgs {
		use_tls: config::DEFAULT_USE_TLS,
		verbosity: config::DEFAULT_VERB
	};

	// HTTP mode (no TLS)
	if matches.is_present("no-https") {
		args_config.use_tls = false;
	}
	
	// Silent mode
	let mut is_silent = false;
	if matches.is_present("silent") {
		args_config.verbosity = 0;
		is_silent = true;
	}

	// Verbosity
	if !is_silent {
		let verb_val = matches.value_of("verbosity");
		args_config.verbosity = config::DEFAULT_VERB;

		if verb_val != None {
			println!();

			match verb_val.unwrap() {
				"1" | "2" | "3" => {

					// Set verbosity
					let value = verb_val.unwrap().parse::<u8>().unwrap();
					args_config.verbosity = value;

					print_info(format!("Verbosity level: {}", verb_val.unwrap()));
				},
				_ => {
					print_warn(format!("Invalid verbosity level, using default value: {}.", config::DEFAULT_VERB));
				},
			}
		} else {
			print_warn(format!("Verbosity level not set, using default value: {}.", config::DEFAULT_VERB));
		}
	}

	args_config
}