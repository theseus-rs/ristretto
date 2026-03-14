use ristretto_classfile::VersionSpecification::Between;
use ristretto_classfile::{JAVA_11, JAVA_17};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "java/net/SocketCleanable.cleanupClose0(I)V",
    Between(JAVA_11, JAVA_17)
)]
#[async_method]
pub async fn cleanup_close_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _fd = parameters.pop_int()?;
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cleanup_close_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = cleanup_close_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
