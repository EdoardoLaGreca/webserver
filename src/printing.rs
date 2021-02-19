// This file contains functions used for printing.
// Use format!() on the text arguments to use these functions as if they were println!().

use colored::Colorize;
use lazy_static;

use crate::config::CONFIG;

lazy_static!{
	pub static ref ERROR_MARKER: String = "[E]".red().to_string();
	pub static ref WARNING_MARKER: String = "[W]".yellow().to_string();
	pub static ref INFO_MARKER: String = "[I]".clear().to_string();
}

pub enum MsgType {
	Error,
	Warning,
	Info
}

pub fn print_markers() {
	print_msg("<- Error", MsgType::Error);
	print_msg("<- Warning", MsgType::Warning);
	print_msg("<- Info", MsgType::Info);
}

// Print a message
pub fn print_msg<S: Into<String>>(text: S, msg_type: MsgType) {
	match msg_type {
		MsgType::Error => {
			if CONFIG.printing.verbosity >= 1 {
				eprintln!("{}", format!("{} {}", *ERROR_MARKER, text.into()));
			}
		},
		MsgType::Warning => {
			if CONFIG.printing.verbosity >= 2 {
				println!("{}", format!("{} {}", *WARNING_MARKER, text.into()));
			}
		},
		MsgType::Info => {
			if CONFIG.printing.verbosity >= 3 {
				println!("{}", format!("{} {}", *INFO_MARKER, text.into()));
			}
		}
	}
}

// Prints '\n'
pub fn print_separator() {
	// No need to print separators without output
	if CONFIG.printing.verbosity != 0 {
		println!();
	}
}
