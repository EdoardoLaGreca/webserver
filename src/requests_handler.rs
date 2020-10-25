use http::{Request, Response, Method};
use crate::io_ops::get_file_content;

// Returns: body content, content MIME type (html, plain text, etc...), status code
fn deeper_handle<'a>(req_method: &Method, req_uri: &str) -> (Vec<u8>, &'a str, u16) {
    match (req_method, req_uri) {

        // GET /
        (&Method::GET, "/") => {
            (
                get_file_content("index.html"),
                "text/html",
                200
            )
        },

        // GET /favicon.ico
        (&Method::GET, "/favicon.ico") => {
            (
                get_file_content("icons/favicon.ico"),
                "image/x-icon",
                200
            )
        },

        _ => {
            (
                "ERROR 404: Not found.".into(),
                "text/plain",
                404
            )
        }
    }
}

pub fn handle_request(req: Request<String>) -> Response<Vec<u8>> {
    
    let (resp_body, content_type, status_code) = deeper_handle(req.method(), req.uri().path());

    Response::builder()
        .status(status_code)
        .header("Content-Type", content_type)
        .header("Content-Length", resp_body.len())
        .body(resp_body)
        .unwrap()
}