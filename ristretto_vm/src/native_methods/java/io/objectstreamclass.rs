use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.io.ObjectStreamClass`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/io/ObjectStreamClass";
    registry.register(
        class_name,
        "hasStaticInitializer",
        "(Ljava/lang/Class;)Z",
        has_static_initializer,
    );
    registry.register(class_name, "initNative", "()V", init_native);
}

#[async_recursion(?Send)]
async fn has_static_initializer(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.io.ObjectStreamClass.hasStaticInitializer(Ljava/lang/Class;)Z")
}

#[async_recursion(?Send)]
async fn init_native(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}
