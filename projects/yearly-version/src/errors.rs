#[derive(Debug, Copy, Clone)]
pub enum VersionError {
    MissingYear,
    MissingMajor,
    MissingMinor,
    MissingPatch,
}

pub type Result<T> = core::result::Result<T, VersionError>;
