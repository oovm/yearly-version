use alloc::string::String;
use core::fmt::{Display, Formatter};

pub type Result<T> = core::result::Result<T, VersionError>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum VersionError {
    MissingPart { part: String, offset: usize },
    InvalidPart { part: String, start: usize, end: usize },
    ExtraPart { extra: String, offset: usize },
}

impl Display for VersionError {
    fn fmt(&self, _: &mut Formatter<'_>) -> core::fmt::Result {
        todo!()
    }
}
