// This file contains functions used for printing.
// Use format!() on the text arguments to use these functions as if they were println!().

use colored::Colorize;
use lazy_static;

use crate::config::DEFAULT_VERB;

lazy_static!{
	pub static ref ERROR_MARKER: String = "[E]".red().to_string();
	pub static ref WARNING_MARKER: String = "[W]".yellow().to_string();
	pub static ref INFO_MARKER: String = "[I]".clear().to_string();
}

pub static mut VERBOSITY: u8 = DEFAULT_VERB;

pub fn print_markers() {
	unsafe {
		if VERBOSITY > 0 {
			println!("{} {}", *ERROR_MARKER, "<- Error");
			if VERBOSITY > 1 {
				println!("{} {}", *WARNING_MARKER, "<- Warning");
				if VERBOSITY > 2 {
					println!("{} {}", *INFO_MARKER, "<- Info");
				}
			}
		}
	}
}

// Used to print an error to screen.
pub fn print_err<S: Into<String>>(text: S) {
	unsafe {
		if VERBOSITY >= 1 {
			eprintln!("{}", format!("{} {}", *ERROR_MARKER, text.into()));
		}
	}
}

// Used to print a warning message to screen.
pub fn print_warn<S: Into<String>>(text: S) {
	unsafe {
		if VERBOSITY >= 2 {
			println!("{}", format!("{} {}", *WARNING_MARKER, text.into()));
		}
	}
}

// Used to print an information to screen.
pub fn print_info<S: Into<String>>(text: S) {
	unsafe {
		if VERBOSITY >= 3 {
			println!("{}", format!("{} {}", *INFO_MARKER, text.into()));
		}
	}
}

// Prints '\n'
pub fn print_separator() {
	unsafe {
		// No need to print separators without output
		if VERBOSITY != 0 {
			println!();
		}
	}
}
