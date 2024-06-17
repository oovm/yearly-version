use alloc::string::String;
use super::*;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

impl Serialize for VersionRequire {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let text = self.to_string();
        serializer.serialize_str(&text)
    }
}

impl<'de> Deserialize<'de> for VersionRequire {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        VersionRequire::from_str(&s).map_err(serde::de::Error::custom)
    }
}
