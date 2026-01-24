use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "jdk/internal/jimage/NativeImageBuffer.getNativeMap(Ljava/lang/String;)Ljava/nio/ByteBuffer;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn get_native_map(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "jdk.internal.jimage.NativeImageBuffer.getNativeMap(Ljava/lang/String;)Ljava/nio/ByteBuffer;"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.jimage.NativeImageBuffer.getNativeMap(Ljava/lang/String;)Ljava/nio/ByteBuffer;"
    )]
    async fn test_get_native_map() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_native_map(thread, Parameters::default()).await;
    }
}
