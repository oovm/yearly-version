use core::str::FromStr;

use super::*;

impl Into<u64> for Version {
    fn into(self) -> u64 {
        (self.year as u64) << 48 + (self.major as u64) << 40 + (self.minor as u64) << 32 + self.patch as u64
    }
}

impl From<u64> for Version {
    fn from(value: u64) -> Self {
        Self {
            year: (value >> 48) as u32,
            major: (value >> 40) as u8,
            minor: (value >> 32) as u8,
            patch: value as u16,
        }
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
            pre:
            match self.patch {
                0 => { semver::Prerelease::EMPTY }
                x => unsafe {
                    let pre = alloc::format!("pre.{}", x);
                    semver::Prerelease::from_str(&pre).unwrap_unchecked()
                }
            },
            build: semver::BuildMetadata::EMPTY,
        }
    }
}

impl FromStr for Version {
    type Err = ();

    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        let mut parts = s.split('.');
        let year = parts
            .next()
            .and_then(|part| part.parse::<u32>().ok())
            .ok_or(())?;
        let major = parts
            .next()
            .and_then(|part| part.parse::<u8>().ok())
            .ok_or(())?;
        let minor = parts
            .next()
            .and_then(|part| part.parse::<u8>().ok())
            .ok_or(())?;
        let patch = parts
            .next()
            .and_then(|part| part.parse::<u16>().ok())
            .ok_or(())?;
        Ok(Version {
            year,
            major,
            minor,
            patch,
        })
    }
}

