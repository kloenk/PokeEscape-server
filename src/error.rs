use std::fmt;
use std::convert;
use std::io;

pub struct Error {
    kind: ErrorKind,
}

impl Error {
    /// creates a new Error
    pub fn new(kind: ErrorKind) -> Error {
        Error {
            kind,
        }
    }

    fn error_kind_as_str(kind: io::ErrorKind) -> ErrorKind {
        match kind {
            io::ErrorKind::NotFound => ErrorKind::io_NotFound,
            io::ErrorKind::PermissionDenied => ErrorKind::io_PermissionDenied,
            io::ErrorKind::ConnectionRefused => ErrorKind::io_ConnectionRefused,
            io::ErrorKind::ConnectionReset => ErrorKind::io_ConnectionAborted,
            io::ErrorKind::ConnectionAborted => ErrorKind::io_ConnectionAborted,
            io::ErrorKind::NotConnected => ErrorKind::io_NotConnected,
            io::ErrorKind::AddrInUse => ErrorKind::io_AddrInUse,
            io::ErrorKind::AddrNotAvailable => ErrorKind::io_AddrNotAvailable,
            io::ErrorKind::BrokenPipe => ErrorKind::io_BrokenPipe,
            io::ErrorKind::AlreadyExists => ErrorKind::io_AlreadyExists,
            io::ErrorKind::WouldBlock => ErrorKind::io_WouldBlock,
            io::ErrorKind::InvalidInput => ErrorKind::io_InvalidInput,
            io::ErrorKind::InvalidData => ErrorKind::io_InvalidData,
            io::ErrorKind::TimedOut => ErrorKind::io_TimedOut,
            io::ErrorKind::WriteZero => ErrorKind::io_writeZero,
            io::ErrorKind::Interrupted => ErrorKind::io_Interrupted,
            io::ErrorKind::Other => ErrorKind::io_Other,
            io::ErrorKind::UnexpectedEof => ErrorKind::io_UnexpectedEof,
            _ => ErrorKind::Unknown,
        }
    }
}

/// print trait
impl fmt::Display for Error {
    /// standart formater for print! macro
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error of Kind {}", self.kind.to_string()) // TODO: Colors
    }
}

/// Debug print trait
impl fmt::Debug for Error {
    /// formater for `Debug` in print! macro
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(Error of Kind {})", self.kind.to_string())
    }
}

impl std::convert::From<std::io::Error> for Error {
    fn from(io_error: std::io::Error) -> Self {
        let kind = Error::error_kind_as_str(io_error.kind());
        Error {
            kind,
        }
    }
}

impl std::convert::From<std::string::String> for Error {
    fn from(string: String) -> Self {
        Error {
            kind: ErrorKind::Unknown,   // FIXME: better doing
        }
    }
}

pub enum ErrorKind {
    io_NotFound,
    io_PermissionDenied,
    io_ConnectionRefused,
    io_ConnectionReset,
    io_ConnectionAborted,
    io_NotConnected,
    io_AddrInUse,
    io_AddrNotAvailable,
    io_BrokenPipe,
    io_AlreadyExists,
    io_WouldBlock,
    io_InvalidInput,
    io_InvalidData,
    io_TimedOut,
    io_writeZero,
    io_Interrupted,
    io_Other,
    io_UnexpectedEof,
    Unknown,
}

impl ErrorKind {
    pub fn to_string(&self) -> String {
        match self {
            ErrorKind::io_NotFound => String::from("io_NotFound"),
            _ => String::from("Unknown")
        }
    }
}