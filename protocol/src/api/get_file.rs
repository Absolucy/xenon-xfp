//! # GET `/:mount/file/:path`
//!
//! This is the simplest function in xfp API - it just downloads a file from
//! the specified path in the given mount.
//!
//! The HTTP `Range` header can be used to download a partial file.
//!
//! ## Return Codes
//!
//! ### OK
//!  - `200 OK`: The file was successfully retrieved.
//!  - `204 No Content`: The file was successfully retrieved, but it was empty.
//!  - `206 Partial Content`: The part of the file specified in the `Range` header was successfully retrieved.
//!
//! ### Errors
//! All errors will return a JSON error object, explaining what went wrong in more detail than just the status code.
//!
//! Any server which just returns the JSON without the proper status code, or vice versa,
//! is considered **not** compliant with the protocol.
//!
//!  - `401 Unauthorized` - A proper `Authorization` header is required to access this mount or file.
//!  - `403 Forbidden` - The requested mount or file is not accessible by the current authorized user.
//!  - `404 Not Found` - The requested mount or file was not found.
//!  - `413 Request Entity Too Large` - If the server has any size limits,
//!    then this will be returned if the file is too large.
//!  - `416 Range Not Satisfiable` - The requested range was not satisfiable. Perhaps it was longer than the file.
//!  - `500 Internal Server Error` - The server encountered an error while processing the request.
//!    A more detailed error should be in the `description` field of the JSON response.
