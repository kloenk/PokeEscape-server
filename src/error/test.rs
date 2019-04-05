
use super::*;

#[test]
fn error_kind_to_string() {
    let kind = ErrorKind::IoNotFound;

    // IoNotFound
    assert_eq!(kind.to_string(), String::from("IoNotFound"));

    // IoPermissionDenied
    let kind = ErrorKind::IoPermissionDenied;
    assert_eq!(kind.to_string(), String::from("IoPermissionDenied"));

    // IoConnectionRefused
    let kind = ErrorKind::IoConnectionRefused;
    assert_eq!(kind.to_string(), String::from("IoConnectionRefused"));

    // IoConnectionReset
    let kind = ErrorKind::IoConnectionReset;
    assert_eq!(kind.to_string(), String::from("IoConnectionReset"));

    //IoConnectionAborted
    let kind = ErrorKind::IoConnectionAborted;
    assert_eq!(kind.to_string(), String::from("IoConnectionAborted"));

    // IoNotConnected
    let kind = ErrorKind::IoNotConnected;
    assert_eq!(kind.to_string(), String::from("IoNotConnected"));

    // IoAddrInUse
    let kind = ErrorKind::IoAddrInUse;
    assert_eq!(kind.to_string(), String::from("IoAddrInUse"));

    // IoAddrNotAvailable
    let kind = ErrorKind::IoAddrNotAvailable;
    assert_eq!(kind.to_string(), String::from("IoAddrNotAvailable"));

    // IoBrokenPipe
    let kind = ErrorKind::IoBrokenPipe;
    assert_eq!(kind.to_string(), String::from("IoBrokenPipe"));

    // IoAlreadyExists
    let kind = ErrorKind::IoAlreadyExists;
    assert_eq!(kind.to_string(), String::from("IoAlreadyExists"));

    // IoWouldBlock
    let kind = ErrorKind::IoWouldBlock;
    assert_eq!(kind.to_string(), String::from("IoWouldBlock"));

    // IoInvalidInput
    let kind = ErrorKind::IoInvalidInput;
    assert_eq!(kind.to_string(), String::from("IoInvalidInput"));

    // IoInvalidData
    let kind = ErrorKind::IoInvalidData;
    assert_eq!(kind.to_string(), String::from("IoInvalidData"));

    // IoTimedOut
    let kind = ErrorKind::IoTimedOut;
    assert_eq!(kind.to_string(), String::from("IoTimedOut"));

    // IowriteZero
    let kind = ErrorKind::IoWriteZero;
    assert_eq!(kind.to_string(), String::from("IoWriteZero"));

    // IoInterrupted
    let kind = ErrorKind::IoInterrupted;
    assert_eq!(kind.to_string(), String::from("IoInterrupted"));

    // IoOther
    let kind = ErrorKind::IoOther;
    assert_eq!(kind.to_string(), String::from("IoOther"));

    // IoUnexpectedEof
    let kind = ErrorKind::IoUnexpectedEof;
    assert_eq!(kind.to_string(), String::from("IoUnexpectedEof"));

    // Other
    let kind = ErrorKind::Other(String::from("test"));
    assert_eq!(kind.to_string(), String::from("Other(test)"));

    // Unknow(String)
    let kind = ErrorKind::Unknown(String::from("test"));
    assert_eq!(kind.to_string(), String::from("Unknown(test)"));
}