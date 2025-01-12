use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `jdk.internal.jimage.NativeImageBuffer`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "jdk/internal/jimage/NativeImageBuffer";
    registry.register(class_name, "getNativeMap", "getNativeMap", get_native_map);
}

#[async_recursion(?Send)]
async fn get_native_map(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.jimage.NativeImageBuffer.getNativeMap(Ljava/lang/String;)Ljava/nio/ByteBuffer;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "jdk/internal/jimage/NativeImageBuffer";
        assert!(registry
            .method(class_name, "getNativeMap", "getNativeMap")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "jdk.internal.jimage.NativeImageBuffer.getNativeMap(Ljava/lang/String;)Ljava/nio/ByteBuffer;"
    )]
    async fn test_get_native_map() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_native_map(thread, Arguments::default()).await;
    }
}
