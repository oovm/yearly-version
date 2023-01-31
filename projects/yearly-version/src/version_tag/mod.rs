use crate::version::Version;
use alloc::string::String;
use core::{
    cmp::Ordering,
    fmt::{Display, Formatter},
    hash::{Hash, Hasher},
    str::FromStr,
};

mod convert;
#[cfg(feature = "serde")]
mod ser_der;

#[repr(C, align(8))]
#[derive(Clone, Debug)]
pub struct VersionTag {
    /// The 64bit version number
    pub number: Version,
    /// The additional tagged information
    ///
    /// This field does not affect the determination of the version number
    pub tag: String,
}

impl Eq for VersionTag {}

impl PartialEq<Self> for VersionTag {
    fn eq(&self, other: &Self) -> bool {
        self.number.eq(&other.number)
    }
}

impl PartialOrd<Self> for VersionTag {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.number.partial_cmp(&other.number)
    }
}

impl Ord for VersionTag {
    fn cmp(&self, other: &Self) -> Ordering {
        self.number.cmp(&other.number)
    }
}
impl Hash for VersionTag {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.number.hash(state)
    }
}

impl Display for VersionTag {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}-{}", self.number, self.tag)
    }
}
