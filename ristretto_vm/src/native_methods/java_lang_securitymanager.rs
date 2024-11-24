use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.lang.SecurityManager`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/SecurityManager";
    registry.register(
        class_name,
        "classDepth",
        "(Ljava/lang/String;)I",
        class_depth,
    );
    registry.register(class_name, "classLoaderDepth0", "()I", class_loader_depth_0);
    registry.register(
        class_name,
        "currentClassLoader0",
        "()Ljava/lang/ClassLoader;",
        current_class_loader_0,
    );
    registry.register(
        class_name,
        "currentLoadedClass0",
        "()Ljava/lang/Class;",
        current_loaded_class_0,
    );
    registry.register(
        class_name,
        "getClassContext",
        "()[Ljava/lang/Class;",
        get_class_context,
    );
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn class_depth(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn class_loader_depth_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn current_class_loader_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn current_loaded_class_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_class_context(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
