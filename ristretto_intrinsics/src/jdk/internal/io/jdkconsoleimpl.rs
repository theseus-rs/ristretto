use ristretto_classfile::JAVA_21;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("jdk/internal/io/JdkConsoleImpl.echo(Z)Z", GreaterThanOrEqual(JAVA_21))]
#[async_method]
pub async fn echo<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.internal.io.JdkConsoleImpl.echo(Z)Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: jdk.internal.io.JdkConsoleImpl.echo(Z)Z")]
    async fn test_echo() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = echo(thread, Parameters::default()).await;
    }
}
