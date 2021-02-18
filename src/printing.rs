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

pub fn print_markers() {
	print_err("<- Error");
	print_warn("<- Warning");
	print_info("<- Info");
}

// Used to print an error to screen.
pub fn print_err<S: Into<String>>(text: S) {
	if CONFIG.printing.verbosity >= 1 {
		eprintln!("{}", format!("{} {}", *ERROR_MARKER, text.into()));
	}
}

// Used to print a warning message to screen.
pub fn print_warn<S: Into<String>>(text: S) {
	if CONFIG.printing.verbosity >= 2 {
		println!("{}", format!("{} {}", *WARNING_MARKER, text.into()));
	}
}

// Used to print an information to screen.
pub fn print_info<S: Into<String>>(text: S) {
	if CONFIG.printing.verbosity >= 3 {
		println!("{}", format!("{} {}", *INFO_MARKER, text.into()));
	}
}

// Prints '\n'
pub fn print_separator() {
	// No need to print separators without output
	if CONFIG.printing.verbosity != 0 {
		println!();
	}
}
