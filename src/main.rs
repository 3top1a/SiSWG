use std::borrow::Cow;
use std::error::Error;
use std::fs;
use std::path::Path;

use clap::{Arg, Command};
use terminal_size::terminal_size;

use path_absolutize::Absolutize;

use comrak::{markdown_to_html, ComrakOptions};

const APP_NAME: &str = "Markdown to HTML Converter";
const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const CARGO_PKG_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

mod config;

fn main() -> Result<(), Box<dyn Error>> {
	let matches = Command::new(APP_NAME)
        .term_width(terminal_size().map(|(width, _)| width.0 as usize).unwrap_or(0))
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

	let markdown_path = matches.value_of("MARKDOWN_PATH").unwrap();
	let html_path = matches.value_of("HTML_PATH");

	let force = matches.is_present("FORCE");

	let markdown_path = Path::new(markdown_path.trim());

	if markdown_path.is_dir() {
		return Err(format!(
			"`{}` is a directory!",
			markdown_path.absolutize()?.to_string_lossy()
		)
		.into());
	}

	let file_ext = markdown_path
		.extension()
		.map(|ext| ext.to_string_lossy())
		.unwrap_or_else(|| "".into());

	if !file_ext.eq_ignore_ascii_case("md")
		&& !file_ext.eq_ignore_ascii_case("markdown")
	{
		return Err(format!(
			"`{}` is not a Markdown file.",
			markdown_path.absolutize()?.to_string_lossy()
		)
		.into());
	}

	let file_stem = markdown_path
		.file_stem()
		.map(|ext| ext.to_string_lossy())
		.unwrap_or_else(|| "".into());

	let html_path = match html_path {
		Some(html_path) => Cow::from(Path::new(html_path.trim())),
		None => {
			let folder_path = markdown_path.parent().unwrap();

			Cow::from(folder_path.join(format!("{}.html", file_stem)))
		}
	};

	if let Ok(metadata) = html_path.metadata() {
		if metadata.is_dir() || !force {
			return Err(format!(
				"`{}` exists!",
				html_path.absolutize()?.to_string_lossy()
			)
			.into());
		}
	}

	// TODO Get title from files h1
	let title = String::from("Title");

	let markdown = fs::read_to_string(markdown_path)?;

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

		markdown_to_html(&markdown, &options)
	};

	/*let mut html_minifier = HTMLMinifier::new();

	html_minifier.digest("<!DOCTYPE html>")?;
	html_minifier.digest("<html>")?;

	html_minifier.digest("<head>")?;
	html_minifier.digest("<meta charset=UTF-8>")?;
	html_minifier.digest("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1, shrink-to-fit=no\">")?;
	html_minifier.digest(format!(
		"<meta name=\"generator\" content=\"{} {} by magiclen.org\"/>",
		APP_NAME, CARGO_PKG_VERSION,
	))?;
	html_minifier.digest("<title>")?;
	html_minifier.digest(html_escape::encode_text(&title).as_ref())?;
	html_minifier.digest("</title>")?;

	html_minifier.digest("<style>")?;
	html_minifier.digest(crate::config::CSS)?;
	html_minifier.digest("</style>")?;

	// TODO Mathjax svgs
	let has_mathjax = markdown_html.contains("#{{");
	if has_mathjax {
		html_minifier.digest("<script>")?;
		html_minifier.digest(crate::config::MATHJAX_CONFIG)?;
		html_minifier.digest("</script>")?;

		html_minifier.digest("<script>")?;
		html_minifier
			.digest(html_escape::encode_script(&crate::config::MATHJAX_SCRIPT).as_ref())?;
		html_minifier.digest("</script>")?;
	}

	html_minifier.digest("</head>")?;

	html_minifier.digest("<body>")?;

	html_minifier.digest("<article class=\"markdown-body\">")?;
	*/

	let html = crate::config::HTML.to_string();

	let html = html.replace("{title}", &html_escape::encode_text(&title).as_ref());
	// TODO desc
	let html = html.replace("{description}", &html_escape::encode_text(&title).as_ref());
	let html = html.replace("{content}", &markdown_html);

	let minified_html = minify_html::minify(html.as_bytes(), &minify_html::Cfg::spec_compliant());
	fs::write(html_path, minified_html)?;

	Ok(())
}
