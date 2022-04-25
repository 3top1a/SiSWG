#[cfg(test)]
mod tests {
	#[test]
	fn test_get_file_stem() {
		assert_eq!(
			crate::utils::get_file_stem("/home/guest/index.html"),
			"index"
		);
		assert_eq!(
			crate::utils::get_file_stem(std::path::Path::new(
				"/home/guest/index.html"
			)),
			"index"
		);
	}
}
