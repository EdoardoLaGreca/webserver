use clap::{App, Arg, ArgMatches};

use crate::printing::*;
use crate::defaults;

pub fn parse_args() {

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
The -s flag gets priority over this option.", defaults::DEFAULT_VERB))
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
				.conflicts_with("verbosity"))
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
	
	// Silent mode
	let mut is_silent = false;
	if matches.is_present("silent") {
		unsafe { VERBOSITY = 0; }
		is_silent = true;
	}

	// Verbosity
	if !is_silent {
		let verb_val = matches.value_of("verbosity");
		unsafe { VERBOSITY = defaults::DEFAULT_VERB; }

		if verb_val != None {
			println!();

			match verb_val.unwrap() {
				"1" | "2" | "3" => {

					// Set verbosity
					let value = verb_val.unwrap().parse::<u8>().unwrap();
					unsafe { VERBOSITY = value; }

					print_info(format!("Verbosity level: {}", verb_val.unwrap()));
				},
				_ => {
					print_warn(format!("Invalid verbosity level, using default value: {}.", defaults::DEFAULT_VERB));
				},
			}
		} else {
			print_warn(format!("Verbosity level not set, using default value: {}.", defaults::DEFAULT_VERB));
		}
	}
}