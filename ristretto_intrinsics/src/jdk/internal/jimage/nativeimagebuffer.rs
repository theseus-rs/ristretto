use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "jdk/internal/jimage/NativeImageBuffer.getNativeMap(Ljava/lang/String;)Ljava/nio/ByteBuffer;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_native_map<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("jdk.internal.jimage.NativeImageBuffer.getNativeMap(Ljava/lang/String;)Ljava/nio/ByteBuffer;".to_string()).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_native_map() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_native_map(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
