use std::error::Error;
use std::fs;
use std::path::Path;

use clap::{Arg, Command};

use comrak::{markdown_to_html, ComrakOptions};

const APP_NAME: &str = "Markdown to HTML Converter";
const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const CARGO_PKG_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

mod config;
mod tests;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
	println!("--- SiSWG ---");

	let matches = Command::new(APP_NAME)
		.version(CARGO_PKG_VERSION)
		.author(CARGO_PKG_AUTHORS)
		.about("A tool to convert markdown files to HTML for my website \n\nExamples:
		\nsiswg file.md
		\n	# Convert file.md to file.html, titled \"file\"
		\nsiswg file.md -o output.html
		\n	# Convert file.md to output.html, titled \"output\"
		\n")
		.arg(Arg::new("MARKDOWN_PATH")
			.required(true)
			.help("Specify the path of your Markdown file")
			.takes_value(true)
		)
		.arg(Arg::new("HTML_PATH")
			.required(false)
			.long("html-path")
			.short('o')
			.help("Specify the path of your HTML file")
			.takes_value(true)
			.display_order(2)
		)
		.arg(Arg::new("FORCE")
			.long("force")
			.short('f')
			.help("Force to output if the HTML file exists")
		)
		.after_help("Enjoy it! https://3top1a.github.io/")
		.get_matches();

	// Parse
	let markdown_path =
		Path::new(matches.value_of("MARKDOWN_PATH").unwrap().trim());
	let html_path = Path::new(matches.value_of("HTML_PATH").unwrap().trim());
	let force = matches.is_present("FORCE");

	// Directory mode
	// Checks if out is a folder
	let directory_mode = Path::new(markdown_path).is_dir();

	// Get all files in one big list
	// jesus
	let files = if markdown_path.is_dir() {
		let mut x = Vec::new();

		let paths = fs::read_dir(markdown_path).unwrap();

		for path in paths {
			if path.as_ref().unwrap().path().is_dir() {
				continue;
			}
			if path
				.as_ref()
				.unwrap()
				.path()
				.extension()
				.unwrap_or(std::ffi::OsStr::new(""))
				!= "md"
			{
				continue;
			}
			x.push(path.unwrap().path().display().to_string());
		}

		if x.len() == 0 {
			return Err(format!(
				"No files found in {}",
				markdown_path.display()
			)
			.into());
		}

		if !html_path.is_dir() {
			return Err(format!(
				"Output {} is not a directory!",
				html_path.display()
			)
			.into());
		}

		x
	} else {
		let mut x = Vec::new();

		if markdown_path.extension().unwrap() != "md" {
			return Err(format!(
				"No files found in {}",
				markdown_path.display()
			)
			.into());
		}

		x.push(markdown_path.display().to_string());
		x
	};

	for file in files {
		// If in dir mode, generate output dir
		let path: std::path::PathBuf = if directory_mode {
			html_path
				.canonicalize()
				.unwrap()
				.join(crate::utils::get_file_stem(file.as_str()) + ".html")
		} else {
			html_path.to_path_buf()
		};

		println!("Converting {}", crate::utils::get_file_stem(&path));

		convert_file(Path::new(&file), path.as_path(), force).unwrap();
	}

	println!("Done converting!");
	Ok(())
}

fn convert_file(
	markdown_path: &Path,
	path: &Path,
	force: bool,
) -> std::result::Result<(), std::io::Error> {
	if let Ok(metadata) = path.metadata() {
		if metadata.is_dir() || !force {
			return Err(std::io::Error::new(
				std::io::ErrorKind::AlreadyExists,
				format!("File {} already exists!", path.display()),
			));
		}
	}

	// Read text
	let markdown = fs::read_to_string(markdown_path);

	// Metadata
	let meta = crate::utils::extract_meta_from_toml(
		markdown_path.with_extension("toml"),
	);

	// Try to use the provided metadata from 'x'.toml
	let title = meta.title.unwrap_or(crate::utils::get_title_from_text(
		&markdown.as_ref().unwrap(),
		path,
	));
	let title: String = if title == crate::config::NAME {
		title
	} else {
		crate::config::NAME.to_owned() + &" - ".to_owned() + &title
	};
	let description = meta.description.unwrap_or(
		crate::utils::get_description_from_text(&markdown.as_ref().unwrap()),
	);

	// Parser options
	let markdown_html = {
		let mut options = ComrakOptions::default();

		options.render.unsafe_ = true;
		options.extension.autolink = true;
		options.extension.description_lists = true;
		options.extension.footnotes = true;
		options.extension.strikethrough = true;
		options.extension.superscript = true;
		options.extension.table = true;
		options.extension.tagfilter = true;
		options.extension.tasklist = true;
		options.render.hardbreaks = true;

		markdown_to_html(&markdown.as_ref().unwrap(), &options)
	};

	let html = crate::config::HTML.to_string();
	let html =
		html.replace("{title}", &html_escape::encode_text(&title).as_ref());

	let html = html.replace(
		"{description}",
		&html_escape::encode_text(&description).as_ref(),
	);
	let html = html.replace("{content}", &markdown_html);
	let html = html.replace("{style}", crate::config::CSS);

	let minified_html = minify_html::minify(
		html.as_bytes(),
		&minify_html::Cfg::spec_compliant(),
	);
	fs::write(&path, minified_html)
}
