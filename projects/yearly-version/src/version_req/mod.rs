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
    /// >1.0; <2.0
    pub constraints: Vec<VersionConstraint>,
}

#[repr(C, align(8))]
#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct VersionConstraint {
    pub comparator: VersionComparator,
    pub year: Option<u32>,
    pub major: Option<u8>,
    pub minor: Option<u8>,
    pub patch: Option<u16>,
}

#[doc = include_str!("VersionComparator.md")]
#[repr(C, align(8))]
#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum VersionComparator {
    /// ```text
    /// =YEAR
    /// =YEAR.MAJOR
    /// =YEAR.MAJOR.MINOR
    /// =YEAR.MAJOR.MINOR.PATCH
    /// ```
    Exact,
    /// ```text
    /// >YEAR
    /// >YEAR.MAJOR
    /// >YEAR.MAJOR.MINOR
    /// >YEAR.MAJOR.MINOR.PATCH
    /// ```
    Greater,
    /// ```text
    /// >=YEAR
    /// >=YEAR.MAJOR
    /// >=YEAR.MAJOR.MINOR
    /// >=YEAR.MAJOR.MINOR.PATCH
    /// ```
    GreaterEqual,
    /// ```text
    /// <YEAR
    /// <YEAR.MAJOR
    /// <YEAR.MAJOR.MINOR
    /// <YEAR.MAJOR.MINOR.PATCH
    /// ```
    Less,
    /// ```text
    /// <=YEAR
    /// <=YEAR.MAJOR
    /// <=YEAR.MAJOR.MINOR
    /// <=YEAR.MAJOR.MINOR.PATCH
    /// ```
    LessEqual,
    /// ```text
    /// ~YEAR
    /// ~YEAR.MAJOR
    /// ~YEAR.MAJOR.MINOR
    /// ~YEAR.MAJOR.MINOR.PATCH
    /// ```
    Tilde,
    /// ```text
    /// ^YEAR
    /// ^YEAR.MAJOR
    /// ^YEAR.MAJOR.MINOR.*
    /// ^YEAR.MAJOR.MINOR.PATCH
    /// ```
    Caret,
    /// ```text
    /// *
    /// YEAR.*
    /// YEAR.MAJOR.*
    /// YEAR.MAJOR.MINOR.*
    /// ```
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
