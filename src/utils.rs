
pub fn get_file_stem<P: AsRef<std::path::Path>>(path: P) -> String
{
	path.as_ref()
		.file_stem()
		.map(|ext| ext.to_os_string())
		.unwrap_or_else(|| "".into()).into_string().unwrap()
}
