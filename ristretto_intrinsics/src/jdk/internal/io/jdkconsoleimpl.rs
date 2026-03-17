use ristretto_classfile::JAVA_21;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("jdk/internal/io/JdkConsoleImpl.echo(Z)Z", GreaterThanOrEqual(JAVA_21))]
#[async_method]
pub async fn echo<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("jdk.internal.io.JdkConsoleImpl.echo(Z)Z".to_string())
            .into(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_echo() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = echo(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
