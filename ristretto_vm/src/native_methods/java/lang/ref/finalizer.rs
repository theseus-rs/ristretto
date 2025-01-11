use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.lang.ref.Finalizer`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/ref/Finalizer";
    registry.register(
        class_name,
        "isFinalizationEnabled",
        "()Z",
        is_finalization_enabled,
    );
    registry.register(
        class_name,
        "reportComplete",
        "(Ljava/lang/Object;)V",
        report_complete,
    );
}

#[async_recursion(?Send)]
async fn is_finalization_enabled(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    Ok(Some(Value::from(false)))
}

#[async_recursion(?Send)]
async fn report_complete(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}
