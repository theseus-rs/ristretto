use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `jdk.internal.reflect.NativeMethodAccessorImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "jdk/internal/reflect/NativeMethodAccessorImpl";
    registry.register(
        class_name,
        "invoke0",
        "(Ljava/lang/reflect/Method;Ljava/lang/Object;[Ljava/lang/Object;)Ljava/lang/Object;",
        invoke_0,
    );
}

#[async_recursion(?Send)]
async fn invoke_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.reflect.NativeMethodAccessorImpl.invoke0(Ljava/lang/reflect/Method;Ljava/lang/Object;[Ljava/lang/Object;)Ljava/lang/Object;")
}
