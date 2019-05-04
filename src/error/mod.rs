use std::fmt;
use std::io;

// tests as sub module
#[cfg(test)] // only add when running tests
mod test;

/// public type for Result predifined with `error::Error` as Error type
pub type Result<T> = std::result::Result<T, Error>;

/// Error object used for Return values
pub struct Error {
    my_kind: ErrorKind,
}

impl Error {
    /// creates a new Error
    pub fn new(kind: ErrorKind) -> Self {
        Error { my_kind: kind }
    }

    /// returns the internal type of the error
    pub fn kind(&self) -> ErrorKind {
        self.my_kind.clone()
    }

    /// creates a new error of the type NoVersionSupplied
    pub fn new_no_version_supplied() -> Self {
        Self::new(ErrorKind::NoVersionSupplied)
    }

    /// creates a new error of the Kind FieldNotExists
    pub fn new_field_not_exists(field: String) -> Self {
        Self::new(ErrorKind::FieldNotExists(field))
    }

    /// converts a `std::io::ErrorKind` to an own ErrorKind enum
    fn io_to_kind(kind: io::ErrorKind) -> ErrorKind {
        match kind {
            io::ErrorKind::NotFound => ErrorKind::IoNotFound,
            io::ErrorKind::PermissionDenied => ErrorKind::IoPermissionDenied,
            io::ErrorKind::ConnectionRefused => ErrorKind::IoConnectionRefused,
            io::ErrorKind::ConnectionReset => ErrorKind::IoConnectionReset,
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

/// implement std::fmt::Display to allow printing and to_string()
impl fmt::Display for Error {
    /// standart formater for print! macro
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error of Kind {}", self.my_kind.to_string()) // TODO: create a more readable output
    }
}

/// implement std::fmt::Debug to allow Debug acces to the error
impl fmt::Debug for Error {
    /// formater for `Debug` in print! macro
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(Error of Kind {})", self.my_kind.to_string())
    }
}

/// implement std::convert::From for std::io::Error
impl std::convert::From<std::io::Error> for Error {
    fn from(io_error: std::io::Error) -> Self {
        let kind = Error::io_to_kind(io_error.kind());
        Error { my_kind: kind }
    }
}

/// implement std::convert::From for std::io::ErrorKind
impl std::convert::From<std::io::ErrorKind> for Error {
    fn from(io_kind: std::io::ErrorKind) -> Self {
        let kind = Error::io_to_kind(io_kind);
        Error { my_kind: kind }
    }
}

/// implement std::convert::From for string
impl std::convert::From<std::string::String> for Error {
    fn from(string: String) -> Self {
        Error {
            my_kind: ErrorKind::Other(string),
        }
    }
}

/// implement std::convert::From for toml::de::Error
impl std::convert::From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Error {
            my_kind: ErrorKind::NotParsable(err.to_string()),
        }
    }
}

/// implement std::convert::from for serde_json::error::Error
impl std::convert::From<serde_json::error::Error> for Error {
    fn from(err: serde_json::error::Error) -> Self {
        Error {
            my_kind: ErrorKind::NotParsable(err.to_string()),
        }
    }
}

/// implement std::convert::from for semver::SemVerError
impl std::convert::From<semver::SemVerError> for Error {
    fn from(err: semver::SemVerError) -> Self {
        let error = match err {
            semver::SemVerError::ParseError(error) => error,
        };
        Error {
            my_kind: ErrorKind::VersionNotParsable(error),
        }
    }
}

/// implement std::cmp::PartialEq for Error to provied the `==` operator
impl std::cmp::PartialEq for Error {
    fn eq(&self, other: &Error) -> bool {
        self.my_kind == other.my_kind
    }
}

/// ErrorKind represents the internal Error type of the Error
#[derive(Clone, PartialEq)]
pub enum ErrorKind {
    // FIXME: private or rename to Error
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

    /// Format Not Supported, raised when the format defined in config.toml is not supported
    FormatNotSupported,

    /// Field Not Exists, raised when a important field is missing in config
    /// also raised if file is of the wrong type
    FieldNotExists(String), // TODO: create better type for data

    /// Not Parsable error, raised when serde reports an error
    NotParsable(String), // TODO: create better type for data

    /// No Version Supplied error, used if the version of the client is none
    NoVersionSupplied,

    /// Version Not Parsable error, used if the version cannot be parsed
    VersionNotParsable(String),

    /// Pool To Small is returned when the Threapool is to small to be created
    PoolToSmall,

    /// Pool Send Error is a send error in a channel for the ThreadPool
    /// contains true if it wasn't a terminate instruction
    PoolSendError(bool),

    /// Other error, used for string to error conversion
    Other(String),

    /// Unknown error, used for default in match statements
    Unknown(String),
}

impl ErrorKind {
    /// represents the type as String
    fn convert_to_string(&self) -> String {
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
            ErrorKind::FormatNotSupported => String::from("FormatNotSupported"),
            ErrorKind::FieldNotExists(data) => format!("FieldNotExists({})", data),
            ErrorKind::NotParsable(data) => format!("NotParsable({})", data),
            ErrorKind::NoVersionSupplied => String::from("NoVersionSupplied"),
            ErrorKind::VersionNotParsable(data) => format!("VersionNotParsable({})", data),
            ErrorKind::PoolToSmall => String::from("PoolToSmall"),
            ErrorKind::PoolSendError(t) => match t {
                true => String::from("PoolSendError(Job)"),
                false => String::from("PoolSendError(Terminate)"),
            },
            ErrorKind::Other(data) => format!("Other({})", data),
            ErrorKind::Unknown(data) => format!("Unknown({})", data),
        }
    }

    /// returns the name of the field of the enum as String
    pub fn error_string(&self) -> String {
        self.convert_to_string()
    }
}

/// implements std::fmt::Display to provide printing and to_string()
impl fmt::Display for ErrorKind {
    /// standart formater for print! macro
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.convert_to_string())
    }
}

/// Debug print trait
impl fmt::Debug for ErrorKind {
    /// formater for `Debug` in print! macro
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(Error: {})", self.convert_to_string())
    }
}

/// implement std::convert::From for std::io::ErrorKind
impl std::convert::From<std::io::ErrorKind> for ErrorKind {
    fn from(io_kind: std::io::ErrorKind) -> Self {
        Error::io_to_kind(io_kind)
    }
}
