use crate::arguments::Arguments;
use crate::call_stack::CallStack;
use crate::native_methods::registry::MethodRegistry;
use crate::Result;
use ristretto_classloader::Value;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Register all native methods for java.lang.Throwable.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/Throwable";
    registry.register(
        class_name,
        "fillInStackTrace",
        "(I)Ljava/lang/Throwable;",
        fill_in_stack_trace,
    );
}

#[expect(clippy::needless_pass_by_value)]
fn fill_in_stack_trace(
    _call_stack: Arc<CallStack>,
    mut arguments: Arguments,
) -> Pin<Box<dyn Future<Output = Result<Option<Value>>>>> {
    Box::pin(async move {
        let _dummy = arguments.pop_int()?;
        let throwable = arguments.pop_object()?;
        // TODO: Implement fillInStackTrace
        Ok(Some(Value::Object(throwable)))
    })
}
