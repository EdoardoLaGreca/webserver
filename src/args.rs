use clap::{App, Arg, ArgMatches};

use crate::printing::*;
use crate::metadata;
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
			.arg(Arg::with_name("meta-path")
				.long("meta-path")
				.help("Specifies the path of meta.json")
				.takes_value(true)
				.value_name("PATH")
				.multiple(false))
			.get_matches()
	};


	// Parse CLI args
	
	// Silent mode
	if matches.is_present("silent") {
		set_verb_lvl(0);
		return;
	}

	// Verbosity
	let verb_val = matches.value_of("verbosity");
	let mut parsed_verb_val: u8 = defaults::DEFAULT_VERB;

	if verb_val != None {
		print_info(format!("Verbosity level: {}", verb_val.unwrap()));

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
				print_warn(format!("Invalid verbosity level, using default value: 2."))
			},
		}
	} else {
		print_info(format!("Verbosity level not set, using default value: 2."));
	}

	set_verb_lvl(parsed_verb_val);

	if let Some(p) = matches.value_of("meta-path") {
		metadata::set_meta_path(p);
	} else {
		metadata::set_meta_path(defaults::DEFAULT_META_PATH);
	}
	
}