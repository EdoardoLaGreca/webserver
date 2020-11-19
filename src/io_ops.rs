use std::fs;
use std::path::PathBuf;

use crate::defaults;
use crate::printing::*;

// Get the content of a file which is located in the www directory
// Extensions are written without the point character before
pub fn get_file_content(filename: String, extension: Option<&str>) -> Result<Vec<u8>, ()> {

	let mut file_path: PathBuf = PathBuf::from(defaults::WWW);

	// Add extension if needed
	let filename_complete = {
		if let Some(ext) = extension {
			format!("{}.{}", filename, ext)
		} else {
			filename
		}
	};

	file_path.push(&filename_complete);

	print_info(format!("Getting {} from disk...", file_path.to_str().unwrap()));

	let content = fs::read(&file_path);

	if let Err(_) = content {
		print_err(format!("Error while getting the file {}", file_path.to_str().unwrap()));
		return Err(());
	}

	Ok(content.unwrap())
}

// Same as the function get_file_content() above but returns a String
pub fn get_file_content_string(filename: String, extension: Option<&str>) -> Result<String, ()>  {

	let file_content = get_file_content(filename.clone(), extension);

	if let Err(_) = file_content {
		return Err(())
	}

	// Check if bytes can be converted into String
	if let Ok(s) = String::from_utf8(file_content.unwrap()) {
		Ok(s)
	} else {
		print_err(format!("Cannot read file {} as UTF-8. Not going to send it", filename));
		Err(())
	}
}