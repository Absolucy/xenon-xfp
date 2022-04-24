//! Mounts are a concept seen throughout the Xenon File Protocol. They're important to using the protocol,
//! and are intended to be powerful and capable of allowing a single XFP server to do many different things,
//! while being simple to understand.
//!
//! To start, a mount has a name, which is how it is accessed throughout the API.
//! The name is a human-readable name for the mount, which must be url-safe. It can be validated with the `^[a-zA-Z0-9_-]*$` regex.
//!
//! The "backing store" behind a mount can be anything, there are no expectations for whatever is behind the mount itself.
//! A mount can store files on a normal disk, on cloud storage, a tape drive, a floppy disk, or even on a potato\*,
//! as XFP does not care which, as long as it is capable of file-based storage.
//!
//! A mount can also have authentication requirements. This is a way to restrict access to the mount,
//! and can be used to secure mounts against unauthorized access.
//!
//! The default mount is always named `default`, and any wrappers should interpret no mount as the default mount.
//!
//! ---
//!
//! \* as long as said potato is capable of file storage.
