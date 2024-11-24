use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.security.AccessController`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/security/AccessController";
    registry.register(
        class_name,
        "doPrivileged",
        "(Ljava/security/PrivilegedAction;)Ljava/lang/Object;",
        do_privileged_1,
    );
    registry.register(
        class_name,
        "doPrivileged",
        "(Ljava/security/PrivilegedAction;Ljava/security/AccessControlContext;)Ljava/lang/Object;",
        do_privileged_2,
    );
    registry.register(
        class_name,
        "doPrivileged",
        "(Ljava/security/PrivilegedExceptionAction;)Ljava/lang/Object;",
        do_privileged_3,
    );
    registry.register(class_name, "doPrivileged", "(Ljava/security/PrivilegedExceptionAction;Ljava/security/AccessControlContext;)Ljava/lang/Object;", do_privileged_4);
    registry.register(
        class_name,
        "getInheritedAccessControlContext",
        "()Ljava/security/AccessControlContext;",
        get_inherited_access_control_context,
    );
    registry.register(
        class_name,
        "getStackAccessControlContext",
        "()Ljava/security/AccessControlContext;",
        get_stack_access_control_context,
    );
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn do_privileged_1(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn do_privileged_2(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn do_privileged_3(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn do_privileged_4(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_inherited_access_control_context(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_stack_access_control_context(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    Ok(Some(Value::Object(None)))
}
