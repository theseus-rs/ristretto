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
    registry.register(
        class_name,
        "getNativeMap",
        "(Ljava/lang/String;)Ljava/nio/ByteBuffer;",
        get_native_map,
    );
}

#[async_recursion(?Send)]
async fn get_native_map(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
