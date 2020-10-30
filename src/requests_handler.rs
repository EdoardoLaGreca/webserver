use http::{Request, Response, Method};
use regex::Regex;

use crate::io_ops::*;
use crate::markdown;

lazy_static! {
    static ref ERROR_404: (Vec<u8>, &'static str, u16) = ("ERROR 404: Not found.".into(), "text/plain", 404);
}

// Choose a route based on the method and the URI
fn choose_route<'a>(req_method: &Method, req_uri: &str) -> Option<(Vec<u8>, &'a str, u16)> {
    match (req_method, req_uri) {

        // GET /
        (&Method::GET, "/") => {
            get_checked_file_content(
                "index.html".into(),
                None,
                "text/html",
                200
            )
        },

        // GET /favicon.ico
        (&Method::GET, "/favicon.ico") => {
            get_checked_file_content(
                "icons/favicon.ico".into(),
                None,
                "image/x-icon",
                200
            )
        },

        _ => {

            if req_method == &Method::GET {

                // Regex expressions based on pages types
                let normal_page_regex = Regex::new(r"^(/[0-9A-z-_]+)+").unwrap();
                let style_regex = Regex::new(r"^(/[0-9A-z-_]+)+.css").unwrap();
                
                // Check if the match takes the entire string
                if let Some(r_match) = normal_page_regex.find(req_uri) {
                    if r_match.start() == 0 && r_match.end() == req_uri.len() { // Normal page/Markdown

                        // Remove the first character ('/') and return the markdown page
                        let md_page_name = req_uri.strip_prefix('/').unwrap();

                        let file_content = get_checked_file_content_string(
                            md_page_name.into(),
                            Some("md"),
                            "text/html",
                            200
                        );

                        if let None = file_content {
                            return None;
                        }

                        //let converted_md = markdown::Markdown::from_file(Path::new(&real_path)).unwrap();
                        let converted_md = markdown::Markdown::from_string(
                            file_content.unwrap().0
                        );
                        
                        let html_version = converted_md.to_html();

                        Some((html_version.as_bytes().to_vec(), "text/html", 200))

                    } else if let Some(r_match) = style_regex.find(req_uri) { // Stylesheet
                        
                        if r_match.start() == 0 && r_match.end() == req_uri.len() {

                            // Remove the first character ('/') and return the style page
                            let css_page_name = req_uri.strip_prefix('/').unwrap();

                            let file_content = get_checked_file_content(css_page_name.into(), None, "text/css", 200);

                            file_content
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }

                // Other cases...

            } else {

                None
            }
            
        }
    }
}

// Gets the file and returns Some(...)/None based on the Result returned by get_file_content()
fn get_checked_file_content<'a>(filename: String, extension: Option<&str>, mime_type: &'static str, http_code: u16) -> Option<(Vec<u8>, &'a str, u16)> {
    let file_content = get_file_content(filename, extension);

    if let Err(_) = file_content {
        return None;
    }

    Some((file_content.unwrap(), mime_type, http_code))
}

fn get_checked_file_content_string<'a>(filename: String, extension: Option<&str>, mime_type: &'static str, http_code: u16) -> Option<(String, &'a str, u16)> {
    let file_content = get_file_content_string(filename, extension);

    if let Err(_) = file_content {
        return None;
    }

    Some((file_content.unwrap(), mime_type, http_code))
}

// Returns: body content, content MIME type (html, plain text, etc...), status code
fn handle_request<'a>(req_method: &Method, req_uri: &str) -> (Vec<u8>, &'a str, u16) {

    println!("\nGot a request: {} {}", req_method.to_string(), req_uri);

    let response: Option<(Vec<u8>, &'a str, u16)> = choose_route(req_method, req_uri);


    if let None = response {
        return (ERROR_404.0.clone(), ERROR_404.1, ERROR_404.2);
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