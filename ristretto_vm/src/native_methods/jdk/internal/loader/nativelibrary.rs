use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `jdk.internal.loader.NativeLibrary`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "jdk/internal/loader/NativeLibrary";
    registry.register(
        class_name,
        "findEntry0",
        "(JLjava/lang/String;)J",
        find_entry_0,
    );
}

#[async_recursion(?Send)]
async fn find_entry_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.loader.NativeLibrary.findEntry0(JLjava/lang/String;)J")
}
