use libhttp::Method;
use mime_guess;
use regex::Regex;

use crate::html::{build_html_document, generate_title};
use crate::io_ops::{get_file_content, get_file_content_string};
use crate::metadata::Config;
use crate::css::sass_to_css;
use crate::printing::*;

// DO NOT write the "www/" part
static METADATA_PATH: &str = "meta.json";

// DO NOT write the "www/style/" part
static DEFAULT_MD_STYLE: &str = "markdown.scss";

// Webserver routes
pub fn get_routes() -> Vec<Route> {
	vec![
		// Route::new(
		//	 Method::GET, "/",
		//	 |_| {
		//		 if let Some(html) = get_md_file_as_html("index.md".into()) {
		//			 Some((html, "text/html".into(), 200))
		//		 } else {
		//			 None
		//		 }
		//	 }
		// ),
		Route::new(
			Method::GET, "/favicon.ico",
			|_| {
				if let Some(f_content) = get_checked_file_content("icon/favicon.ico".into(), None) {
					Some((f_content, "image/x-icon".into(), 200))
				} else {
					None
				}
			}
		),
		Route::new( // Markdown pages, valid for index ("/") as well
			Method::GET, r"^(/[0-9A-z-_]*)+",
			|req_uri| {

				// Remove the first character ('/') and return the markdown page name + language
				let md_page_name = {
					if req_uri == "/" {
						"index" // Index route ("/")
					} else {
						req_uri.strip_prefix('/').unwrap()
					}
				};
				

				let file_content = get_checked_file_content_string(md_page_name.into(), Some("md"));

				if let None = file_content {
					return None;
				}

				print_info(&format!("Translating markdown file {}.md into HTML...", md_page_name));

				// Get configuration from meta.json
				let config_file = Config::parse_metadata(METADATA_PATH);
				let config = config_file.get_by_path(req_uri);

				let converted_md = {
					// Check if there is a configuration for this
					// markdown file in meta.json
					if let Some(c) = config {

						let mut styles: Vec<String> = c.get_styles();
						styles.push(DEFAULT_MD_STYLE.to_owned());

						build_html_document(
							&file_content.unwrap(),
							&c.get_title(),
							styles,
							vec![],
							Some(&c.get_lang())
						)
					} else {
						let page_title = generate_title(req_uri);
			
						build_html_document(
							&file_content.unwrap(),
							&page_title,
							vec![DEFAULT_MD_STYLE.to_owned()],
							vec![],
							None
						)
					}
				};

				Some((converted_md.as_bytes().to_vec(), "text/html".into(), 200))
			}
		),
		Route::new( // Style dir
			Method::GET, r"^(/style/.+\..+)",
			|req_uri| {
				
				// Remove the first character ('/') and return the file
				let file_name = req_uri.strip_prefix('/').unwrap();

				let mg = mime_guess::from_path(file_name);
				let mut mime_type: String;

				if let Some(t) = mg.first() {
					mime_type = t.to_string();
				} else {
					mime_type = "*/*".into();
				}

				if let Some(f_content) = get_checked_file_content(file_name.into(), None) {

					if file_name.ends_with(".scss") {
						let final_content = sass_to_css(String::from_utf8(f_content).unwrap());
						mime_type = "text/css".into();
	
						return Some((final_content.as_bytes().to_vec(), mime_type, 200))
					}

					Some((f_content, mime_type, 200))
				} else {
					None
				}
			}
		)
	]
}


pub struct Route {
	method: Method,
	uri: Regex,
	handler: fn(&str) -> Option<(Vec<u8>, String, u16)>
}

impl Route {
	pub fn new(method: Method, uri_str: &str, handler: fn(&str) -> Option<(Vec<u8>, String, u16)>) -> Route {
		Route {
			method: method,
			uri: Regex::new(uri_str).unwrap(),
			handler: handler
		}
	}


	pub fn is_complete_match(&self, method: Method, path: &str) -> bool {
		if self.method == method {
			if let Some(r_match) = self.uri.find(path) {
				if r_match.start() == 0 && r_match.end() == path.len() {
					return true;
				}
			}
		}

		false
	}

	pub fn handle(&self, req_uri: &str) -> Option<(Vec<u8>, String, u16)> {
		let h = self.handler;
		h(req_uri)
	}
}

// fn get_md_file_as_html<'a>(path: &str) -> Option<Vec<u8>> {

//	 let html = get_md_file_as_html_string(path.into());

//	 if let None = html {
//		 return None;
//	 }

//	 let html_u = html.unwrap();

//	 Some(html_u.as_bytes().to_owned())
// }

// fn get_md_file_as_html_string<'a>(path: &str) -> Option<String> {

//	 let file_content = get_file_content_string(path.into(), None);

//	 if let Err(_) = file_content {
//		 return None;
//	 }

//	 Some(file_content.unwrap())
// }

// Gets the file and returns Some(...)/None based on the Result returned by get_file_content()
fn get_checked_file_content(filename: String, extension: Option<&str>) -> Option<Vec<u8>> {
	let file_content = get_file_content(filename, extension);

	if let Err(_) = file_content {
		return None;
	}

	Some(file_content.unwrap())
}

fn get_checked_file_content_string(filename: String, extension: Option<&str>) -> Option<String> {
	let file_content = get_file_content_string(filename, extension);

	if let Err(_) = file_content {
		return None;
	}

	Some(file_content.unwrap())
}