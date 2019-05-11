//! DMA status codes

use core::fmt::{self, Display};

// TODO(tarcieri): refactor these into `Result` and `Error` types?

/// Status codes returned by some DMA functions and/or held in
/// a channel's job_status variable.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Status {
    /// Ok
    Ok,

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

impl Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Status: ")?;
        match self {
            Status::Ok => write!(f, "OK"),
            Status::NotFound => write!(f, "NOT FOUND"),
            Status::NotInitialized => write!(f, "NOT INITIALIZED"),
            Status::InvalidArg => write!(f, "INVALID ARGUMENT"),
            Status::Io => write!(f, "IO ERROR"),
            Status::Timeout => write!(f, "TIMEOUT"),
            Status::Busy => write!(f, "BUSY"),
            Status::Suspend => write!(f, "SUSPENDED"),
            Status::Aborted => write!(f, "ABORTED"),
        }
    }
}
