use crate::Result;
use simple_fs::SFile;
use std::path::Path;

/// Returns the file that matches the path for a given list of directories.
/// This is useful for finding a file path with some directory precedence rules.
pub fn first_file_from_dirs(dirs: &[&str], path: &str) -> Result<Option<SFile>> {
	for dir in dirs {
		let file_path = Path::new(dir).join(path);
		if file_path.exists() {
			return Ok(Some(SFile::from_path(file_path)?));
		}
	}

	Ok(None)
}
