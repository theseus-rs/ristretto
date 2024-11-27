use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.nio.MappedByteBuffer`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/nio/MappedByteBuffer";
    registry.register(
        class_name,
        "force0",
        "(Ljava/io/FileDescriptor;JJ)V",
        force_0,
    );
    registry.register(class_name, "isLoaded0", "(JJI)Z", is_loaded_0);
    registry.register(class_name, "load0", "(JJ)V", load_0);
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn force_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn is_loaded_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn load_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
