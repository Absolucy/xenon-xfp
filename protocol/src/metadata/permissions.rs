use faccess::PathExt;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::{io, path::Path};

#[cfg(unix)]
use nix::unistd::{Gid, Group, Uid, User};
#[cfg(unix)]
use std::os::unix::fs::MetadataExt;

#[cfg(windows)]
use windows_permissions::{
	constants::{SeObjectType, SecurityInformation},
	wrappers::{GetNamedSecurityInfo, LookupAccountSid},
};

/// Describes the user that owns a file, including their unique identifier
/// and visible name.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
#[serde(untagged)]
pub enum FileOwner {
	Unix {
		/// The user's unique identifier.
		uid: u32,
		#[serde(default, skip_serializing_if = "String::is_empty")]
		/// The user's visible name.
		name: String,
	},
	Windows {
		/// The security identifier (SID) of the user.
		sid: String,
		/// The user's domain name.
		#[serde(default, skip_serializing_if = "String::is_empty")]
		domain: String,
		/// The user's visible name.
		#[serde(default, skip_serializing_if = "String::is_empty")]
		name: String,
	},
}

/// Describes the group that owns a file, including their unique identifier
/// and visible name.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
#[serde(untagged)]
pub enum FileGroup {
	Unix {
		/// The group's unique identifier.
		gid: u32,
		/// The group's visible name.
		#[serde(default, skip_serializing_if = "String::is_empty")]
		name: String,
	},
	Windows {
		/// The security identifier (SID) of the group.
		sid: String,
		#[serde(default, skip_serializing_if = "String::is_empty")]
		/// The group's domain.
		domain: String,
		/// The group's visible name.
		#[serde(default, skip_serializing_if = "String::is_empty")]
		name: String,
	},
}

#[skip_serializing_none]
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
#[serde(rename_all = "kebab-case")]
pub struct FilePermissions {
	read: bool,
	write: bool,
	execute: bool,
	owner: Option<FileOwner>,
	group: Option<FileGroup>,
}

impl FilePermissions {
	#[cfg(unix)]
	pub fn from_path<P: AsRef<Path>>(path: P) -> io::Result<Self> {
		let path = path.as_ref();
		let metadata = path.metadata()?;
		let uid = metadata.uid();
		let owner = User::from_uid(Uid::from_raw(uid))
			.ok()
			.flatten()
			.map(|user| FileOwner::Unix {
				uid: user.uid.as_raw(),
				name: user.name,
			});
		let group = Group::from_gid(Gid::from_raw(metadata.gid()))
			.ok()
			.flatten()
			.map(|group| FileGroup::Unix {
				gid: group.gid.as_raw(),
				name: group.name,
			});
		Ok(Self {
			read: path.readable(),
			write: path.writable(),
			execute: path.executable(),
			owner,
			group,
		})
	}

	#[cfg(windows)]
	pub fn from_path<P: AsRef<Path>>(path: P) -> io::Result<Self> {
		let path = path.as_ref();
		let info = GetNamedSecurityInfo(
			path,
			SeObjectType::SE_FILE_OBJECT,
			SecurityInformation::Group | SecurityInformation::Owner,
		)?;
		let owner = info.owner().map(|sid| {
			let (domain, name) = LookupAccountSid(sid).unwrap_or_default();
			FileOwner::Windows {
				sid: sid.to_string(),
				domain: domain
					.into_string()
					.unwrap_or_else(|osstr| osstr.to_string_lossy().into_owned()),
				name: name
					.into_string()
					.unwrap_or_else(|osstr| osstr.to_string_lossy().into_owned()),
			}
		});
		let group = info.group().map(|sid| {
			let (domain, name) = LookupAccountSid(sid).unwrap_or_default();
			FileGroup::Windows {
				sid: sid.to_string(),
				domain: domain
					.into_string()
					.unwrap_or_else(|osstr| osstr.to_string_lossy().into_owned()),
				name: name
					.into_string()
					.unwrap_or_else(|osstr| osstr.to_string_lossy().into_owned()),
			}
		});
		Ok(Self {
			read: path.readable(),
			write: path.writable(),
			execute: path.executable(),
			owner,
			group,
		})
	}

	/// Returns if this file is readable.
	///
	/// Represented as a bool in JSON, with the key `read`.
	#[inline]
	pub fn read(&self) -> bool {
		self.read
	}

	/// Duplicates this permissions object,
	/// setting the read flag to `read`.
	#[inline]
	pub fn with_read(self, read: bool) -> Self {
		Self { read, ..self }
	}

	/// Returns if this file is writable.
	/// This is only true if the file is not read-only.
	///
	/// Represented as a bool in JSON, with the key `write`.
	#[inline]
	pub fn write(&self) -> bool {
		self.write
	}

	/// Duplicates this permissions object,
	/// setting the write flag to `write`.
	#[inline]
	pub fn with_write(self, write: bool) -> Self {
		Self { write, ..self }
	}

	/// Returns if this file is executable.
	/// This is only true if the file is not read-only.
	///
	/// Represented as a bool in JSON, with the key `execute`.
	#[inline]
	pub fn execute(&self) -> bool {
		self.execute
	}

	/// Duplicates this permissions object,
	/// setting the execute flag to `execute`.
	#[inline]
	pub fn with_execute(self, execute: bool) -> Self {
		Self { execute, ..self }
	}

	/// Returns the owner of this file, if any.
	///
	/// Represented as an object in JSON, with the key `owner`.
	#[inline]
	pub fn owner(&self) -> Option<&FileOwner> {
		self.owner.as_ref()
	}

	/// Duplicates this permissions object,
	/// setting the owner to `owner`.
	#[inline]
	pub fn with_owner(self, owner: impl Into<Option<FileOwner>>) -> Self {
		Self {
			owner: owner.into(),
			..self
		}
	}

	/// Returns the group of this file, if any.
	///
	/// Represented as an object in JSON, with the key `group`.
	#[inline]
	pub fn group(&self) -> Option<&FileGroup> {
		self.group.as_ref()
	}

	/// Duplicates this permissions object,
	/// setting the group to `group`.
	#[inline]
	pub fn with_group(self, group: impl Into<Option<FileGroup>>) -> Self {
		Self {
			group: group.into(),
			..self
		}
	}
}
