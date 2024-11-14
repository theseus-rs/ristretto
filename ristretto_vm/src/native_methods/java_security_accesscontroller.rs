use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for java.security.AccessController.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/security/AccessController";
    registry.register(
        class_name,
        "ensureMaterializedForStackWalk",
        "(Ljava/lang/Object;)V",
        ensure_materialized_for_stack_walk,
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
async fn ensure_materialized_for_stack_walk(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    Ok(None)
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_stack_access_control_context(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    Ok(Some(Value::Object(None)))
}
