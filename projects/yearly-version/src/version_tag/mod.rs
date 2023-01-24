use alloc::string::String;
use core::fmt::{Display, Formatter};
use crate::version::Version;

#[repr(C, align(8))]
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct VersionTag {
    pub version: Version,
    pub tag: String,
}

impl Display for VersionTag {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}-{}", self.version, self.tag)
    }
}