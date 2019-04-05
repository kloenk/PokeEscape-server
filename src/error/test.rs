
mod error_kind_error_string {
    use super::super::ErrorKind;
    #[test]
    fn io_not_found() {
        let kind = ErrorKind::IoNotFound;
        assert_eq!(kind.error_string(), String::from("IoNotFound"));
    }

    #[test]
    fn io_permission_denied() {
        let kind = ErrorKind::IoPermissionDenied;
        assert_eq!(kind.error_string(), String::from("IoPermissionDenied"));
    }

    #[test]
    fn io_connection_refused() {
        let kind = ErrorKind::IoConnectionRefused;
        assert_eq!(kind.error_string(), String::from("IoConnectionRefused"));
    }

    #[test]
    fn io_connection_reset() {
        let kind = ErrorKind::IoConnectionReset;
        assert_eq!(kind.error_string(), String::from("IoConnectionReset"));
    }

    #[test]
    fn io_connection_aborted() {
        let kind = ErrorKind::IoConnectionAborted;
        assert_eq!(kind.error_string(), String::from("IoConnectionAborted"));
    }

    #[test]
    fn io_not_connected() {
        let kind = ErrorKind::IoNotConnected;
        assert_eq!(kind.error_string(), String::from("IoNotConnected"));
    }

    #[test]
    fn io_addr_in_use() {
        let kind = ErrorKind::IoAddrInUse;
        assert_eq!(kind.error_string(), String::from("IoAddrInUse"));
    }

    #[test]
    fn io_addr_not_available() {
        let kind = ErrorKind::IoAddrNotAvailable;
        assert_eq!(kind.error_string(), String::from("IoAddrNotAvailable"));
    }

    #[test]
    fn io_broken_pipe() {
        let kind = ErrorKind::IoBrokenPipe;
        assert_eq!(kind.error_string(), String::from("IoBrokenPipe"));
    }

    #[test]
    fn io_already_exists() {
        let kind = ErrorKind::IoAlreadyExists;
        assert_eq!(kind.error_string(), String::from("IoAlreadyExists"));
    }

    #[test]
    fn io_would_block() {
        let kind = ErrorKind::IoWouldBlock;
        assert_eq!(kind.error_string(), String::from("IoWouldBlock"));
    }

    #[test]
    fn io_invalid_input() {
        let kind = ErrorKind::IoInvalidInput;
        assert_eq!(kind.error_string(), String::from("IoInvalidInput"));
    }

    #[test]
    fn io_invalid_data() {
        let kind = ErrorKind::IoInvalidData;
        assert_eq!(kind.error_string(), String::from("IoInvalidData"));
    }

    #[test]
    fn io_timed_out() {
        let kind = ErrorKind::IoTimedOut;
        assert_eq!(kind.error_string(), String::from("IoTimedOut"));
    }

    #[test]
    fn io_write_zero() {
        let kind = ErrorKind::IoWriteZero;
        assert_eq!(kind.error_string(), String::from("IoWriteZero"));
    }

    #[test]
    fn io_interrupted() {
        let kind = ErrorKind::IoInterrupted;
        assert_eq!(kind.error_string(), String::from("IoInterrupted"));
    }

    #[test]
    fn io_other() {
        let kind = ErrorKind::IoOther;
        assert_eq!(kind.error_string(), String::from("IoOther"));
    }

    #[test]
    fn io_unexpected_eof() {
        let kind = ErrorKind::IoUnexpectedEof;
        assert_eq!(kind.error_string(), String::from("IoUnexpectedEof"));
    }

    #[test]
    fn other() {
        let kind = ErrorKind::Other(String::from("test"));
        assert_eq!(kind.error_string(), String::from("Other(test)"));
    }

    #[test]
    fn unknow() {
        let kind = ErrorKind::Unknown(String::from("test"));
        assert_eq!(kind.error_string(), String::from("Unknown(test)"));
    }
}
