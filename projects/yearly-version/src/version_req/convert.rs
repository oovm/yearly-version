use super::*;


#[cfg(feature = "semver")]
impl Into<semver::VersionReq> for VersionRequest {
    /// Used to register to a management system based on semantic versioning,
    fn into(self) -> semver::VersionReq {
        todo!()
    }
}

impl FromStr for VersionRequest {
    type Err = VersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (version, rest, offset) = Self::parse_advance_version_request(s, 0)?;
        if !rest.is_empty() {
            Err(VersionError::ExtraPart { offset, extra: rest.to_string() })?
        }
        Ok(version)
    }
}

impl VersionRequest {
    /// Parse a version from a string
    pub fn parse_advance_version_request(s: &str, start: usize) -> Result<(VersionRequest, &str, usize), VersionError> {
        todo!()
    }
}

impl VersionComparator {
    pub fn parse_advance_version_comparator(s: &str, start: usize) -> Result<(VersionRequest, &str, usize), VersionError> {
        todo!()
    }
}