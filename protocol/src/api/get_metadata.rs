//! # GET `/:mount/metadata/:path`
//!
//! This is simple - it returns the metadata for the given path in the given mount,
//! represented as a JSON object.
//!
//! ## Return Codes
//!
//! ### OK
//!  - `200 OK`: The file metadata was successfully retrieved, and returned as a [`Metadata`](crate::metadata::Metadata) object.
//!
//! ### Errors
//! All errors will return a JSON error object, explaining what went wrong in more detail than just the status code.
//!
//! Any server which just returns the JSON without the proper status code, or vice versa,
//! is **non-compliant** with the protocol.
//!
//!  - `401 Unauthorized` - A proper `Authorization` header is required to access this mount or file.
//!  - `403 Forbidden` - The requested mount or file is not accessible by the current authorized user.
//!  - `404 Not Found` - The requested mount or file was not found.
//!  - `500 Internal Server Error` - The server encountered an error while processing the request.
//!    A more detailed error should be in the `description` field of the JSON response.
//!
//! ## Response
//!
//! ### Metadata
//!
//! The metadata is a JSON object, with the following basic fields:
//!
//! - [`created`](crate::metadata::Metadata#method.created): The time the file was created, represented as a RFC3339-formatted [date-time](time::OffsetDateTime). Optional.
//! - [`updated`](crate::metadata::Metadata#method.updated): The time the file was last updated, represented as a RFC3339-formatted [date-time](time::OffsetDateTime). Optional.
//! - [`accessed`](crate::metadata::Metadata#method.accessed): The time the file was last accessed, represented as a RFC3339-formatted [date-time](time::OffsetDateTime). Optional.
//! - [`permissions`](crate::metadata::Metadata#method.permissions): The permissions for the file, represented as a JSON object. See [`Permissions`](crate::metadata::FilePermissions).
//! - [`size`](crate::metadata::Metadata#method.size): The size of the file, in bytes.
//!
//! ### Permissions
//! The permissions are a JSON object, with the following fields:
//!
//! - [`read`](crate::metadata::FilePermissions#method.read): Whether the file is readable by the server.
//! - [`write`](crate::metadata::FilePermissions#method.write): Whether the file is writable by the server.
//! - [`execute`](crate::metadata::FilePermissions#method.execute): Whether the file is executable by the server.
//! - [`owner`](crate::metadata::FilePermissions#method.owner): The owner of the file, represented as a JSON object. See [`FileOwner`](crate::metadata::FileOwner). Optional.
//!   - **Note**: The contents of field differs based on if the server is running on Windows or a Unix-like platform.
//!   - On Windows, it contains the `sid` and `domain` fields. On Unix-like platforms, it contains a `uid` field. Both platforms have a `name` field, which is the textual name of the user.
//! - [`group`](crate::metadata::FilePermissions#method.group): The group of the file, represented as a JSON object. See [`FileGroup`](crate::metadata::FileGroup). Optional.
//!   - **Note**: The contents of field differs based on if the server is running on Windows or a Unix-like platform.
//!   - On Windows, it contains the `sid` and `domain` fields. On Unix-like platforms, it contains a `gid` field. Both platforms have a `name` field, which is the textual name of the group.
