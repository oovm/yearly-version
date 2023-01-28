use super::*;

impl Into<u64> for Version {
    fn into(self) -> u64 {
        (self.year as u64) << 48 + (self.major as u64) << 40 + (self.minor as u64) << 32 + self.patch as u64
    }
}

impl From<u64> for Version {
    fn from(value: u64) -> Self {
        Self { year: (value >> 48) as u32, major: (value >> 40) as u8, minor: (value >> 32) as u8, patch: value as u16 }
    }
}

#[cfg(feature = "semver")]
impl Into<semver::Version> for Version {
    /// Used to register to a management system based on semantic versioning,
    ///
    /// Since the yearly version is stricter, which always complies with semver compatibility semantics.
    fn into(self) -> semver::Version {
        semver::Version {
            major: self.year as u64,
            minor: self.major as u64,
            patch: self.minor as u64,
            pre: match self.patch {
                0 => semver::Prerelease::EMPTY,
                x => unsafe {
                    let pre = alloc::format!("pre.{}", x);
                    semver::Prerelease::from_str(&pre).unwrap_unchecked()
                },
            },
            build: semver::BuildMetadata::EMPTY,
        }
    }
}

impl FromStr for Version {
    type Err = VersionError;

    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        let (version, rest, offset) = Self::parse_advance(s)?;
        if !rest.is_empty() {
            Err(VersionError::ExtraPart { offset, extra: rest.to_string() })?
        }
        Ok(version)
    }
}

impl Version {
    pub(crate) fn parse_advance(s: &str) -> Result<(Version, &str, usize), VersionError> {
        let mut parts = s.split('.');
        let year = match parts.next() {
            Some(s) => match s.parse() {
                Ok(o) => o,
                Err(_) => Err(VersionError::InvalidYear { offset: 0 })?,
            },
            None => Err(VersionError::InvalidYear { offset: 0 })?,
        };
        let major = match parts.next() {
            Some(s) => match s.parse() {
                Ok(o) => o,
                Err(_) => Err(VersionError::InvalidMajor { offset: 0 })?,
            },
            None => Err(VersionError::InvalidMajor { offset: 0 })?,
        };
        let minor = match parts.next() {
            Some(s) => match s.parse() {
                Ok(o) => o,
                Err(_) => Err(VersionError::InvalidMinor { offset: 0 })?,
            },
            None => Err(VersionError::InvalidMinor { offset: 0 })?,
        };
        let patch = match parts.next() {
            Some(s) => match s.parse() {
                Ok(o) => o,
                Err(_) => Err(VersionError::InvalidPatch { offset: 0 })?,
            },
            None => Err(VersionError::InvalidPatch { offset: 0 })?,
        };
        let version = Version { year, major, minor, patch };
        Ok((version, "", 0))
    }
}
