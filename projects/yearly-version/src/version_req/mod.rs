use crate::{Version, VersionError};
use alloc::string::ToString;
use alloc::vec::Vec;
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
#[derive(Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct VersionRequest {
    pub constraints: Vec<VersionConstraint>,
}

#[repr(C, align(8))]
#[derive(Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct VersionConstraint {
    pub comparator: VersionComparator,
    pub year: Option<u32>,
    pub major: Option<u8>,
    pub minor: Option<u8>,
    pub patch: Option<u16>,
}

#[doc = include_str!("VersionComparator.md")]
#[repr(C, align(8))]
#[derive(Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum VersionComparator {
    Exact,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    /// patch version
    ///
    /// **~YEAR.MAJOR.MINOR**
    Tilde,
    /// compatible version
    ///
    /// **^YEAR.MAJOR.MINOR**
    Caret,
    #[default]
    Wildcard,
}

impl Display for VersionComparator {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Exact => { f.write_str("=") }
            Self::Greater => { f.write_str(">") }
            Self::GreaterEqual => { f.write_str(">=") }
            Self::Less => { f.write_str("<") }
            Self::LessEqual => { f.write_str("<=") }
            Self::Tilde => { f.write_str("~") }
            Self::Caret => { f.write_str("^") }
            Self::Wildcard => { f.write_str("*") }
        }
    }
}

impl Display for VersionRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        todo!()
    }
}
