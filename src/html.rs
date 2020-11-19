use comrak::markdown_to_html;
use regex::Regex;

use crate::defaults;

// Convert Markdown into HTML by using comrak
// markdown_file_content: Content of the markdown file
// crk_opts:			  comrak options
// page_title:			  HTML page title (<title> ... </title>)
// ext_styles:			  External style files (<link rel="stylesheet" href="...">)
// emb_styles:			  Embedded styles (<style> ... </style>)
// page_lang:			  Page language (<html lang="...">)
pub fn build_html_document(markdown_file_content: &str, page_title: &str, ext_styles: Vec<String>, emb_styles: Vec<&str>, page_lang: Option<&str>) -> String {

	let mut html_body = markdown_to_html(
		&markdown_file_content,
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

	// Add styles (line by line)
	let mut html_styles: Vec<String> = vec![];

	// Add <link> styles
	for link_style in ext_styles {
		html_styles.push(format!(r#"<link rel="stylesheet" href="/style/{}">"#, link_style))
	}

	// Add embedded styles
	if emb_styles.len() > 0 {
		html_styles.push("<style>".into());
		for embedded_style in emb_styles {
			html_styles.push(embedded_style.clone().to_owned());
		}
		html_styles.push("</style>".into());
	}

	let html_header: String = format!("<head>\n{}\n{}\n</head>", title, html_styles.join("\n"));

	let html_comment_content = "This document was automatically generated using a custom HTML generator.\nFor more information visit: https://github.com/EdoardoLaGreca/webserver";
	let html_comment = format!("<!-- {} -->", html_comment_content);

	let html_page_lang = {
		match page_lang {
			Some(lang) => lang.clone(),
			None => "".into()
		}
	};

	let final_html_document = format!("{}\n<!DOCTYPE html>\n<html lang=\"{}\">\n<meta charset=\"utf-8\">\n{}\n{}\n</html>", html_comment, html_page_lang, html_header, html_body);

	final_html_document
}

pub fn generate_title(path: &str) -> String {
	// Get the last part of path (filename)
	let mut page_title = path
		.split('/')
		.last()
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