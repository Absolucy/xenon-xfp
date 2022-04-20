use crate::util::SerializableDateTime;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct Metadata {
	created: Option<SerializableDateTime>,
	updated: Option<SerializableDateTime>,
	accessed: Option<SerializableDateTime>,
	mode: u32,
	uid: Option<u32>,
	gid: Option<u32>,
	size: u64,
}

impl Metadata {
	/// The time the file was created.
	#[inline]
	pub fn created(&self) -> Option<OffsetDateTime> {
		self.created.as_deref().copied()
	}

	/// Duplicates this metadata object,
	/// updating the created time with the specified time.
	#[inline]
	pub fn with_created(self, created: impl Into<Option<OffsetDateTime>>) -> Self {
		Self {
			created: created.into().map(SerializableDateTime::from),
			..self
		}
	}

	/// The time the file was last updated.
	#[inline]
	pub fn updated(&self) -> Option<OffsetDateTime> {
		self.updated.as_deref().copied()
	}

	/// Duplicates this metadata object,
	/// updating the updated time with the specified time.
	#[inline]
	pub fn with_updated(self, updated: impl Into<Option<OffsetDateTime>>) -> Self {
		Self {
			updated: updated.into().map(SerializableDateTime::from),
			..self
		}
	}

	/// The time the file was last accessed.
	///
	/// This may always be `None` on some systems,
	/// and as such, nothing should rely on this field's presence.
	#[inline]
	pub fn accessed(&self) -> Option<OffsetDateTime> {
		self.accessed.as_deref().copied()
	}

	/// Duplicates this metadata object,
	/// updating the accessed time with the specified time.
	#[inline]
	pub fn with_accessed(self, accessed: impl Into<Option<OffsetDateTime>>) -> Self {
		Self {
			accessed: accessed.into().map(SerializableDateTime::from),
			..self
		}
	}

	/// The file's permissions.
	#[inline]
	pub fn mode(&self) -> u32 {
		self.mode
	}

	/// Duplicates this metadata object,
	/// updating the mode with the specified value.
	#[inline]
	pub fn with_mode(self, mode: u32) -> Self {
		Self { mode, ..self }
	}

	/// The file's owner's user ID.
	///
	/// On non-Unix-like systems, this may always be `None`.
	#[inline]
	pub fn uid(&self) -> Option<u32> {
		self.uid
	}

	/// Duplicates this metadata object,
	/// updating the owner's user ID with the specified value.
	#[inline]
	pub fn with_uid(self, uid: impl Into<Option<u32>>) -> Self {
		Self {
			uid: uid.into(),
			..self
		}
	}

	/// The file's owner's group ID.
	///
	/// On non-Unix-like systems, this may always be `None`.
	#[inline]
	pub fn gid(&self) -> Option<u32> {
		self.gid
	}

	/// Duplicates this metadata object,
	/// updating the owner's group ID with the specified value.
	#[inline]
	pub fn with_gid(self, gid: impl Into<Option<u32>>) -> Self {
		Self {
			gid: gid.into(),
			..self
		}
	}

	/// The file's size in bytes.
	#[inline]
	pub fn size(&self) -> u64 {
		self.size
	}

	/// Duplicates this metadata object,
	/// updating the size with the specified value.
	#[inline]
	pub fn with_size(self, size: u64) -> Self {
		Self { size, ..self }
	}
}
