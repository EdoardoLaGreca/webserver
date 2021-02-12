use comrak::markdown_to_html;
use regex::Regex;

use crate::io_ops::get_file_content;
use crate::defaults;
use crate::printing::print_info;
use crate::config::CONFIG;

// Convert Markdown into HTML by using comrak
// md_fc:				markdown file content
// page_title:			HTML page title (<title> ... </title>)
// stylesheets:			Style files (<link rel="stylesheet" href="...">)
fn build_html_document(md_fc: &str, page_title: &str, stylesheets: Vec<String>) -> String {

	let mut html_body = markdown_to_html(
		&md_fc,
		&defaults::COMRAK_OPTIONS
	);

	// Add enclosing body tags
	html_body = format!("<body>\n{}\n</body>", html_body);

	// Build title
	let title = {
		if page_title != "" {
			format!("<title> {} </title>", page_title)
		} else {
			String::new()
		}
	};

	let charset = "<meta charset=\"utf-8\">";

	// Add styles (line by line)
	let mut html_styles: Vec<String> = vec![];

	// Add <link> styles
	for link_style in stylesheets {
		html_styles.push(format!(r#"<link rel="stylesheet" href="/style/{}">"#, link_style))
	}

	let html_header: String = format!("<head>\n{}\n{}\n{}\n</head>", title, charset, html_styles.join("\n"));

	let final_html_document = format!("<!DOCTYPE html>\n{}\n{}\n</html>", html_header, html_body);

	final_html_document
}

fn generate_title(path: &str) -> String {
	// Get the last part of path (filename) without file extension
	let mut page_title = path
		.split('/')
		.last()
		.unwrap()
		.to_owned()
		.split('.')
		.nth(0)
		.unwrap()
		.to_owned();

	// Replace non-alphanumeric characters
	let regex = Regex::new(r"[^\w]+").unwrap();
	page_title = regex.replace_all(&page_title, " ").to_string();

	// Capitalize the first letter
	if let Some(r) = page_title.get_mut(0..1) {
		r.to_uppercase();
	}

	page_title
}

// "Compile" the markdown file into an HTML file.
// file_path does not include WWW
pub fn md_to_html(file_path: &String) -> Result<String, ()> {

	let file_content_bytes = get_file_content(&file_path);

	if let Err(_) = file_content_bytes {
		// File couldn't be read
		return Err(());
	}

	// Get file content as string from bytes
	let file_content_str = String::from_utf8(file_content_bytes.unwrap());

	// The file may be corrupted
	if let Err(_) = file_content_str {
		return Err(());
	}

	print_info(format!("Translating markdown file {}.md into HTML...", file_path));

	let page_title = format!("{} | {}", CONFIG.server.title, generate_title(&file_path));

	// Markdown file translated in HTML
	let html_translation = build_html_document(
		&file_content_str.unwrap(),
		&page_title,
		vec![defaults::DEFAULT_MD_STYLE.to_owned()],
	);

	print_info(format!("Markdown file {}.md translated into HTML", file_path));

	Ok(html_translation)
}