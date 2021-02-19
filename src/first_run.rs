/*
Server directory

config.toml
www/
|- favicon.ico
|- index.md
|- img/
|
|- style/
   |- default.scss
   |- font/
*/

use std::path::Path;
use std::fs;

use crate::config::CONFIG;
use crate::printing::{print_msg, MsgType};

// Check if all the files needed to run are available
// true = ok
// false = files are missing
pub fn check_files() {
	// Directories always end with "/", otherwise they will be recognized as files
	// Specify a directory only if it needs to be empty
	// The first tuple element represents the file path
	// The second tuple element represents the file content
	let www_path = &CONFIG.server.www_path;
	let base: [(String, Option<String>); 5] = [
		("config.toml".into(), Some(include_str!("../config.toml").into())),
		(format!("{}index.md", www_path), Some("Hello World!".into())),
		(format!("{}favicon.ico", www_path), None),
		(format!("{}style/default.scss", www_path), Some(include_str!("../www/style/default.scss").into())),
		(format!("{}style/font/", www_path), None),
	];

	for entity in base.iter() {
		let path = &entity.0;
		let content = &entity.1;

		// Check if the entity exists (can be anything: file, directory, etc...)
		if !Path::new(path).exists() {
			if path.ends_with("/") {
				// Create a directory and all of its parent components if they are missing
				if let Err(_) = fs::create_dir_all(path) {
					print_msg(format!("Unable to create {}", path), MsgType::Error);
					std::process::exit(1);
				}
			} else {
				if let Err(_) = fs::File::create(path) {
					print_msg(format!("Unable to create {}", path), MsgType::Error);
					std::process::exit(1);
				}

				// Write file if needed
				// If the program could create the file, it's likely to also be able to write it
				if let Some(c) = content {
					fs::write(path, c).unwrap();
				}
			}
		}
	}
}