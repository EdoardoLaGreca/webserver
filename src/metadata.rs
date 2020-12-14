use json;

use std::path::Path;

use crate::io_ops::get_file_content_string;
use crate::printing::*;
use crate::defaults;

static mut META_PATH: String = String::new();

#[derive(Clone, Debug, Default)]
pub struct Config {
	address: String,
	threads: usize,
	verbosity: u8,
	page_404_path: String,
	pages: Vec<PageMetadata>
}

#[derive(Clone, Debug, Default)]
pub struct PageMetadata {
	filename: String,
	title: String,
	lang: String,
	path: String,
	translations: Vec<PageTranslation>,
	styles: Vec<String>
}

#[derive(Clone, Debug, Default)]
pub struct PageTranslation {
	lang: String,
	path: String
}

pub fn set_meta_path(path: &str) {

	let meta_path: String;

	if !Path::new(path).exists() {
		print_warn(format!("Specified path \"{}\" does not exists, using default path.", path));

		meta_path = defaults::DEFAULT_META_PATH.into();
	} else {
		meta_path = path.into();
	}

	unsafe {
		META_PATH = meta_path;
	}
}

pub fn get_meta_path() -> String {
	unsafe {
		META_PATH.clone()
	}
}

impl Config {

	// If one or more required values are empty, use default values.
	pub fn parse_metadata() -> Self {
		print_info("Reading the JSON metadata file ({})...");

		let metadata_content = get_file_content_string(get_meta_path(), None);

		if let Err(_) = metadata_content {
			print_warn("Failed reading metadata file. Using the default values.");
			return Config::default()
		}

		print_info("Parsing the JSON metadata file...");
		
		let parsed_metadata = json::parse(&metadata_content.unwrap());

		if let Err(_) = parsed_metadata {
			print_warn("Parsing metadata failed. Using the default values.");
			return Config::default();
		}

		// Start parsing

		let metadata = parsed_metadata.unwrap();

		let mut config = Config::default();

		config.address = {
			if metadata["address"].is_null() {
				defaults::DEFAULT_ADDRESS.into()
			} else {
				metadata["address"].to_string()
			}
		};

		config.threads = {
			if metadata["thread"].is_null() {
				defaults::DEFAULT_THREADS
			} else {
				metadata["thread"].as_usize().unwrap()
			}
		};

		config.page_404_path = {
			if metadata["page_404_path"].is_null() {
				// Check if path exists
				if Path::new(defaults::DEFAULT_PAGE_404_PATH).exists() {
					defaults::DEFAULT_PAGE_404_PATH.into()
				} else {
					"".into()
				}
			} else {
				metadata["page_404_path"].to_string()
			}
		};

		if metadata["pages"].is_null() {
			print_warn(format!("'pages' array not found in the JSON metadata file. Using the default values."));
			return Config::default();
		}

		// Get all elements from pages
		let mut pgs_indx = 0;
		while !metadata["pages"][pgs_indx].is_null() {
			let current_page_config = metadata["pages"][pgs_indx].clone();

			let mut pg_metadata = PageMetadata {
				filename: current_page_config["filename"].to_string(),
				title: current_page_config["title"].to_string(),
				lang: current_page_config["lang"].to_string(),
				path: current_page_config["path"].to_string(),
				translations: vec![],
				styles: vec![]
			};

			// Get all elements from translations
			let mut trs_indx = 0;
			while !metadata["pages"][pgs_indx]["translations"][trs_indx].is_null() {
				let current_trs_config = metadata["pages"][pgs_indx]["translations"][trs_indx].clone();

				let pg_translation = PageTranslation {
					lang: current_trs_config["lang"].to_string(),
					path: current_trs_config["path"].to_string()
				};

				pg_metadata.translations.push(pg_translation);
				trs_indx += 1;
			}

			// Get all elements from styles
			let mut stl_indx = 0;
			while !metadata["pages"][pgs_indx]["styles"][stl_indx].is_null() {

				let pg_styles = metadata["pages"][pgs_indx]["styles"][stl_indx].clone().to_string();

				pg_metadata.styles.push(pg_styles);
				stl_indx += 1;
			}

			config.pages.push(pg_metadata);
			pgs_indx += 1;
		}

		return config;
	}

	// Get a PageMetadata instance by path. Return None if the path
	// Doesn't correspond to any instance.
	pub fn get_by_path(&self, file_path: &str) -> Option<PageMetadata> {
		for page in self.pages.iter() {
			if page.path == file_path {
				return Some(page.clone());
			}
		}

		return None
	}

	pub fn get_address(&self) -> String {
		self.address.clone()
	}

	pub fn get_threads(&self) -> usize {
		self.threads
	}

	pub fn get_verbosity(&self) -> u8 {
		self.verbosity
	}

	pub fn get_page_404_path(&self) -> String {
		self.page_404_path.clone()
	}
}

impl PageMetadata {
	pub fn get_filename(&self) -> String {
		self.filename.clone()
	}

	pub fn get_title(&self) -> String {
		self.title.clone()
	}

	pub fn get_lang(&self) -> String {
		self.lang.clone()
	}

	pub fn get_path(&self) -> String {
		self.path.clone()
	}

	pub fn get_translations(&self) -> Vec<PageTranslation> {
		self.translations.clone()
	}

	pub fn get_styles(&self) -> Vec<String> {
		self.styles.clone()
	}
}

impl PageTranslation {
	pub fn get_lang(&self) -> String {
		self.lang.clone()
	}

	pub fn get_path(&self) -> String {
		self.path.clone()
	}
}