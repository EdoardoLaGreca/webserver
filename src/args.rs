use clap::{App, Arg, ArgMatches};
use crate::printing::*;

pub fn parse_args() {

	let matches: ArgMatches = {
		App::new("webserver")
			.version(env!("CARGO_PKG_VERSION"))
			.author(env!("CARGO_PKG_AUTHORS"))
			.about(env!("CARGO_PKG_DESCRIPTION"))
			.arg(Arg::with_name("verbosity")
				.short("v")
				.long("verbosity")
				.long_help(
"Sets the level of verbosity.
If no verbosity level is set or it's an invalid value, the default value (2) will be used.
Possible values:
 1 = Only errors
 2 = Errors and warnings
 3 = Errors, warnings and info
")
				.multiple(false)
				.takes_value(true)
				//.default_value("2")
				.value_name("N"))
			.arg(Arg::with_name("silent")
				.short("s")
				.long("silent")
				.help("Enables the silent mode: no output printed at all."))
			.get_matches()
	};


	// Parse CLI args

	// Verbosity
	let verb_val = matches.value_of("verbosity");
	let mut parsed_verb_val: u8 = 2;

	if verb_val != None {
		print_info(&format!("Verbosity level: {}", verb_val.unwrap()));
		match verb_val.unwrap() {
			"1" => {
				print_info(&format!("Printing only errors"));
				parsed_verb_val = 1;
			},
			"2" => {
				print_info(&format!("Printing errors and warnings"));
				parsed_verb_val = 2;
			},
			"3" => {
				print_info(&format!("Printing errors, warnings and info"));
				parsed_verb_val = 3;
			}
			_ => {
				print_warn(&format!("Invalid verbosity level. Using default value: 2."))
			},
		}
	} else {
		print_info(&format!("Verbosity level not set, using default value: 2."));
	}

	// Wait to check if there is the silent argument
	//VERB_LVL = parsed_verb_val;

	// Silent mode
	// If it's present, cancels the verbosity effect
	if matches.is_present("silent") {
		print_info("Silent mode enabled. Verbosity effect canceled.");
		parsed_verb_val = 0;
	}

	unsafe {
		VERB_LVL = parsed_verb_val;
	}
	
}