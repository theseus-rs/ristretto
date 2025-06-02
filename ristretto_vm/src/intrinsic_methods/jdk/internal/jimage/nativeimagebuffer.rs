use crate::Result;
use crate::intrinsic_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "jdk/internal/jimage/NativeImageBuffer";

/// Register all intrinsic methods for `jdk.internal.jimage.NativeImageBuffer`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "getNativeMap",
        "(Ljava/lang/String;)Ljava/nio/ByteBuffer;",
        get_native_map,
    );
}

#[async_recursion(?Send)]
async fn get_native_map(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
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
