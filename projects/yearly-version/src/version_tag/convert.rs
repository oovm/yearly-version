use crate::VersionError;
use super::*;

#[cfg(feature = "semver")]
impl Into<semver::Version> for VersionTag {
    fn into(self) -> semver::Version {
        semver::Version {
            major: self.number.year as u64,
            minor: self.number.major as u64,
            patch: self.number.minor as u64,
            pre: semver::Prerelease::from_str(&alloc::format!("pre.{}", self.number.patch)).unwrap(),
            build: semver::BuildMetadata::from_str(&self.tag).unwrap(),
        }
    }
}

impl FromStr for VersionTag {
    type Err = VersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}