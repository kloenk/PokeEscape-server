//! test file to test some things around error

mod io_error_kind_to_error_kind {
    use super::super::ErrorKind;
    use std::io::ErrorKind as IoErrorKind;

    #[test]
    fn io_not_found() {
        let kind = ErrorKind::from(IoErrorKind::NotFound);
        assert_eq!(kind, ErrorKind::IoNotFound);
    }

    #[test]
    fn io_permission_denied() {
        let kind = ErrorKind::from(IoErrorKind::PermissionDenied);
        assert_eq!(kind, ErrorKind::IoPermissionDenied);
    }

    #[test]
    fn io_connection_refused() {
        let kind = ErrorKind::from(IoErrorKind::ConnectionRefused);
        assert_eq!(kind, ErrorKind::IoConnectionRefused);
    }

    #[test]
    fn io_connection_reset() {
        let kind = ErrorKind::from(IoErrorKind::ConnectionReset);
        assert_eq!(kind, ErrorKind::IoConnectionReset);
    }

    #[test]
    fn io_connection_aborted() {
        let kind = ErrorKind::from(IoErrorKind::ConnectionAborted);
        assert_eq!(kind, ErrorKind::IoConnectionAborted);
    }

    #[test]
    fn io_not_connected() {
        let kind = ErrorKind::from(IoErrorKind::NotConnected);
        assert_eq!(kind, ErrorKind::IoNotConnected);
    }

    #[test]
    fn io_addr_in_use() {
        let kind = ErrorKind::from(IoErrorKind::AddrInUse);
        assert_eq!(kind, ErrorKind::IoAddrInUse);
    }

    #[test]
    fn io_addr_not_available() {
        let kind = ErrorKind::from(IoErrorKind::AddrNotAvailable);
        assert_eq!(kind, ErrorKind::IoAddrNotAvailable);
    }

    #[test]
    fn io_broken_pipe() {
        let kind = ErrorKind::from(IoErrorKind::BrokenPipe);
        assert_eq!(kind, ErrorKind::IoBrokenPipe);
    }

    #[test]
    fn io_already_exists() {
        let kind = ErrorKind::from(IoErrorKind::AlreadyExists);
        assert_eq!(kind, ErrorKind::IoAlreadyExists);
    }

    #[test]
    fn io_would_block() {
        let kind = ErrorKind::from(IoErrorKind::WouldBlock);
        assert_eq!(kind, ErrorKind::IoWouldBlock);
    }

    #[test]
    fn io_invalid_input() {
        let kind = ErrorKind::from(IoErrorKind::InvalidInput);
        assert_eq!(kind, ErrorKind::IoInvalidInput);
    }

    #[test]
    fn io_invalid_data() {
        let kind = ErrorKind::from(IoErrorKind::InvalidData);
        assert_eq!(kind, ErrorKind::IoInvalidData);
    }

    #[test]
    fn io_timed_out() {
        let kind = ErrorKind::from(IoErrorKind::TimedOut);
        assert_eq!(kind, ErrorKind::IoTimedOut);
    }

    #[test]
    fn io_write_zero() {
        let kind = ErrorKind::from(IoErrorKind::WriteZero);
        assert_eq!(kind, ErrorKind::IoWriteZero);
    }

    #[test]
    fn io_interrupted() {
        let kind = ErrorKind::from(IoErrorKind::Interrupted);
        assert_eq!(kind, ErrorKind::IoInterrupted);
    }

    #[test]
    fn io_other() {
        let kind = ErrorKind::from(IoErrorKind::Other);
        assert_eq!(kind, ErrorKind::IoOther);
    }

    #[test]
    fn io_unexpected_eof() {
        let kind = ErrorKind::from(IoErrorKind::UnexpectedEof);
        assert_eq!(kind, ErrorKind::IoUnexpectedEof);
    }
}

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
    fn format_not_supported() {
        let kind = ErrorKind::FormatNotSupported;
        assert_eq!(kind.error_string(), String::from("FormatNotSupported"));
    }

    #[test]
    fn field_not_exists() {
        let kind = ErrorKind::FieldNotExists("test".to_string());
        assert_eq!(kind.error_string(), String::from("FieldNotExists(test)"));
    }

    #[test]
    fn not_parsable() {
        let kind = ErrorKind::NotParsable("test".to_string());
        assert_eq!(kind.error_string(), String::from("NotParsable(test)"));
    }

    #[test]
    fn no_version_supplied() {
        let kind = ErrorKind::NoVersionSupplied;
        assert_eq!(kind.error_string(), String::from("NoVersionSupplied"));
    }

    #[test]
    fn version_not_parsable() {
        let kind = ErrorKind::VersionNotParsable("test".to_string());
        assert_eq!(
            kind.error_string(),
            String::from("VersionNotParsable(test)")
        );
    }

    #[test]
    fn pool_to_small() {
        let kind = ErrorKind::PoolToSmall;
        assert_eq!(kind.error_string(), String::from("PoolToSmall"));
    }

    #[test]
    fn pool_send_error() {
        let kind = ErrorKind::PoolSendError(true);
        assert_eq!(kind.error_string(), String::from("PoolSendError(Job)"));

        let kind = ErrorKind::PoolSendError(false);
        assert_eq!(
            kind.error_string(),
            String::from("PoolSendError(Terminate)")
        );
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

// FIXME: tests
