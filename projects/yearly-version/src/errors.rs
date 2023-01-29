use alloc::string::String;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum VersionError {
    MissingPart { part: String, offset: usize },
    InvalidPart { part: String, start: usize, end: usize },
    ExtraPart { extra: String, offset: usize },
}

pub type Result<T> = core::result::Result<T, VersionError>;
