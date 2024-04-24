use crate::{Version, VersionError};
use alloc::{string::ToString, vec::Vec};
use core::{
    fmt::{Display, Formatter},
    str::FromStr,
};
use core::ops::{Range, RangeInclusive};
use semver::Op;

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

impl VersionConstraint {
    pub fn upper(&self) -> Version {
        self.comparator.range(self.year, self.major, self.minor, self.patch)
    }
    pub fn lower(&self) -> Version {
        self.comparator.lower(self.year, self.major, self.minor, self.patch)
    }
    pub fn unlimited(&self) -> bool {
        self.year.is_none() && self.major.is_none() && self.minor.is_none() && self.patch.is_none()
    }
}

impl VersionComparator {
    pub fn range(&self, year: Option<u32>, major: Option<u8>, minor: Option<u8>, patch: Option<u16>) -> RangeInclusive<Version> {
        match self {
            Self::Exact | Self::Wildcard => {
                match year {
                    Some(year) => {
                        match major {
                            Some(major) => {
                                match minor {
                                    Some(minor) => {
                                        match patch {
                                            // 1.2.3.4 -> [1.2.3.4, 1.2.3.4]
                                            Some(patch) => Version::new(year, major, minor, patch)..=Version::new(year, major, minor, patch),
                                            // 1.2.3.* -> [1.2.3.0, 1.2.3.MAX]
                                            None => Version::new(year, major, minor, 0)..=Version::new(year, major, minor, u16::MAX),
                                        }
                                    }
                                    // 1.2.*.* -> [1.2.0.0, 1.2.MAX.MAX]
                                    None => Version::new(year, major, u8::MAX, u16::MAX)..=Version::new(year, major, u8::MAX, u16::MAX),
                                }
                            }
                            // 1.*.*.* -> [1.0.0.0, 1.MAX.MAX.MAX]
                            None => Version::new(year, major, u8::MAX, u16::MAX)..=Version::new(year, u8::MAX, u8::MAX, u16::MAX),
                        }
                    }
                    // *.*.*.* -> [0.0.0.0, MAX.MAX.MAX.MAX]
                    None => Version::new(year, major, u8::MAX, u16::MAX)..=Version::new(u32::MAX, u8::MAX, u8::MAX, u16::MAX),
                }
            }
            Self::Greater => {
                match year {
                    Some(year) => {
                        match major {
                            Some(major) => {
                                match minor {
                                    Some(minor) => {
                                        match patch {
                                            // > 1.2.3.4 -> [1.2.3.5, 1.2.3.4]
                                            Some(patch) => Version::new(year, major, minor, patch),
                                            // > 1.2.3.* -> [1.2.3.0, 1.2.3.MAX]
                                            None => Version::new(year, major, minor, u16::MAX),
                                        }
                                    }
                                    // > 1.2.*.* -> [1.2.0.0, 1.2.MAX.MAX]
                                    None => Version::new(year, major, u8::MAX, u16::MAX),
                                }
                            }
                            // > 1.*.*.* -> [1.0.0.0, 1.MAX.MAX.MAX]
                            None => Version::new(year, u8::MAX, u8::MAX, u16::MAX),
                        }
                    }
                    // > *.*.*.* -> [0.0.0.0, MAX.MAX.MAX.MAX]
                    None => Version::new(u32::MAX, u8::MAX, u8::MAX, u16::MAX),
                }
            }
            Self::GreaterEqual => {
                todo!()
            }
            Self::Less => {
                todo!()
            }
            Self::LessEqual => {
                todo!()
            }
            Self::Tilde => {
                todo!()
            }
            Self::Caret => {
                todo!()
            }
        }
    }
    pub fn lower(&self, year: Option<u32>, major: Option<u8>, minor: Option<u8>, patch: Option<u16>) -> Version {
        match self {
            Self::Exact | Self::Wildcard => {
                match year {
                    Some(year) => {
                        match major {
                            Some(major) => {
                                match minor {
                                    Some(minor) => {
                                        match patch {
                                            // 1.2.3.4 -> [1.2.3.4, 1.2.3.4]
                                            Some(patch) => Version::new(year, major, minor, patch),
                                            // 1.2.3.* -> [1.2.3.0, 1.2.3.MAX]
                                            None => Version::new(year, major, minor, 0),
                                        }
                                    }
                                    // 1.2.*.* -> [1.2.0.0, 1.2.MAX.MAX]
                                    None => Version::new(year, major, 0, 0),
                                }
                            }
                            // 1.*.*.* -> [1.0.0.0, 1.MAX.MAX.MAX]
                            None => Version::new(year, 0, 0, 0),
                        }
                    }
                    // *.*.*.* -> [0.0.0.0, MAX.MAX.MAX.MAX]
                    None => Version::new(0, 0, 0, 0),
                }
            }
            Self::Greater => {
                todo!()
            }
            Self::GreaterEqual => {
                todo!()
            }
            Self::Less => {
                todo!()
            }
            Self::LessEqual => {
                todo!()
            }
            Self::Tilde => {
                todo!()
            }
            Self::Caret => {
                todo!()
            }
        }
    }
}

impl Display for VersionComparator {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Exact => f.write_str("="),
            Self::Greater => f.write_str(">"),
            Self::GreaterEqual => f.write_str(">="),
            Self::Less => f.write_str("<"),
            Self::LessEqual => f.write_str("<="),
            Self::Tilde => f.write_str("~"),
            Self::Caret => f.write_str("^"),
            Self::Wildcard => f.write_str("*"),
        }
    }
}

impl Display for VersionRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        todo!()
    }
}

impl Display for VersionConstraint {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        todo!()
    }
}
