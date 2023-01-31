use alloc::string::String;
use core::fmt::{Display, Formatter};

/// The result type for parsing a version number
pub type Result<T> = core::result::Result<T, VersionError>;

/// The error type for parsing a version number
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum VersionError {
    /// The version number is missing a part
    MissingPart {
        /// The part that is missing
        part: String,
        /// The offset of the missing part
        offset: usize,
    },
    /// The version number has an invalid part
    InvalidPart {
        /// The part that is invalid
        part: String,
        /// The start of the invalid part
        start: usize,
        /// The end of the invalid part
        end: usize,
    },
    /// The version number has an extra part
    ExtraPart {
        /// The extra part
        extra: String,
        /// The offset of the extra part
        offset: usize,
    },
}

impl Display for VersionError {
    fn fmt(&self, _: &mut Formatter<'_>) -> core::fmt::Result {
        todo!()
    }
}
