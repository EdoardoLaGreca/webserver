use std::fs;
use std::path::PathBuf;

static WWW: &str = "www/";

// Get the content of a file which is located in the www directory
// Extensions are written without the point character before
pub fn get_file_content(filename: String, extension: Option<&str>) -> Result<Vec<u8>, ()> {

    let mut file_path: PathBuf = PathBuf::from(WWW);

    // Add extension if needed
    let mut filename_complete = {
        if let Some(ext) = extension {
            format!("{}.{}", filename, ext)
        } else {
            filename
        }
    };

    file_path.push(&filename_complete);

    println!("Getting {} from disk...", file_path.to_str().unwrap());

    let content = fs::read(file_path);

    if let Err(_) = content {
        eprintln!("  Error while getting the file.");
        return Err(());
    } else {
        println!("  Done!");
    }

    Ok(content.unwrap())
}

// Same as the function get_file_content() above but returns a String
pub fn get_file_content_string(filename: String, extension: Option<&str>) -> Result<String, ()>  {

    let file_content = get_file_content(filename, extension);

    if let Err(_) = file_content {
        return Err(())
    }

    Ok(
        String::from_utf8(file_content.unwrap()).unwrap()
    )
}