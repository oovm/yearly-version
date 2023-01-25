#[derive(Debug, Copy, Clone)]
pub enum Error {
    UnknownError
}

pub type Result<T> = core::result::Result<T, Error>;
