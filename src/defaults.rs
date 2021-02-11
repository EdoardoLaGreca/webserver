use comrak::{ComrakOptions, ComrakExtensionOptions, ComrakParseOptions, ComrakRenderOptions};
use grass::{Options, OutputStyle};

// DO NOT write the "WWW/style/" part
pub static DEFAULT_MD_STYLE: &str = "default.scss";

pub static WWW: &str = "www/";

// DO NOT write the "WWW/" part
pub static DEFAULT_ADDRESS: &str = "127.0.0.1:80";
pub static DEFAULT_THREADS: usize = 4;
pub static DEFAULT_VERB: u8 = 2;
pub static DEFAULT_PAGE_404_PATH: &str = "404.md";
pub static DEFAULT_404_PAGE_CONTENT: &str = "ERROR 404: Not found.";
pub static DEFAULT_CONFIG_PATH: &str = "config.toml";

lazy_static!{
	// Directories always end with "/", otherwise they will be recognized as files
	// Specify a directory only if it needs to be empty
	pub static ref BASE: [String; 5] = [
		"config.toml".into(),
		format!("{}index.md", WWW),
		format!("{}favicon.ico", WWW),
		format!("{}style/default.scss", WWW),
		format!("{}style/font/", WWW),
	];

	// See https://docs.rs/comrak/0.9.0/comrak/struct.ComrakOptions.html
	pub static ref COMRAK_OPTIONS: ComrakOptions = {
		ComrakOptions {
			extension: ComrakExtensionOptions {
				strikethrough: true,
				tagfilter: false,
				table: true,
				autolink: true,
				tasklist: true,
				superscript: true,
				header_ids: None,
				footnotes: true,
				description_lists: false
			},
			parse: ComrakParseOptions {
				smart: true,
				default_info_string: None
			},
			render: ComrakRenderOptions {
				hardbreaks: true,
				github_pre_lang: true,
				width: 80,
				unsafe_: false,
				escape: false
			}
		}
	};
}

pub fn get_default_grass_options() -> Options<'static> {
	Options::default()
		.style(OutputStyle::Compressed)
		.quiet(false)
}