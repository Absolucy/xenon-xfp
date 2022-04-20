use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};
use time::{serde::rfc3339, OffsetDateTime};

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
pub(crate) struct SerializableDateTime(#[serde(with = "rfc3339")] OffsetDateTime);

impl Deref for SerializableDateTime {
	type Target = OffsetDateTime;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl From<OffsetDateTime> for SerializableDateTime {
	fn from(date: OffsetDateTime) -> Self {
		SerializableDateTime(date)
	}
}