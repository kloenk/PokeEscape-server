use std::fmt;
use std::io;

// tests as sub file
#[cfg(test)]
mod test;

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
            io::ErrorKind::WriteZero => ErrorKind::IoWriteZero,
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

    /// Io Not Found error, transformed `from std::io::ErrorKind::NotFound`
    IoNotFound,

    /// Io Perssion Denied error, transformed from `std::io::ErrorKind::PermissionDenied
    IoPermissionDenied,

    /// Io Connection Refused error, transformed from `std::io::ErrorKind::ConnectionRefused`
    IoConnectionRefused,

    /// Io ConnectionReset error, transformed from `std::io::ErrorKind::ConnectionReset`
    IoConnectionReset,

    /// Io ConnectionAborted error, transformed from `std::io::ErrorKind::ConnectionAborted`
    IoConnectionAborted,

    /// Io Not Connected error, transformed from `std::io::ErrorKind::NotConnected`
    IoNotConnected,

    /// IO Addr In Use error, transformed from `std::io::ErrorKind::AddrInUse`
    IoAddrInUse,

    /// Io Addr Not Available error, transformed from `std::io::ErrorKind::AddrNotAvailable`
    IoAddrNotAvailable,

    /// Io Broken Pipe error, transformed from `std::io::ErrorKind::BrokenPipe`
    IoBrokenPipe,

    /// IO Already Exists error, transformed from `std::io::ErrorKind::AlreadyExists`
    IoAlreadyExists,

    /// IO Would Block error, transformed from `std::io::ErrorKind::WouldBlock`
    IoWouldBlock,

    /// IO Invalid Input error, transformed from `std::io::ErrorKind::InvalidInput`
    IoInvalidInput,

    /// IO Invalid Data error, transformed from `std::io::ErrorKind::InvalidData`
    IoInvalidData,

    /// IO Timed Out error, transformed from `std::io::ErrorKind::TimedOut`
    IoTimedOut,

    /// IO Write Zero error, transformed from `std::io::ErrorKind::WriteZero`
    IoWriteZero,

    /// IO Interrupted error, transformed from `std::io::ErrorKind::Interrupted`
    IoInterrupted,

    /// IO Other error, transformed from `std::io::ErrorKind::Other`
    IoOther,

    /// IO Unexpected EOF error, transformed from `std::io::ErrorKind::UnexpectedEof
    IoUnexpectedEof,

    /// Other error, used for string to error conversion
    Other(String),

    /// Unknown error, used for default in match statements
    Unknown(String),
}

impl ErrorKind {
    pub fn to_string(&self) -> String { // FIXME: should not be implemented?
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
            ErrorKind::IoWriteZero => String::from("IoWriteZero"),
            ErrorKind::IoInterrupted => String::from("IoInterrupted"),
            ErrorKind::IoOther => String::from("IoOther"),
            ErrorKind::IoUnexpectedEof => String::from("IoUnexpectedEof"),
            ErrorKind::Other(data) => format!("Other({})", data),
            ErrorKind::Unknown(data) => format!("Unknown({})", data),
        }
    }
}