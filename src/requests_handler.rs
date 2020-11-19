use http::{Request, Response, Method};

use crate::router::get_routes;
use crate::printing::*;
use crate::CONFIG;
use crate::io_ops;
use crate::defaults;

fn error_404() -> (Vec<u8>, String, u16) {

	if let Ok(content) = io_ops::get_file_content(CONFIG.get_page_404_path(), None) {
		let mg = mime_guess::from_path(CONFIG.get_page_404_path());
		let mime_type: String;

		if let Some(t) = mg.first() {
			mime_type = t.to_string();
		} else {
			mime_type = "text/plain".into();
		}

		(content, mime_type, 404)
	} else {
		print_warn(format!("404 error page path \"{}\" does not exist, using default page.", CONFIG.get_page_404_path()));

		(defaults::DEFAULT_404_PAGE_CONTENT.into(), "text/plain".into(), 404)
	}
}

// Choose a route based on the method and the URI
fn choose_route<'a>(req_method: &Method, req_uri: &str) -> Option<(Vec<u8>, String, u16)> {

	for route in get_routes() {
		if route.is_complete_match(req_method.clone(), req_uri) {
			return route.handle(req_uri);
		}
	}

	print_err(format!("Route \"{} {}\" not found.", req_method, req_uri));
	None
}

// Returns: body content, content MIME type (html, plain text, etc...), status code
fn handle_request<'a>(req_method: &Method, req_uri: &str) -> (Vec<u8>, String, u16) {

	print_info(format!("Request: {} {}", req_method.to_string(), req_uri));

	let response: Option<(Vec<u8>, String, u16)> = choose_route(req_method, req_uri);

	if let None = response {
		return error_404();
	}

	return response.unwrap();
}

pub fn response_builder(req: Request<String>) -> Response<Vec<u8>> {
	
	let (resp_body, content_type, status_code) = handle_request(req.method(), req.uri().path());

	Response::builder()
		.status(status_code)
		.header("Content-Type", content_type)
		.header("Content-Length", resp_body.len())
		.body(resp_body)
		.unwrap()
}