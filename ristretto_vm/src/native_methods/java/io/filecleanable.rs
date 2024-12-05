use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.io.FileCleanable`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/io/FileCleanable";
    registry.register(class_name, "cleanupClose0", "(IJ)V", cleanup_close_0);
}

#[async_recursion(?Send)]
async fn cleanup_close_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.io.FileCleanable.cleanupClose0(IJ)V")
}
