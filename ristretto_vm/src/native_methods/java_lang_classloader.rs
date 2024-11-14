use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for java.lang.ClassLoader.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/ClassLoader";
    registry.register(
        class_name,
        "initSystemClassLoader",
        "()Ljava/lang/ClassLoader;",
        init_system_class_loader,
    );
    registry.register(class_name, "registerNatives", "()V", register_natives);
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn init_system_class_loader(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    // TODO: implement this method; temporarily return null to allow the VM to initialize
    Ok(Some(Value::Object(None)))
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn register_natives(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}
