//! DMA error codes

use core::fmt::{self, Display};

/// Error codes returned by some DMA functions and/or held in
/// a channel's job_status variable.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    /// Not found
    NotFound,

    /// Not initialized
    NotInitialized,

    /// Invalid argument
    InvalidArg,

    /// I/O error
    Io,

    /// Timeout
    Timeout,

    /// Busy
    Busy,

    /// Suspend
    Suspend,

    /// Aborted
    Aborted,
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Status: ")?;
        match self {
            Error::NotFound => write!(f, "NOT FOUND"),
            Error::NotInitialized => write!(f, "NOT INITIALIZED"),
            Error::InvalidArg => write!(f, "INVALID ARGUMENT"),
            Error::Io => write!(f, "IO ERROR"),
            Error::Timeout => write!(f, "TIMEOUT"),
            Error::Busy => write!(f, "BUSY"),
            Error::Suspend => write!(f, "SUSPENDED"),
            Error::Aborted => write!(f, "ABORTED"),
        }
    }
}
