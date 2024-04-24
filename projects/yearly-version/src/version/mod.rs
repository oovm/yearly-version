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

#[repr(C, align(8))]
#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Version {
    pub year: u32,
    pub major: u8,
    pub minor: u8,
    pub patch: u16,
}

impl Version {
    pub const MIN: Version = Version { year: 0, major: 0, minor: 0, patch: 0 };
    pub const MAX: Version = Version { year: u32::MAX, major: u8::MAX, minor: u8::MAX, patch: u16::MAX };
    pub fn new(year: u32, major: u8, minor: u8, patch: u16) -> Self {
        Self { year, major, minor, patch }
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}.{}.{}.{}", self.year, self.major, self.minor, self.patch)
    }
}
