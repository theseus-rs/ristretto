use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.nio.Bits`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/nio/Bits";
    registry.register(
        class_name,
        "copySwapMemory0",
        "(Ljava/lang/Object;JLjava/lang/Object;JJJ)V",
        copy_swap_memory_0,
    );
}

#[async_recursion(?Send)]
async fn copy_swap_memory_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
