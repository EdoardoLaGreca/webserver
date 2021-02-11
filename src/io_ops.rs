use std::fs;
use std::path::PathBuf;

use crate::defaults;
use crate::printing::*;

// Get the content of a file which is located in the WWW directory (see defaults.rs module)
pub fn get_file_content(path: &String) -> Result<Vec<u8>, ()> {

	if path.is_empty() {
		return Err(());
	}

	// Path that includes WWW (but still a relative path)
	let mut complete_path: PathBuf = PathBuf::from(defaults::WWW);

	complete_path.push(&path);

	print_info(format!("Getting {} from disk...", complete_path.to_str().unwrap()));

	let content = fs::read(&complete_path);

	if let Err(_) = content {
		print_err(format!("Error while getting the file {}", complete_path.to_str().unwrap()));
		return Err(());
	}

	print_info(format!("Got {} from disk", complete_path.to_str().unwrap()));

	Ok(content.unwrap())
}