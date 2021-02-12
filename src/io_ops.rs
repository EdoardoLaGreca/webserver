use std::fs;
use std::path::PathBuf;

use crate::config;
use crate::defaults;
use crate::printing::*;

// Check if a file can be sent by the webserver by checking if it's inside WWW
fn is_file_accessible<P: AsRef<Path>>(path: P) {
    if path.canonicalize().starts_with(config::CONFIG::www_path) {
        return true;
    }

    false
}

// Returns the config file's content (config.toml)
// This is the only function that can return a file placed outside WWW
pub fn get_config_file() -> String {
    fs::read(Path::new("config.toml"));
}

// Get the content of a file which is located in the WWW directory (see defaults.rs module)
pub fn get_file_content<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, ()> {

	if path.is_empty() {
		return Err(());
	}

	// Path that includes WWW (but technically still a relative path)
	let complete_path: PathBuf = PathBuf::from(defaults::WWW).push(&path);

    // Check whether the requested file is contained in WWW
    if !is_file_accessible(complete_path) {
        print_warn(format!("File {} cannot be accessed because it resides outside the WWW directory."));
        return Err(());
    }

	print_info(format!("Getting {} from disk...", complete_path.to_str().unwrap()));

	let content = fs::read(&complete_path);

	if let Err(_) = content {
		print_err(format!("Error while getting the file {}", complete_path.to_str().unwrap()));
		return Err(());
	}

	print_info(format!("Got {} from disk", complete_path.to_str().unwrap()));

	Ok(content.unwrap())
}
