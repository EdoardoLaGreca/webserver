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

use crate::defaults;

// Check if all the files needed to run are available
// true = ok
// false = files are missing
pub fn check_files() {

	for entity in &*defaults::BASE {
		
		// Check if the entity exists (can be anything: file, directory, etc...)
		if !Path::new(entity).exists() {
			if entity.ends_with("/") {
				// Create a directory and all of its parent components if they are missing
				fs::create_dir_all(entity).unwrap();
			} else {
				fs::File::create(entity).unwrap();
			}
		}
	}
}