use std::fs;

// Get the content of a file which is located in the www directory
pub fn get_file_content(filename: &str) -> Vec<u8> {
    let file_path = format!("www/{}", filename);

    let content: Vec<u8> = fs::read(file_path).unwrap();

    content
}

// Same as above but returns a String
// pub fn get_file_content_str(filename: &str) -> String {
//     String::from_utf8(
//         get_file_content(filename)
//     ).unwrap()
// }