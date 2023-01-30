use crate::VersionError;
use alloc::string::ToString;
use core::{
    fmt::{Display, Formatter},
    str::FromStr,
};


mod convert;
mod ser_der;


#[repr(C, align(8))]
#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Version {
    pub year: u32,
    pub major: u8,
    pub minor: u8,
    pub patch: u16,
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}.{}.{}.{}", self.year, self.major, self.minor, self.patch)
    }
}