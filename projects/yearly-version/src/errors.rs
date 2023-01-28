use alloc::string::String;

#[derive(Debug, Clone)]
pub enum VersionError {
    InvalidYear { offset: usize },
    InvalidMajor { offset: usize },
    InvalidMinor { offset: usize },
    InvalidPatch { offset: usize },
    ExtraPart { offset: usize, extra: String },
}

pub type Result<T> = core::result::Result<T, VersionError>;
