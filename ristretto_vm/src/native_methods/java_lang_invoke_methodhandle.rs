use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.lang.invoke.MethodHandle`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/invoke/MethodHandle";
    registry.register(
        class_name,
        "invoke",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        invoke,
    );
    registry.register(
        class_name,
        "invokeBasic",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        invoke_basic,
    );
    registry.register(
        class_name,
        "invokeExact",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        invoke_exact,
    );
    registry.register(
        class_name,
        "linkToInterface",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        link_to_interface,
    );
    registry.register(
        class_name,
        "linkToSpecial",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        link_to_special,
    );
    registry.register(
        class_name,
        "linkToStatic",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        link_to_static,
    );
    registry.register(
        class_name,
        "linkToVirtual",
        "([Ljava/lang/Object;)Ljava/lang/Object;",
        link_to_virtual,
    );
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn invoke(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn invoke_basic(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn invoke_exact(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn link_to_interface(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn link_to_special(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn link_to_static(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn link_to_virtual(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
