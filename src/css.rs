use grass;
use crate::printing::*;

// Use the grass library to compile Sass to CSS
pub fn sass_to_css(file_content: String, options: grass::Options) -> String {

	let css = grass::from_string(file_content, &options);

	// Avoid crash just for a Sass error
	if let Err(_) = css {
		print_err("Cannot compile the file into CSS, not going to send it.");
		return String::new();
	} else {
		return css.unwrap();
	}
}