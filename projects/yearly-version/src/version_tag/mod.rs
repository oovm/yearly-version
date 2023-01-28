use alloc::string::String;
use core::fmt::{Display, Formatter};
use crate::version::Version;

mod convert;

#[repr(C, align(8))]
#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct VersionTag {
    pub number: Version,
    pub tag: String,
}

impl Display for VersionTag {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}-{}", self.number, self.tag)
    }
}