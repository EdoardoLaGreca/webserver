use std::fs;
use std::path::{Path, PathBuf};

use crate::config;
use crate::defaults;
use crate::printing::*;

// Check if a file can be sent by the webserver by checking if it's inside WWW
fn is_file_accessible<P: AsRef<Path>>(path: P) -> bool {
	let root_dir = Path::new(&config::CONFIG.server.www_path).canonicalize().unwrap();

	if path.as_ref().canonicalize().unwrap().starts_with(&root_dir) {
		return true;
	}

	false
}

// Returns the config file's content (config.toml)
// This is the only function that can return a file placed outside WWW
pub fn get_config_file() -> Result<String, ()> {
	let file_content = fs::read_to_string(Path::new(defaults::DEFAULT_CONFIG_PATH));

	if let Ok(c) = file_content {
		return Ok(c);
	}

	Err(())
}

// Get the content of a file which is located in the WWW directory (see defaults.rs module)
pub fn get_file_content<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, ()> {

	// Path that includes WWW (but technically still a relative path)
	let mut complete_path: PathBuf = PathBuf::from(&config::CONFIG.server.www_path);
	complete_path.push(&path);

	print_info(format!("Getting {} from disk...", complete_path.to_str().unwrap()));

	// Check if the file exists
	if !path.as_ref().exists() {
		print_warn(format!("File {} does not exist.", path.as_ref().to_str().unwrap()));
		return Err(());
	}

	// Check whether the requested file is contained in WWW
	if !is_file_accessible(&complete_path) {
		print_warn(format!("File {} cannot be accessed because it resides outside the WWW directory.", path.as_ref().to_str().unwrap()));
		return Err(());
	}

	let content = fs::read(&complete_path);

	if let Err(_) = content {
		print_err(format!("Error while getting the file {}", complete_path.to_str().unwrap()));
		return Err(());
	}

	Ok(content.unwrap())
}
