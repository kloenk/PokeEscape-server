use std::fmt;
use std::convert;
use std::io::ErrorKind;

pub struct Error {
    kind: String,
}

impl Error {
    /// creates a new Error
    pub fn new(kind: String) -> Error {
        Error {
            kind,
        }
    }

    fn error_kind_as_str(kind: ErrorKind) -> String {
        match kind {
            ErrorKind::NotFound => String::from("entity not found"),
            ErrorKind::PermissionDenied => String::from("permission denied"),
            ErrorKind::ConnectionRefused => String::from("connection refused"),
            ErrorKind::ConnectionReset => String::from("connection reset"),
            ErrorKind::ConnectionAborted => String::from("connection aborted"),
            ErrorKind::NotConnected => String::from("not connected"),
            ErrorKind::AddrInUse => String::from("address in use"),
            ErrorKind::AddrNotAvailable => String::from("address not available"),
            ErrorKind::BrokenPipe => String::from("broken pipe"),
            ErrorKind::AlreadyExists => String::from("entity already exists"),
            ErrorKind::WouldBlock => String::from("operation would block"),
            ErrorKind::InvalidInput => String::from("invalid input parameter"),
            ErrorKind::InvalidData => String::from("invalid data"),
            ErrorKind::TimedOut => String::from("timed out"),
            ErrorKind::WriteZero => String::from("write zero"),
            ErrorKind::Interrupted => String::from("operation interrupted"),
            ErrorKind::Other => String::from("other os error"),
            ErrorKind::UnexpectedEof => String::from("unexpected end of file"),
            _ => String::from("unknown error"),
        }
    }
}

impl fmt::Display for Error {
    /// standart formater for print! macro
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error of Kind {}", self.kind) // TODO: Colors
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
            kind: string,   // FIXME: better doing
        }
    }
}