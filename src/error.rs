use std::{error, fmt, io};

/// Common result type used in the public API of thise crate.
pub type AsmResult<T = ()> = ::std::result::Result<T, AsmError>;

/// Common error type used for the public API of this crate.
#[derive(Debug)]
pub enum AsmError {
    /// An error originating from `std::io`.
    Io(::std::io::Error),
}

impl From<io::Error> for AsmError {
    fn from(e: io::Error) -> Self {
        AsmError::Io(e)
    }
}

impl fmt::Display for AsmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AsmError::Io(e) => e.fmt(f),
        }
    }
}

impl error::Error for AsmError {}
