use super::*;

#[cfg(feature = "semver")]
impl Into<semver::VersionReq> for VersionRequire {
    /// Used to register to a management system based on semantic versioning,
    fn into(self) -> semver::VersionReq {
        semver::VersionReq { comparators: self.constraints.into_iter().map(Into::into).collect() }
    }
}

#[cfg(feature = "semver")]
impl Into<semver::Comparator> for VersionConstraint {
    /// Used to register to a management system based on semantic versioning,
    fn into(self) -> semver::Comparator {
        semver::Comparator { op: self.comparator.into(), major: 0, minor: None, patch: None, pre: Default::default() }
    }
}
#[cfg(feature = "semver")]
impl Into<semver::Op> for VersionComparator {
    /// Used to register to a management system based on semantic versioning,
    fn into(self) -> semver::Op {
        match self {
            Self::Exact => semver::Op::Exact,
            Self::Greater => semver::Op::Greater,
            Self::GreaterEqual => semver::Op::GreaterEq,
            Self::Less => semver::Op::Less,
            Self::LessEqual => semver::Op::LessEq,
            Self::Tilde => semver::Op::Tilde,
            Self::Caret => semver::Op::Caret,
            Self::Wildcard => semver::Op::Wildcard,
        }
    }
}

impl FromStr for VersionRequire {
    type Err = VersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (version, rest, offset) = Self::parse_advance_version_request(s, 0)?;
        if !rest.is_empty() {
            Err(VersionError::ExtraPart { offset, extra: rest.to_string() })?
        }
        Ok(version)
    }
}

impl VersionRequire {
    /// Parse a version from a string
    pub fn parse_advance_version_request(s: &str, start: usize) -> Result<(VersionRequire, &str, usize), VersionError> {
        Ok((Self { constraints: Vec::with_capacity(0) }, s, start))
    }
}

impl VersionComparator {
    pub fn parse_advance_version_comparator(s: &str, start: usize) -> Result<(VersionComparator, &str, usize), VersionError> {
        Ok((Self::Exact, s, start))
    }
}
