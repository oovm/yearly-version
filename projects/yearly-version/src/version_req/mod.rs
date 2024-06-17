use crate::{Version, VersionError};
use alloc::{string::ToString, vec::Vec};
use core::{
    fmt::{Display, Formatter},
    ops::{Add, RangeInclusive},
    str::FromStr,
};

mod convert;
#[cfg(feature = "schemars")]
mod json_schema;
#[cfg(feature = "serde")]
mod ser_der;

#[repr(C, align(8))]
#[derive(Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct VersionRequire {
    /// >1.0; <2.0
    pub constraints: Vec<VersionConstraint>,
}

#[repr(C, align(8))]
#[derive(Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct VersionConstraint {
    pub comparator: VersionComparator,
    pub year: Option<u32>,
    pub major: Option<u32>,
    pub minor: Option<u32>,
    pub patch: Option<u32>,
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
    /// Get the upper bound of the range
    pub fn upper(&self) -> Option<Version> {
        Some(*self.comparator.range(self.year, self.major, self.minor, self.patch)?.start())
    }
    /// Get the lower bound of the range
    pub fn lower(&self) -> Option<Version> {
        Some(*self.comparator.range(self.year, self.major, self.minor, self.patch)?.end())
    }
    /// Check if the constraint is unlimited
    pub fn unlimited(&self) -> bool {
        self.year.is_none() && self.major.is_none() && self.minor.is_none() && self.patch.is_none()
    }
}

