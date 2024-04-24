use super::*;

impl From<Version> for u128 {
    fn from(value: Version) -> Self {
        ((value.year as u128) << 96) | ((value.major as u128) << 64) | ((value.minor as u128) << 32) | (value.patch as u128)
    }
}

impl From<u128> for Version {
    fn from(v: u128) -> Self {
        Self {
            year: (v >> 96) as u32,
            major: ((v >> 64) & 0xFFFF) as u32,
            minor: ((v >> 32) & 0xFFFF) as u32,
            patch: (v & 0xFFFF) as u32,
        }
    }
}

#[cfg(feature = "semver")]
impl Into<semver::Version> for Version {
    /// Used to register to a management system based on semantic versioning,
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

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (version, rest, offset) = Self::parse_advance_version(s, 0)?;
        if !rest.is_empty() {
            Err(VersionError::ExtraPart { offset, extra: rest.to_string() })?
        }
        Ok(version)
    }
}

impl Version {
    /// Parse a version from a string
    pub fn parse_advance_version(s: &str, start: usize) -> Result<(Version, &str, usize), VersionError> {
        let (year, rest, offset) = Self::parse_advance_year(s, start)?;
        let (major, rest, offset) = Self::parse_advance_major(rest, offset)?;
        let (minor, rest, offset) = Self::parse_advance_minor(rest, offset)?;
        let (patch, rest, offset) = Self::parse_advance_patch(rest, offset)?;
        Ok((Version { year, major, minor, patch }, rest, offset))
    }
    pub fn parse_advance_year(input: &str, offset: usize) -> Result<(u32, &str, usize), VersionError> {
        match input.find('.') {
            Some(position) => {
                let split = unsafe { input.get_unchecked(0..position) };
                let rest = unsafe { input.get_unchecked(position + 1..) };
                match split.parse() {
                    Ok(o) => Ok((o, rest, offset + position + 1)),
                    Err(_) => {
                        Err(VersionError::InvalidPart { part: "year".to_string(), start: offset, end: offset + position })?
                    }
                }
            }
            None => Err(VersionError::MissingPart { part: "major".to_string(), offset }),
        }
    }
    pub fn parse_advance_major(input: &str, offset: usize) -> Result<(u32, &str, usize), VersionError> {
        match input.find('.') {
            Some(position) => {
                let split = unsafe { input.get_unchecked(0..position) };
                let rest = unsafe { input.get_unchecked(position + 1..) };
                match split.parse() {
                    Ok(o) => Ok((o, rest, offset + position + 1)),
                    Err(_) => {
                        Err(VersionError::InvalidPart { part: "major".to_string(), start: offset, end: offset + position })?
                    }
                }
            }
            None => Err(VersionError::MissingPart { part: "minor".to_string(), offset }),
        }
    }
    pub fn parse_advance_minor(input: &str, offset: usize) -> Result<(u32, &str, usize), VersionError> {
        match input.find('.') {
            Some(position) => {
                let split = unsafe { input.get_unchecked(0..position) };
                let rest = unsafe { input.get_unchecked(position + 1..) };
                match split.parse() {
                    Ok(o) => Ok((o, rest, offset + position + 1)),
                    Err(_) => {
                        Err(VersionError::InvalidPart { part: "minor".to_string(), start: offset, end: offset + position })?
                    }
                }
            }
            None => Err(VersionError::MissingPart { part: "patch".to_string(), offset }),
        }
    }

    pub fn parse_advance_patch(input: &str, offset: usize) -> Result<(u32, &str, usize), VersionError> {
        match input.find('-') {
            Some(position) => {
                let split = unsafe { input.get_unchecked(0..position) };
                let rest = unsafe { input.get_unchecked(position + 1..) };
                match split.parse() {
                    Ok(o) => Ok((o, rest, offset + position + 1)),
                    Err(_) => {
                        Err(VersionError::InvalidPart { part: "patch".to_string(), start: offset, end: offset + position })?
                    }
                }
            }
            None => {
                let position = input.len();
                let split = input;
                let rest = "";
                match split.parse() {
                    Ok(o) => Ok((o, rest, offset + position + 1)),
                    Err(_) => {
                        Err(VersionError::InvalidPart { part: "patch".to_string(), start: offset, end: offset + position })?
                    }
                }
            }
        }
    }
}
