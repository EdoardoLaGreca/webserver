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
If no verbosity level is set or it's an invalid value,
the default value (2) will be used.
Possible values:
 1 = Only errors
 2 = Errors and warnings
 3 = Errors, warnings and info
The -s flag gets priority over this option.")
				.multiple(false)
				.takes_value(true)
				//.default_value("2")
				.value_name("N"))
			.arg(Arg::with_name("silent")
				.short("s")
				.long("silent")
				.long_help(
"Enables the silent mode: no output gets printed at all.
This flag gets priority over the -v option."))
			.get_matches()
	};


	// Parse CLI args
	
	// Silent mode
	// If it's present: print nothing, set VERB_LVL to 0 and return to skip the verbosity check
	if matches.is_present("silent") {
		set_verb_lvl(0);
		return;
	}

	// Verbosity
	let verb_val = matches.value_of("verbosity");
	let mut parsed_verb_val: u8 = 2;

	if verb_val != None {
		print_info(&format!("Verbosity level: {}", verb_val.unwrap()));

		match verb_val.unwrap() {
			"1" => {
				parsed_verb_val = 1;
			},
			"2" => {
				parsed_verb_val = 2;
			},
			"3" => {
				parsed_verb_val = 3;
			}
			_ => {
				print_warn(&format!("Invalid verbosity level, using default value: 2."))
			},
		}
	} else {
		print_info(&format!("Verbosity level not set, using default value: 2."));
	}

	set_verb_lvl(parsed_verb_val)
	
}