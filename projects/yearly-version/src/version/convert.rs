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

impl FromStr for Version {
    type Err = ();

    fn from_str(s: &str) -> core::result::Result<Self, Self::Err> {
        todo!()
    }
}

