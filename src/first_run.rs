/*
Server directory

www/
|
|- index.md
|- meta.json
|- icon/
|  |- favicon.ico
|
|- en/
|  |- sample.md
|
|- style/
|  |- markdown.scss
|  |- font/
*/

use std::path::Path;
use std::fs;

// Directories always end with "/"
// Put directories before files stored in them
static BASE: [&str; 10] = [
	"www/",
	"www/index.md",
	"www/meta.json",
	"www/icon/",
	"www/icon/favicon.ico",
	"www/en/",
	"www/en/sample.md",
	"www/style/",
	"www/style/markdown.scss",
	"www/style/font/"
];


// Check if all the files needed to run are available
// true = ok
// false = files are missing
pub fn check_files() {

	for entity in &BASE {
		
		// Check if the entity exists (can be anything: file, directory, etc...)
		if !Path::new(entity).exists() {
			if entity.ends_with("/") {
				// Using create_dir_all() just in case (someone may forget to add an entity in `base`)
				fs::create_dir_all(entity).unwrap();
			} else {
				fs::File::create(entity).unwrap();
			}
		}
	}
}