use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.reflect.Reflection`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/reflect/Reflection";
    registry.register(
        class_name,
        "getCallerClass",
        "()Ljava/lang/Class;",
        get_caller_class_1,
    );
    registry.register(
        class_name,
        "getCallerClass",
        "(I)Ljava/lang/Class;",
        get_caller_class_2,
    );
    registry.register(
        class_name,
        "getClassAccessFlags",
        "(Ljava/lang/Class;)I",
        get_class_access_flags,
    );
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_caller_class_1(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_caller_class_2(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_class_access_flags(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}
