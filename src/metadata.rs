use json;

use crate::io_ops::get_file_content_string;
use crate::printing::*;

#[derive(Clone, Debug, Default)]
pub struct Config {
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

impl Config {
	pub fn parse_metadata(file_path: &str) -> Self {
		print_info("Reading the JSON metadata file ({})...");

		let metadata_content = get_file_content_string(file_path.into(), None);

		if let Err(_) = metadata_content {
			print_warn("Failed reading metadata. Using the default values.");
			return Config::default()
		}

		print_info("Parsing the JSON metadata file...");
		
		let parsed_metadata = json::parse(&metadata_content.unwrap());

		if let Err(_) = parsed_metadata {
			print_warn("Parsing metadata failed. Using the default values.");
			return Config::default();
		}

		let metadata = parsed_metadata.unwrap();

		if metadata["pages"].is_null() {
			print_warn(&format!("'pages' array not found in the JSON metadata file. Using the default values."));
			return Config::default();
		}

		let mut config = Config { pages: vec![] };

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