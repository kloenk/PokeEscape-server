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
            io::ErrorKind::NotFound => ErrorKind::IoNotFound,
            io::ErrorKind::PermissionDenied => ErrorKind::IoPermissionDenied,
            io::ErrorKind::ConnectionRefused => ErrorKind::IoConnectionRefused,
            io::ErrorKind::ConnectionReset => ErrorKind::IoConnectionAborted,
            io::ErrorKind::ConnectionAborted => ErrorKind::IoConnectionAborted,
            io::ErrorKind::NotConnected => ErrorKind::IoNotConnected,
            io::ErrorKind::AddrInUse => ErrorKind::IoAddrInUse,
            io::ErrorKind::AddrNotAvailable => ErrorKind::IoAddrNotAvailable,
            io::ErrorKind::BrokenPipe => ErrorKind::IoBrokenPipe,
            io::ErrorKind::AlreadyExists => ErrorKind::IoAlreadyExists,
            io::ErrorKind::WouldBlock => ErrorKind::IoWouldBlock,
            io::ErrorKind::InvalidInput => ErrorKind::IoInvalidInput,
            io::ErrorKind::InvalidData => ErrorKind::IoInvalidData,
            io::ErrorKind::TimedOut => ErrorKind::IoTimedOut,
            io::ErrorKind::WriteZero => ErrorKind::IowriteZero,
            io::ErrorKind::Interrupted => ErrorKind::IoInterrupted,
            io::ErrorKind::Other => ErrorKind::IoOther,
            io::ErrorKind::UnexpectedEof => ErrorKind::IoUnexpectedEof,
            _ => ErrorKind::Unknown(String::from("uknwon io errord")),
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
            kind: ErrorKind::Unknown(string),   // FIXME: better doing
        }
    }
}

pub enum ErrorKind {
    IoNotFound,
    IoPermissionDenied,
    IoConnectionRefused,
    IoConnectionReset,
    IoConnectionAborted,
    IoNotConnected,
    IoAddrInUse,
    IoAddrNotAvailable,
    IoBrokenPipe,
    IoAlreadyExists,
    IoWouldBlock,
    IoInvalidInput,
    IoInvalidData,
    IoTimedOut,
    IowriteZero,
    IoInterrupted,
    IoOther,
    IoUnexpectedEof,
    Unknown(String),
}

impl ErrorKind {
    pub fn to_string(&self) -> String {
        match self {
            ErrorKind::IoNotFound => String::from("IoNotFound"),
            ErrorKind::IoPermissionDenied => String::from("IoPermissionDenied"),
            ErrorKind::IoConnectionRefused => String::from("IoConnectionRefused"),
            ErrorKind::IoConnectionReset => String::from("IoConnectionReset"),
            ErrorKind::IoConnectionAborted => String::from("IoConnectionAborted"),
            ErrorKind::IoNotConnected => String::from("IoNotConnected"),
            ErrorKind::IoAddrInUse => String::from("IoAddrInUse"),
            ErrorKind::IoAddrNotAvailable => String::from("IoAddrNotAvailable"),
            ErrorKind::IoBrokenPipe => String::from("IoBrokenPipe"),
            ErrorKind::IoAlreadyExists => String::from("IoAlreadyExists"),
            ErrorKind::IoWouldBlock => String::from("IoWouldBlock"),
            ErrorKind::IoInvalidInput => String::from("IoInvalidInput"),
            ErrorKind::IoInvalidData => String::from("IoInvalidData"),
            ErrorKind::IoTimedOut => String::from("IoTimedOut"),
            ErrorKind::IowriteZero => String::from("IoWriteZero"),
            ErrorKind::IoInterrupted => String::from("IoInterrupted"),
            ErrorKind::IoOther => String::from("IoOther"),
            ErrorKind::IoUnexpectedEof => String::from("IoUnexpectedEof"),
            ErrorKind::Unknown(data) => format!("Unknown({})", data),
        }
    }
}