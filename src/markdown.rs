//
// My own implementation of a MD -> HTML
//

use regex::{self, Regex, RegexBuilder};

use std::default::Default;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

struct MdSettings {
    // CSS styles (<link>)
    css_styles_link: Vec<PathBuf>,

    // CSS styles (<style>)
    css_styles_embedded: Vec<String>,

    // Page title (<title>)
    page_title: String,

    // Page language
    page_lang: Option<String>
}

// Tokenized markdown representation
pub struct Markdown {
    content: String,
    settings: MdSettings
}

impl Markdown {

    pub fn new() -> Markdown {
        Markdown {
            content: String::new(),
            settings: MdSettings::default()
        }
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Markdown, ()> {
        let mut file_content = String::new();
        let file = File::open(&path);

        if let Err(_) = file {
            return Err(());
        }

        if let Err(_) = file.unwrap().read_to_string(&mut file_content) {
            eprintln!("Could not get \"{}\" from disk.", path.as_ref().to_str().unwrap());
        }

        Ok(Markdown::from_string(file_content))
    }

    pub fn from_string<S: ToString>(content: S) -> Markdown {

        let mut md = Markdown::new();
        md.content = content.to_string();
        md.settings = MdSettings::default();

        md
    }

    // Convert Markdown into HTML by using regex replacements
    pub fn to_html(&self) -> String {

        // (<regex_expr>, <html_text>, <multiline_flag>, <needed_>)
        let conversion_table = [
            (r"(^#{6}) (?P<text>.+)", r#"<h6> $text </h6>"#, true), // H6
            (r"(^#{5}) (?P<text>.+)", r#"<h5> $text </h5>"#, true), // H5
            (r"(^#{4}) (?P<text>.+)", r#"<h4> $text </h4>"#, true), // H4
            (r"(^#{3}) (?P<text>.+)", r#"<h3> $text </h3>"#, true), // H3
            (r"(^#{2}) (?P<text>.+)", r#"<h2> $text </h2>"#, true), // H2
            (r"(^#{1}) (?P<text>.+)", r#"<h1> $text </h1>"#, true), // H1
            (r"[*]{3}(?P<text>[^*\n]+)[*]{3}", r#"<b><i> $text </i></b>"#, true), // Bold and italic text
            (r"[*]{2}(?P<text>[^*\n]+)[*]{2}", r#"<b> $text </b>"#, true), // Bold text
            (r"[*]{1}(?P<text>[^*\n]+)[*]{1}", r#"<i> $text </i>"#, true), // Italic text
            (r"^> (?P<text>.+)", r#"<blockquote> $text </blockquote>"#, true), // Blockquote
            (r"[`]{2}(?P<text>[^`\n]+)[`]{2}", r#"<code> $text </code>"#, true), // Code (2 backticks)
            (r"[`]{1}(?P<text>[^`\n]+)[`]{1}", r#"<code> $text </code>"#, true), // Code (1 backtick)
        ];

        let mut html_body = self.content.clone();

        // Apply regex expressions to do the conversion
        for (expr, replacement, is_multiline) in conversion_table.iter() {

            let regex_expr = RegexBuilder::new(expr).multi_line(*is_multiline).build().unwrap();

            html_body = regex_expr.replace_all(&html_body, *replacement).to_string();
        }

        // Convert other markdown elements which cannot be converted just by using a regex expression
        
        // Convert code blocks
        {
            // HTML body as vector of lines
            let mut html_body_lines: Vec<String> = html_body.lines()
                .map(|item| { item.to_owned() })
                .collect();

            let regex_expr = RegexBuilder::new(r"^(?P<indentation>[ ]{4,}|[\t]+[ ]*)(?P<text>.+)").multi_line(true).build().unwrap();

            // Used to check if the content is correct since the "regex" crate does not support the (?! ) group
            let regex_expr_2 = RegexBuilder::new(r"^([ ]*(\d.|-) )").multi_line(true).build().unwrap();

            // Size of indent used by markdown to represent a code block
            // This variable is initialized because otherwise the Rust compiler would think it would possibly be initialized at
            // some point in the for loop
            let mut md_indent_size: usize = 0;

            // Index of the currentmatch
            let mut match_index = 0;

            for (i, line) in html_body_lines.clone().iter().enumerate() {
                if regex_expr.is_match(line) {

                    let code_text = regex_expr.replace_all(&line, "$text").to_string();

                    if !regex_expr_2.is_match(&code_text) {
                        let md_indentation = regex_expr.replace_all(&line, "$indentation").to_string();
                    
                        
                        if match_index == 0 {
                            md_indent_size = md_indentation.len() as usize;
                            html_body_lines.insert(i, "<code>".into());
                        } else {
                            // Actual indentation used in code
                            let code_indentation: String = md_indentation.chars().take(md_indent_size).collect();
        
                            html_body_lines[i] = format!("{}{}", code_indentation, code_text);
                        }

                        match_index += 1;
                    }
                } else {
                    if match_index != 0 {
                        html_body_lines.insert(i, "</code>".into());
                        match_index = 0;
                    }
                }
            }

            // Replace html_body content with html_body_lines
            html_body = html_body_lines.join("\n");
        }

        // Convert ordered lists (also nested)
        // DOES NOT SUPPORT TABS, use spaces to indent
        {
            // HTML body as vector of lines
            let mut html_body_lines: Vec<String> = html_body.lines()
                .map(|item| { item.to_owned() })
                .collect();

            let regex_expr = RegexBuilder::new(r"^(?P<indentation>[ ]*)(?P<number>\d). (?P<text>.+)").multi_line(true).build().unwrap();

            // Size of indent used by markdown to represent a code block
            // This variable is initialized because otherwise the Rust compiler would think it would possibly be initialized at
            // some point in the for loop
            let mut md_indent_size: usize = 0;

            // Index of the currentmatch
            let mut match_index = -1;

            // Keep the index of html_body_lines's clone (used as iterator in the for loop) synchronized with the
            // original html_body_lines's index variable.
            let mut inserted_lines: usize = 0;

            // True if the list is invalid (e.g. does not start with 1)
            let mut invalid = false;

            for (i, line) in html_body_lines.clone().iter().enumerate() {

                if regex_expr.is_match(line) {
                    match_index += 1;

                    if invalid {
                        continue;
                    }

                    let indentation = regex_expr.replace_all(&line, "$indentation").to_string();
                    let number = regex_expr.replace_all(&line, "$number").to_string(); // Used only for the first
                    let text = regex_expr.replace_all(&line, "$text").to_string();

                    if match_index == 0 && number.parse::<u32>().unwrap() != 1 {
                        invalid = true;
                        continue;
                    }

                    if match_index == 0 || indentation.len() > md_indent_size {

                        md_indent_size = indentation.len() as usize;
                        html_body_lines.insert(i + inserted_lines, "<ol>".into());
                        inserted_lines += 1;
                        
                    } else if indentation.len() < md_indent_size {
                        html_body_lines.insert(i + inserted_lines, "</ol>".into());
                        inserted_lines += 1;
                    }

                    html_body_lines[i + inserted_lines] = format!("<li>{}</li>", text);

                } else {
                    if match_index != -1 && !invalid {
                        html_body_lines.insert(i + inserted_lines, "</ol>".into());
                        inserted_lines += 1;
                        match_index = -1;
                    }

                    invalid = false;
                }

            }

            // Replace html_body content with html_body_lines
            html_body = html_body_lines.join("\n");
        }

        // Convert unordered lists (also nested)
        // DOES NOT SUPPORT TABS, use spaces to indent
        {
            // HTML body as vector of lines
            let mut html_body_lines: Vec<String> = html_body.lines()
                .map(|item| { item.to_owned() })
                .collect();

            let regex_expr = RegexBuilder::new(r"^(?P<indentation>[ ]*)(?P<number>\d). (?P<text>.+)").multi_line(true).build().unwrap();

            // Size of indent used by markdown to represent a code block
            // This variable is initialized because otherwise the Rust compiler would think it would possibly be initialized at
            // some point in the for loop
            let mut md_indent_size: usize = 0;

            // Index of the currentmatch
            let mut match_index = -1;

            // Keep the index of html_body_lines's clone (used as iterator in the for loop) synchronized with the
            // original html_body_lines's index variable.
            let mut inserted_lines: usize = 0;

            for (i, line) in html_body_lines.clone().iter().enumerate() {

                if regex_expr.is_match(line) {
                    match_index += 1;

                    let indentation = regex_expr.replace_all(&line, "$indentation").to_string();
                    let text = regex_expr.replace_all(&line, "$text").to_string();

                    if match_index == 0 || indentation.len() > md_indent_size {

                        md_indent_size = indentation.len() as usize;
                        html_body_lines.insert(i + inserted_lines, "<ul>".into());
                        inserted_lines += 1;
                        
                    } else if indentation.len() < md_indent_size {
                        html_body_lines.insert(i + inserted_lines, "</ul>".into());
                        inserted_lines += 1;
                    }

                    html_body_lines[i + inserted_lines] = format!("<li>{}</li>", text);
                } else {
                    if match_index != -1 {
                        html_body_lines.insert(i + inserted_lines, "</ul>".into());
                        inserted_lines += 1;
                        match_index = -1;
                    }
                }
            }

            // Replace html_body content with html_body_lines
            html_body = html_body_lines.join("\n");
        }

        // ...

        // Add <p> tags
        // Basically insert the <p> tag (and the closing one at the end) when the text is contained
        // by no tag except for <body>
        {
            // HTML body as vector of lines
            let mut html_body_lines: Vec<String> = html_body.lines()
                .map(|item| { item.to_owned() })
                .collect();

            // Regex expression used to 
            let regex = Regex::new(r"<\s*(?P<tagname>[^!\s]+).*>").unwrap();

            // A stack-like vector (LIFO) that contains HTML tags (e.g. ["body", "h1"])
            let mut tags_stack: Vec<String> = vec![];

            for (i, line) in html_body_lines.clone().iter().enumerate() {
                if regex.is_match(line) {

                    for found_match in regex.find_iter(line) {
                        let tagname = regex.replace_all(&found_match.as_str(), "$tagname").to_string();
                        
                        // Add/Remove tags to/from the stack
                        if tagname.starts_with("/") {
                            if tags_stack.len() > 0 && tags_stack.last().unwrap() == &tagname {
                                tags_stack.pop();
                            }
                        } else {
                            tags_stack.push(tagname);
                        }

                        // Check if there is some text in the tag's line
                        if 

                        if tags_stack == vec!["body"] {
                        }
                    }
                }
            }

            // Replace html_body content with html_body_lines
            html_body = html_body_lines.join("\n");
        }

        // Add enclosing body tags
        html_body = format!("<body>\n{}\n</body>", html_body);

        // Build title
        let title = format!("<title>{}</title>", self.settings.page_title);

        // Add styles (line by line)
        let mut html_styles: Vec<String> = vec![];

        // Add <link> styles
        for link_style in &self.settings.css_styles_link {
            html_styles.push(format!(r#"<link rel="stylesheet" href="style/{}">"#, link_style.to_str().unwrap()))
        }

        // Add embedded styles
        if self.settings.css_styles_embedded.len() > 0 {
            html_styles.push("<style>".into());
            for embedded_style in &self.settings.css_styles_embedded {
                html_styles.push(embedded_style.clone());
            }
            html_styles.push("</style>".into());
        }

        let html_header: String = format!("<head>\n{}\n{}\n</head>", title, html_styles.join("\n"));

        let html_comment_content = "This document was automatically generated using a custom generator.\nFor more information visit: https://github.com/EdoardoLaGreca/webserver";
        let html_comment = format!("<!-- {} -->", html_comment_content);

        let final_html_document = format!("{}\n<!DOCTYPE html>\n<html>\n<meta charset=\"utf-8\">\n{}\n{}\n</html>", html_comment, html_header, html_body);

        final_html_document
    }
}

impl MdSettings {
    pub fn new(mut css_styles_link: Vec<PathBuf>, css_styles_embedded: Vec<String>, page_title: String, page_lang: Option<String>) -> MdSettings {
        
        css_styles_link.push("markdown.css".into());
        
        MdSettings {
            css_styles_link: css_styles_link,
            css_styles_embedded: css_styles_embedded,
            page_title: page_title,
            page_lang: page_lang
        }
    }   
}

// Default: no styles and empty page title
impl Default for MdSettings {
    fn default() -> MdSettings {
        MdSettings {
            css_styles_link: vec!["markdown.css".into()],
            css_styles_embedded: vec![],
            page_title: String::new(),
            page_lang: None
        }
    }
}