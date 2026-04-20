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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_throw_unix_exception() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let error = throw_unix_exception(&thread, 2).await;
        assert!(
            matches!(error, ristretto_types::Error::Throwable(_)),
            "expected Throwable, got {error:?}"
        );
    }

    #[tokio::test]
    async fn test_throw_unix_exception_zero_errno() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let error = throw_unix_exception(&thread, 0).await;
        // Even with errno 0, should still produce a Throwable (or possibly an internal
        // error if the class is not found, but it should not panic)
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
}
