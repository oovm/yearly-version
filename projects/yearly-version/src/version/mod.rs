use crate::VersionError;
use alloc::string::ToString;
use core::{
    fmt::{Display, Formatter},
    str::FromStr,
};

mod convert;
#[cfg(feature = "schemars")]
mod json_schema;
#[cfg(feature = "serde")]
mod ser_der;

/// The yearly version number, encoding with a 64bit unsigned integer
#[repr(C, align(8))]
#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Version {
    /// Yearly version number, from 0 to 999999, indicating breaking updates
    pub year: u32,
    /// Major version number, from 0 to 255, indicating breaking updates
    pub major: u32,
    /// Minor version number, from 0 to 255, indicating compatibility updates
    pub minor: u32,
    /// Patch version number, from 0 to 65535, indicating compatibility updates
    pub patch: u32,
}

impl Version {
    /// The minimum version number
    pub const MIN: Version = Version { year: 0, major: 0, minor: 0, patch: 0 };
    /// The maximum version number
    pub const MAX: Version = Version { year: u32::MAX, major: u32::MAX, minor: u32::MAX, patch: u32::MAX };
    /// Create a new version number
    pub fn new(year: u32, major: u32, minor: u32, patch: u32) -> Self {
        Self { year, major, minor, patch }
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}.{}.{}.{}", self.year, self.major, self.minor, self.patch)
    }
}
