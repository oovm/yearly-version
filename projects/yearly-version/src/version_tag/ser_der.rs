use super::*;
use alloc::string::ToString;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

impl Serialize for VersionTag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for VersionTag {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match VersionTag::from_str(&s) {
            Ok(o) => Ok(o),
            Err(e) => Err(serde::de::Error::custom(e)),
        }
    }
}
