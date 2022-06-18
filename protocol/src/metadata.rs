mod permissions;

pub use self::permissions::{FileGroup, FileOwner, FilePermissions};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none, DisplayFromStr};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct Metadata {
	#[serde_as(as = "Option<Rfc3339>")]
	created: Option<OffsetDateTime>,
	#[serde_as(as = "Option<Rfc3339>")]
	updated: Option<OffsetDateTime>,
	#[serde_as(as = "Option<Rfc3339>")]
	accessed: Option<OffsetDateTime>,
	permissions: FilePermissions,
	#[serde_as(as = "DisplayFromStr")]
	size: u64,
}

impl Metadata {
	/// The time the file was created.
	///
	/// Represented as a string in RFC 3339 format in JSON, with the key `created`.
	#[inline]
	pub fn created(&self) -> Option<OffsetDateTime> {
		self.created
	}

	/// Duplicates this metadata object,
	/// updating the created time with the specified time.
	#[inline]
	pub fn with_created(self, created: impl Into<Option<OffsetDateTime>>) -> Self {
		Self {
			created: created.into(),
			..self
		}
	}

	/// The time the file was last updated.
	///
	/// Represented as a string in RFC 3339 format in JSON, with the key `updated`.
	#[inline]
	pub fn updated(&self) -> Option<OffsetDateTime> {
		self.updated
	}

	/// Duplicates this metadata object,
	/// updating the updated time with the specified time.
	#[inline]
	pub fn with_updated(self, updated: impl Into<Option<OffsetDateTime>>) -> Self {
		Self {
			updated: updated.into(),
			..self
		}
	}

	/// The time the file was last accessed.
	///
	/// This may always be `None` on some systems,
	/// and as such, nothing should rely on this field's presence.
	///
	/// Represented as a string in RFC 3339 format in JSON, with the key `accessed`.
	#[inline]
	pub fn accessed(&self) -> Option<OffsetDateTime> {
		self.accessed
	}

	/// Duplicates this metadata object,
	/// updating the accessed time with the specified time.
	#[inline]
	pub fn with_accessed(self, accessed: impl Into<Option<OffsetDateTime>>) -> Self {
		Self {
			accessed: accessed.into(),
			..self
		}
	}

	/// The permissions of this file.
	/// This is always present.
	///
	/// Represented as an object in JSON, with the key `permissions`.
	#[inline]
	pub fn permissions(&self) -> &FilePermissions {
		&self.permissions
	}

	/// Duplicates this metadata object,
	/// updating the permissions with the specified permissions.
	#[inline]
	pub fn with_permissions(self, permissions: FilePermissions) -> Self {
		Self {
			permissions,
			..self
		}
	}

	/// The file's size in bytes.
	///
	/// Represented as a number in JSON, with the key `size`.
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