impl VersionComparator {
    pub fn range(&self, year: Option<u32>, major: Option<u32>, minor: Option<u32>, patch: Option<u32>) -> Option<RangeInclusive<Version>> {
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
                                            Some(patch) => Some(Version::new(year, major, minor, patch)..=Version::new(year, major, minor, patch)),
                                            // 1.2.3.* -> [1.2.3.0, 1.2.3.MAX]
                                            None => Some(Version::new(year, major, minor, 0)..=Version::new(year, major, minor, u32::MAX)),
                                        }
                                    }
                                    // 1.2.*.* -> [1.2.0.0, 1.2.MAX.MAX]
                                    None => Some(Version::new(year, major, 0, 0)..=Version::new(year, major, u32::MAX, u32::MAX)),
                                }
                            }
                            // 1.*.*.* -> [1.0.0.0, 1.MAX.MAX.MAX]
                            None => Some(Version::new(year, 0, 0, 0)..=Version::new(year, u32::MAX, u32::MAX, u32::MAX)),
                        }
                    }
                    // *.*.*.* -> [0.0.0.0, MAX.MAX.MAX.MAX]
                    None => Some(Version::new(0, 0, 0, 0)..=Version::new(u32::MAX, u32::MAX, u32::MAX, u32::MAX)),
                }
            }
            Self::Greater => {
                match year {
                    Some(u32::MAX) => None,
                    Some(year) => {
                        match major {
                            Some(u32::MAX) => None,
                            Some(major) => {
                                match minor {
                                    Some(u32::MAX) => None,
                                    Some(minor) => {
                                        match patch {
                                            // > 1.2.3.4 -> [1.2.3.5, MAX]
                                            Some(u32::MAX) => None,
                                            Some(patch) => Some(Version::new(year, major, minor, unsafe { patch.add(1) })..=Version::MAX),
                                            // > 1.2.3 -> [1.2.4.0, MAX]
                                            None => Some(Version::new(year, major, unsafe { minor.add(1) }, 0)..=Version::MAX),
                                        }
                                    }
                                    // > 1.2 -> [1.3.0.0, MAX]
                                    None => Some(Version::new(year, unsafe { major.add(1) }, 0, 0)..=Version::MAX),
                                }
                            }
                            // > 1 -> [2.0.0.0, MAX]
                            None => Some(Version::new(unsafe { year.add(1) }, 0, 0, 0)..=Version::MAX),
                        }
                    }
                    // > * -> impossible
                    None => None,
                }
            }
            Self::GreaterEqual => {
                match year {
                    Some(year) => {
                        match major {
                            Some(major) => {
                                match minor {
                                    Some(minor) => {
                                        match patch {
                                            // ⩾ 1.2.3.4 -> [1.2.3.4, MAX]
                                            Some(patch) => Some(Version::new(year, major, minor, patch)..=Version::MAX),
                                            // ⩾ 1.2.3 -> [1.2.4.0, MAX]
                                            None => Some(Version::new(year, major, minor, 0)..=Version::MAX),
                                        }
                                    }
                                    // ⩾ 1.2 -> [1.3.0.0, MAX]
                                    None => Some(Version::new(year, major, 0, 0)..=Version::MAX),
                                }
                            }
                            // ⩾ 1 -> [2.0.0.0, MAX]
                            None => Some(Version::new(year, 0, 0, 0)..=Version::MAX),
                        }
                    }
                    // ⩾ * -> [MAX, MAX]
                    None => Some(Version::MAX..=Version::MAX),
                }
            }
            Self::Less => {
                let upper = match year {
                    Some(year) => {
                        match major {
                            Some(major) => {
                                match minor {
                                    Some(minor) => {
                                        match patch {
                                            // < 1.2.3.0 -> [MIN, 1.2.2.MAX]
                                            // < 1.2.3.4 -> [MIN, 1.2.3.3]
                                            Some(patch) => Version::new(year, major, minor, patch),
                                            // < 1.2.0 -> [MIN, 1.1.MAX.MAX]
                                            // < 1.2.3 -> [MIN, 1.2.2.MAX]
                                            None => Version::new(year, major, minor, 0),
                                        }
                                    }
                                    // < 1.0 -> [MIN, 0.MAX.MAX.MAX]
                                    // < 1.2 -> [MIN, 1.1.MAX.MAX]
                                    None => Version::new(year, major, 0, 0),
                                }
                            }
                            // < 0 -> impossible
                            // < 1 -> [MIN, 0.0.0.0]
                            None => Version::new(year, 0, 0, 0),
                        }
                    }
                    // < * -> impossible
                    None => Version::new(0, 0, 0, 0),
                };
                let sub1 = u128::from(upper).checked_sub(1)?;
                Some(Version::MIN..=Version::from(sub1))
            }
            Self::LessEqual => {
                match year {
                    Some(year) => {
                        match major {
                            Some(major) => {
                                match minor {
                                    Some(minor) => {
                                        match patch {
                                            // ⩽ 1.2.3.4 -> [MIN, 1.2.3.4]
                                            Some(patch) => Some(Version::MIN..=Version::new(year, major, minor, patch)),
                                            // ⩽ 1.2.3 -> [MIN, 1.2.3.0]
                                            None => Some(Version::MIN..=Version::new(year, major, minor, 0)),
                                        }
                                    }
                                    // ⩽ 1.2 -> [MIN, 1.2.0.0]
                                    None => Some(Version::MIN..=Version::new(year, major, 0, 0)),
                                }
                            }
                            // ⩽ 1 -> [MIN, 1.0.0.0]
                            None => Some(Version::MIN..=Version::new(year, 0, 0, 0)),
                        }
                    }
                    // ⩽ * -> [MIN, MIN]
                    None => Some(Version::MIN..=Version::MIN),
                }
            }
            // patch updates only mode
            Self::Tilde => {
                todo!()
            }
            // compatibility updates only mode
            Self::Caret => {
                todo!()
            }
        }
    }
    pub fn upper(&self, year: Option<u32>, major: Option<u32>, minor: Option<u32>, patch: Option<u32>) -> Version {
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
                                    None => Version::new(year, major, minor, u32::MAX),
                                }
                            }
                            // 1.2.*.* -> [1.2.0.0, 1.2.MAX.MAX]
                            None => Version::new(year, major, u32::MAX, u32::MAX),
                        }
                    }
                    // 1.*.*.* -> [1.0.0.0, 1.MAX.MAX.MAX]
                    None => Version::new(year, u32::MAX, u32::MAX, u32::MAX),
                }
            }
            // *.*.*.* -> [0.0.0.0, MAX.MAX.MAX.MAX]
            None => Version::new(u32::MAX, u32::MAX, u32::MAX, u32::MAX),
        }
    }
    pub fn lower(&self, year: Option<u32>, major: Option<u32>, minor: Option<u32>, patch: Option<u32>) -> Version {
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

impl Display for VersionRequire {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        todo!()
    }
}

impl Display for VersionConstraint {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        todo!()
    }
}
