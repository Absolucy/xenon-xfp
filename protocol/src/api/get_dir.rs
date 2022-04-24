//! # GET `/:mount/dir/:path`
//!
//! This is simple - it returns the contents of the given directory in the given mount,
//! represented as a JSON array of [`DirectoryListEntry`](crate::dir::DirectoryListEntry) objects.
//!
//! ## Return Codes
//!
//! ### OK
//!  - `200 OK`: The directory listing was successfully retrieved.
//!  - `204 No Content`: The directory listing was successfully retrieved, but the directory was empty.
//!
//! ### Errors
//! All errors will return a JSON error object, explaining what went wrong in more detail than just the status code.
//!
//! Any server which just returns the JSON without the proper status code, or vice versa,
//! is **non-compliant** with the protocol.
//!
//!  - `401 Unauthorized` - A proper `Authorization` header is required to access this mount or directory.
//!  - `403 Forbidden` - The requested mount or directory is not accessible by the current authorized user.
//!  - `404 Not Found` - The requested mount or directory was not found.
//!  - `500 Internal Server Error` - The server encountered an error while processing the request.
//!    A more detailed error should be in the `description` field of the JSON response.
//!
//! ## Response
//!
//! The response is a JSON array, with each object representing a file or directory in the directory.
//! The order of the array is the same as they are read by the server.
//!
//! ### Directory Entry
//!
//! The directory entry is a JSON object, with the following basic fields:
//!
//! - [`name`](crate::dir::DirectoryListEntry#method.name): The name of the file or directory.
//! - [`type`](crate::dir::DirectoryListEntry#method.file_type): The type of the file or directory. This can either be `"file"`, `"directory"`, or `"symlink"`. See [`FileType`](crate::file::FileType).
//! - [`metadata`](crate::dir::DirectoryListEntry#method.metadata): The metadata for the file or directory, represented as a JSON object. See [`Metadata`](crate::metadata::Metadata).
