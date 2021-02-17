use comrak::{ComrakOptions, ComrakExtensionOptions, ComrakParseOptions, ComrakRenderOptions};
use grass::{Options, OutputStyle};

// NOTE: When specifying paths, you assume that the root path is WWW, except if stated otherwise.

// DO NOT write the "WWW/style/" part
pub const DEFAULT_MD_STYLE: &str = "default.scss";

pub const WWW: &str = "www/";

pub const DEFAULT_ADDRESS: &str = "127.0.0.1:80";
pub const DEFAULT_THREADS: usize = 4;
pub const DEFAULT_VERB: u8 = 2;
pub const DEFAULT_PAGE_404_PATH: &str = "404.md";
pub const DEFAULT_404_PAGE_CONTENT: &str = "ERROR 404: Not found.";

// Here the root path is the repo root
pub const DEFAULT_CONFIG_PATH: &str = "config.toml";

lazy_static!{
	// See https://docs.rs/comrak/latest/comrak/struct.ComrakOptions.html
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
				description_lists: false,
				front_matter_delimiter: None
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