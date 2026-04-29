use ristretto_classloader::Value;
use ristretto_types::Thread;
use std::sync::Arc;

/// Create and throw a `sun.nio.fs.UnixException` with the given errno.
///
/// Returns a `ristretto_types::Error::Throwable` wrapping the exception object.
pub(crate) async fn throw_unix_exception<T: Thread + 'static>(
    thread: &Arc<T>,
    errno: i32,
) -> ristretto_types::Error {
    match thread
        .object("sun.nio.fs.UnixException", "I", &[Value::Int(errno)])
        .await
    {
        Ok(exception) => ristretto_types::Error::Throwable(exception),
        Err(e) => e,
    }
}

/// Create and throw a `sun.nio.fs.WindowsException` with the given Windows error code.
///
/// Returns a `ristretto_types::Error::Throwable` wrapping the exception object.
#[cfg(target_os = "windows")]
pub(crate) async fn throw_windows_exception<T: Thread + 'static>(
    thread: &Arc<T>,
    last_error: i32,
) -> ristretto_types::Error {
    match thread
        .object(
            "sun.nio.fs.WindowsException",
            "I",
            &[Value::Int(last_error)],
        )
        .await
    {
        Ok(exception) => ristretto_types::Error::Throwable(exception),
        Err(e) => e,
    }
}

/// Map a `std::io::Error` to a Windows error code suitable for `WindowsException`.
///
/// See [System Error Codes](https://learn.microsoft.com/en-us/windows/win32/debug/system-error-codes).
#[cfg(target_os = "windows")]
pub(crate) fn windows_error_code(error: &std::io::Error) -> i32 {
    use std::io::ErrorKind;
    if let Some(code) = error.raw_os_error() {
        return code;
    }
    match error.kind() {
        ErrorKind::NotFound => 2,         // ERROR_FILE_NOT_FOUND
        ErrorKind::PermissionDenied => 5, // ERROR_ACCESS_DENIED
        ErrorKind::AlreadyExists => 183,  // ERROR_ALREADY_EXISTS
        ErrorKind::InvalidInput => 87,    // ERROR_INVALID_PARAMETER
        _ => 1,                           // ERROR_INVALID_FUNCTION
    }
}

#[cfg(all(test, not(target_family = "wasm")))]
mod tests {
    use super::*;

    #[cfg(target_family = "unix")]
    #[tokio::test]
    async fn test_throw_unix_exception() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let error = throw_unix_exception(&thread, 2).await;
        assert!(
            matches!(error, ristretto_types::Error::Throwable(_)),
            "expected Throwable, got {error:?}"
        );
    }

    #[cfg(target_family = "unix")]
    #[tokio::test]
    async fn test_throw_unix_exception_zero_errno() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let error = throw_unix_exception(&thread, 0).await;
        assert!(
            matches!(
                error,
                ristretto_types::Error::Throwable(_)
                    | ristretto_types::Error::InternalError(_)
                    | ristretto_types::Error::JavaError(_)
            ),
            "expected an error variant, got {error:?}"
        );
    }

    #[cfg(target_family = "unix")]
    #[tokio::test]
    async fn test_throw_unix_exception_negative_errno() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let error = throw_unix_exception(&thread, -1).await;
        assert!(
            matches!(
                error,
                ristretto_types::Error::Throwable(_)
                    | ristretto_types::Error::InternalError(_)
                    | ristretto_types::Error::JavaError(_)
            ),
            "expected an error variant, got {error:?}"
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_throw_windows_exception() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let error = throw_windows_exception(&thread, 2).await;
        assert!(
            matches!(error, ristretto_types::Error::Throwable(_)),
            "expected Throwable, got {error:?}"
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_throw_windows_exception_zero_error() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let error = throw_windows_exception(&thread, 0).await;
        assert!(
            matches!(
                error,
                ristretto_types::Error::Throwable(_)
                    | ristretto_types::Error::InternalError(_)
                    | ristretto_types::Error::JavaError(_)
            ),
            "expected an error variant, got {error:?}"
        );
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn test_windows_error_code_raw_os_error() {
        let error = std::io::Error::from_raw_os_error(123);
        assert_eq!(windows_error_code(&error), 123);
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn test_windows_error_code_not_found() {
        let error = std::io::Error::from(std::io::ErrorKind::NotFound);
        let code = windows_error_code(&error);
        assert!(code == 2 || code != 0, "unexpected code {code}");
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn test_windows_error_code_permission_denied() {
        let error = std::io::Error::from(std::io::ErrorKind::PermissionDenied);
        let code = windows_error_code(&error);
        assert!(code == 5 || code != 0, "unexpected code {code}");
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn test_windows_error_code_already_exists() {
        let error = std::io::Error::from(std::io::ErrorKind::AlreadyExists);
        let code = windows_error_code(&error);
        assert!(code == 183 || code != 0, "unexpected code {code}");
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn test_windows_error_code_invalid_input() {
        let error = std::io::Error::from(std::io::ErrorKind::InvalidInput);
        let code = windows_error_code(&error);
        assert!(code == 87 || code != 0, "unexpected code {code}");
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn test_windows_error_code_other_kind() {
        let error = std::io::Error::other("error");
        assert_eq!(windows_error_code(&error), 1);
    }
}
