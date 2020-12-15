use comrak::{ComrakOptions, ComrakExtensionOptions, ComrakParseOptions, ComrakRenderOptions};
use grass::{Options, OutputStyle};

// DO NOT write the "WWW/style/" part
pub static DEFAULT_MD_STYLE: &str = "markdown.scss";

pub static WWW: &str = "www/";

pub static DEFAULT_ADDRESS: &str = "127.0.0.1:80";
pub static DEFAULT_THREADS: usize = 4;
pub static DEFAULT_VERB: u8 = 2;
pub static DEFAULT_PAGE_404_PATH: &str = "404.txt";
pub static DEFAULT_404_PAGE_CONTENT: &str = "ERROR 404: Not found.";
pub static DEFAULT_META_PATH: &str = "meta.json";

lazy_static!{
	// Directories always end with "/"
	// Put directories before files stored in them
	pub static ref BASE: [String; 10] = [
		format!("{}", WWW),
		format!("{}index.md", WWW),
		format!("{}meta.json", WWW),
		format!("{}icon/", WWW),
		format!("{}icon/favicon.ico", WWW),
		format!("{}en/", WWW),
		format!("{}en/sample.md", WWW),
		format!("{}style/", WWW),
		format!("{}style/markdown.scss", WWW),
		format!("{}style/font/", WWW)
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