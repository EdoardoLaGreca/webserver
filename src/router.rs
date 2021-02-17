use libhttp::Method;
use mime_guess;
use regex::Regex;

use crate::html::md_to_html;
use crate::io_ops::get_file_content;
use crate::css::{get_default_grass_options, sass_to_css};

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
		Route::new( // Markdown pages, valid for index ("/") as well
			Method::GET, r"^(/[0-9A-z-_]*)+",
			|req_uri| {

				// Remove the first character ('/') to get the markdown page name
				let md_page_path = {
					if req_uri == "/" {
						"index.md".into() // Index route ("/")
					} else {
						format!("{}.md", req_uri.strip_prefix('/').unwrap())
					}
				};

				let converted_md = md_to_html(&md_page_path);

				if let Err(_) = converted_md {
					return None;
				}

				Some((converted_md.unwrap().as_bytes().to_vec(), "text/html".into(), 200))
			}
		),
		Route::new( // Other files (CSS, HTML, etc...) except Markdown
			Method::GET, r"^(/.+\..+)",
			|req_uri| {
				
				// Remove the first character ('/') and return the file
				let file_name = req_uri.strip_prefix('/').unwrap();

				if let Some(f_content) = get_checked_file_content(&file_name.into()) {
					// Guess the MIME type
					let mg = mime_guess::from_path(file_name);
					let mut mime_type: String;

					// The first guess is probably the most accurate
					if let Some(t) = mg.first() {
						mime_type = t.to_string();
					} else {
						mime_type = "*/*".into();
					}

					// Compile SCSS to CSS if needed
					if file_name.ends_with(".scss") {
						let final_content = sass_to_css(String::from_utf8(f_content).unwrap(), get_default_grass_options());
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

// Gets the file and returns Some(...)/None based on the Result returned by get_file_content()
// The function(s) are wrappers that basically tell whether a file exists or not
fn get_checked_file_content(path: &String) -> Option<Vec<u8>> {
	let file_content = get_file_content(&path);

	if let Err(_) = file_content {
		return None;
	}

	Some(file_content.unwrap())
}