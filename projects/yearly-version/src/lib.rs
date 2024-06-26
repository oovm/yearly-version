#![no_std]
#![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

extern crate alloc;

mod errors;
mod version;
mod version_tag;

pub use crate::errors::{VersionError, Result};
pub use crate::version::Version;
pub use crate::version_tag::VersionTag;

#[cfg(feature = "semver")]
pub use semver::Version as SemVer;