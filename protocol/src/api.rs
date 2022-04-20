//! # xfp
//!
//! The Xenon File Protocol is a protocol for file storage and downloading,
//! intended to be powerful and capable of handling many situations, while
//! being relatively easy to implement and well-documented.
//!
//! xfp is a RESTy protocol based on HTTP and JSON.
//! Metadata is returned in an easy to understand JSON format,
//! and files are downloaded just like any other file that would be downloaded over HTTP.
//!
//! In many ways, xfp is remniscent of WebDAV, but with a few key differences:
//!   - xfp uses a RESTy JSON-based protocol, while WebDAV uses an XML-based protocol.
//!   - xfp allows for "mounting" different folders, each with their own authentication requirements.
//!   - xfp does not have special handling for things such as calenders (CalDAV) or contacts (CardDAV).

/// GET `/:mount/dir/:path`
pub mod get_dir;
/// GET `/:mount/file/:path`
pub mod get_file;
/// GET `/:mount/metadata/:path`
pub mod get_metadata;
