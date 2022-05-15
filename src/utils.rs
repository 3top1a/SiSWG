use std::path::{Path, PathBuf};
extern crate serde_derive;
extern crate toml;

pub fn get_file_stem<P: AsRef<Path>>(path: P) -> String {
	path.as_ref()
		.file_stem()
		.map(|ext| ext.to_os_string())
		.unwrap_or_else(|| "".into())
		.into_string()
		.unwrap()
}

pub fn get_title_from_text<P: AsRef<Path>>(
	input_text: &String,
	path: P,
) -> String {
	let mut lines = input_text.lines();

	let r = if (lines.clone().count() > 0) && (input_text.starts_with("# ")) {
		let first: &str = lines.next().unwrap();
		first.split_at(2).1.to_string()
	} else {
		crate::utils::get_file_stem(&path)
	};

	// Do some pp
	r.trim().to_string()
}

pub fn get_description_from_text(input_text: &String) -> String {
	let mut lines = input_text.lines();

	if lines.clone().count() <= 2 {
		return String::from(
			"Looks like I forgot to complete this file. Oh well.",
		);
	}

	let first_line = lines.next().unwrap_or("");
	let sec_line = lines.next().unwrap_or("");

	let res = if first_line.starts_with("## ") {
		first_line.split_at(3).1.to_owned()
	} else if sec_line.starts_with("## ") {
		sec_line.split_at(3).1.to_owned()
	} else {
		String::from("Description not found, so here's a secret on how to make the perfect hot cocoa: add vanilla sugar and some salt")
	};

	// Do some pp
	res.trim().to_string()
}

#[derive(serde_derive::Deserialize, PartialEq, Debug)]
pub struct Metadata {
	pub title: Option<String>,
	pub description: Option<String>,
}

pub fn extract_meta_from_toml<P: AsRef<Path>>(toml_path: P) -> Metadata {
	//toml::from_str();
	let toml: String =
		std::fs::read_to_string(toml_path).unwrap_or("".to_owned());
	let toml_p: Metadata = toml::from_str(&toml).unwrap();

	toml_p
}

pub fn generate_txt_sitemap(map: Vec<PathBuf>) -> String {
	map.iter()
		.map(|f| f.clone().into_os_string().into_string().unwrap())
		.collect::<Vec<String>>()
		.join("\n")
}
