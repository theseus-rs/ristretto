use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.nio.MappedMemoryUtils`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/nio/MappedMemoryUtils";
    registry.register(
        class_name,
        "force0",
        "(Ljava/io/FileDescriptor;JJ)V",
        force_0,
    );
    registry.register(class_name, "isLoaded0", "(JJJ)Z", is_loaded_0);
    registry.register(class_name, "load0", "(JJ)V", load_0);
    registry.register(class_name, "unload0", "(JJ)V", unload_0);
}

#[async_recursion(?Send)]
async fn force_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.nio.MappedMemoryUtils.force0(Ljava/io/FileDescriptor;JJ)V")
}

#[async_recursion(?Send)]
async fn is_loaded_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.nio.MappedMemoryUtils.isLoaded0(JJJ)Z")
}

#[async_recursion(?Send)]
async fn load_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.nio.MappedMemoryUtils.load0(JJ)V")
}

#[async_recursion(?Send)]
async fn unload_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.nio.MappedMemoryUtils.unload0(JJ)V")
}
