use crate::{file::FileType, metadata::Metadata};
use serde::{Deserialize, Serialize};

pub type DirectoryListing = Vec<DirectoryListEntry>;

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct DirectoryListEntry {
	name: String,
	#[serde(rename = "type")]
	file_type: FileType,
	metadata: Metadata,
}

impl DirectoryListEntry {
	/// The name of this file object.
	///
	/// Represented as a string in JSON, with the key `name`.
	#[inline]
	pub fn name(&self) -> &str {
		&self.name
	}

	/// Duplicates this directory entry object,
	/// updating the name with the specified string.
	#[inline]
	pub fn with_name(self, name: impl ToString) -> Self {
		Self {
			name: name.to_string(),
			..self
		}
	}

	/// The type of this file object.
	///
	/// Represented as a string of either `file`, `directory`, or `symlink` in JSON, with the key `type`.
	#[inline]
	pub fn file_type(&self) -> FileType {
		self.file_type
	}

	/// Duplicates this directory entry object,
	/// updating the type with the specified value.
	#[inline]
	pub fn with_file_type(self, file_type: FileType) -> Self {
		Self { file_type, ..self }
	}

	/// The metadata of this file object.
	///
	/// Represented as an object in JSON, with the key `metadata`.
	#[inline]
	pub fn metadata(&self) -> &Metadata {
		&self.metadata
	}

	/// Duplicates this directory entry object,
	/// updating the metadata with the specified value.
	#[inline]
	pub fn with_metadata(self, metadata: Metadata) -> Self {
		Self { metadata, ..self }
	}
}
