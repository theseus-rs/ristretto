use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.lang.ClassLoader$NativeLibrary`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/ClassLoader$NativeLibrary";
    registry.register(class_name, "find", "(Ljava/lang/String;)J", find);
    registry.register(class_name, "load", "(Ljava/lang/String;Z)V", load);
    registry.register(class_name, "unload", "(Ljava/lang/String;Z)V", unload);
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn find(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn load(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn unload(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
