use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `jdk.internal.loader.RawNativeLibraries`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "jdk/internal/loader/RawNativeLibraries";
    registry.register(
        class_name,
        "load0",
        "(Ljdk/internal/loader/RawNativeLibraries$RawNativeLibraryImpl;Ljava/lang/String;)Z",
        load_0,
    );
    registry.register(class_name, "unload0", "(Ljava/lang/String;J)V", unload_0);
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn load_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn unload_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
