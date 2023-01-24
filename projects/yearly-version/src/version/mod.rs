
use alloc::string::String;
use core::fmt::{Display, Formatter};
pub use crate::errors::{Error, Result};

#[repr(C, align(8))]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Version {
    pub year: u32,
    pub major: u8,
    pub minor: u8,
    pub patch: u16,
}


impl Into<u64> for Version {
    fn into(self) -> u64 {
        (self.year as u64) << 48 + (self.major as u64) << 40 + (self.minor as u64) << 32 + self.patch as u64
    }
}

impl From<u64> for Version {
    fn from(value: u64) -> Self {
        Self {
            year: (value >> 48) as u32,
            major: (value >> 40) as u8,
            minor: (value >> 32) as u8,
            patch: value as u16,
        }
    }
}


impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}.{}.{}.{}", self.year, self.major, self.minor, self.patch)
    }
}