#[cfg(test)]
mod tests {
	use std::path::Path;
	#[test]
	fn test_get_file_stem() {
		assert_eq!(
			crate::utils::get_file_stem("/home/guest/index.html"),
			"index"
		);
		assert_eq!(
			crate::utils::get_file_stem(Path::new("/home/guest/index.html")),
			"index"
		);
	}

	#[test]
	fn test_get_title_from_text() {
		let text = "# Example title
## This is some text

This is also text
";

		assert_eq!(
			crate::utils::get_title_from_text(
				&String::from(text),
				Path::new("Example path.md")
			),
			"Example title"
		);
	}

	#[test]
	fn test_get_title_from_path() {
		let text = "foo
bar

baz";

		assert_eq!(
			crate::utils::get_title_from_text(
				&String::from(text),
				Path::new("Example path.md")
			),
			"Example path"
		);
	}

	#[test]
	fn get_description_from_text()
	{
		let text = "# Example title
## Example description

some light text
";

		assert_eq!(
			crate::utils::get_description_from_text(
				&String::from(text),
			),
			"Example description"
		);


		assert_ne!(
			crate::utils::get_description_from_text(
				&String::from("test text"),
			),
			"Example description"
		);
	}
}
