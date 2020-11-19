// This file contains functions used for printing.
// Use format!() on the text arguments to use these functions as if they were println!().

use colored::Colorize;
use lazy_static;

lazy_static!{
	pub static ref ERROR_MARKER: String = "[E]".red().to_string();
	pub static ref WARNING_MARKER: String = "[W]".yellow().to_string();
	pub static ref INFO_MARKER: String = "[I]".clear().to_string();
}

// Verbosity level
// 0 = silent
// 1 = Only errors
// 2 = Errors and warnings
// 3 = Errors, warnings and info
// If no verbosity level is set or it's an invalid value, keep it at 2.
// Note that if a library used to build this repo prints warning or error messages,
// they won't be handled and will be printed even if the silent flag is enabled.
pub static mut VERB_LVL: u8 = 3;

// Safe wrapper
pub fn get_verb_lvl() -> u8 {
	unsafe {
		VERB_LVL
	}
}

pub fn set_verb_lvl(value: u8) {
	unsafe {
		VERB_LVL = value;
	}
}

// Used to print an error to screen.
pub fn print_err<S: Into<String>>(text: S) {
	if get_verb_lvl() >= 1 {
		eprintln!("{}", format!("{} {}", *ERROR_MARKER, text.into()));
	}
}

// Used to print a warning message to screen.
pub fn print_warn<S: Into<String>>(text: S) {
	if get_verb_lvl() >= 2 {
		println!("{}", format!("{} {}", *WARNING_MARKER, text.into()));
	}
}

// Used to print an information to screen.
pub fn print_info<S: Into<String>>(text: S) {
	if get_verb_lvl() >= 3 {
		println!("{}", format!("{} {}", *INFO_MARKER, text.into()));
	}
}

// Prints '\n'
pub fn print_separator() {

	// No need to print separators without output
	if get_verb_lvl() != 0 {
		println!();
	}
}
