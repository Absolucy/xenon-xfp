use serde::{Deserialize, Serialize};

/// The type of thing an object is.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FileType {
	/// This object is a file.
	///
	/// Represented at the string `file` in JSON.
	File,
	/// This object is a directory.
	///
	/// Represented at the string `directory` in JSON.
	Directory,
	/// This object is a symlink.
	///
	/// Represented at the string `symlink` in JSON.
	Symlink,
}

impl Default for FileType {
	#[inline]
	fn default() -> Self {
		FileType::File
	}
}
