use crate::arguments::Arguments;
use crate::call_stack::CallStack;
use crate::native_methods::registry::MethodRegistry;
use crate::Error::RuntimeError;
use crate::Result;
use ristretto_classloader::{Reference, Value};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Register all native methods for java.lang.StackTraceElement.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/StackTraceElement";
    registry.register(
        class_name,
        "initStackTraceElements",
        "([Ljava/lang/StackTraceElement;Ljava/lang/Object;I)V",
        init_stack_trace_elements,
    );
}

#[expect(clippy::needless_pass_by_value)]
fn init_stack_trace_elements(
    _call_stack: Arc<CallStack>,
    mut arguments: Arguments,
) -> Pin<Box<dyn Future<Output = Result<Option<Value>>>>> {
    Box::pin(async move {
        let depth = usize::try_from(arguments.pop_int()?)?;
        let Some(Reference::Array(_class, back_trace)) = arguments.pop_object()? else {
            return Err(RuntimeError("No back trace object found".to_string()));
        };
        let Some(Reference::Array(_class, stack_trace)) = arguments.pop_object()? else {
            return Err(RuntimeError("No stack trace object found".to_string()));
        };
        for index in 0..depth {
            let Some(value) = back_trace.get(index)? else {
                return Err(RuntimeError("No back trace element found".to_string()));
            };
            stack_trace.set(index, value)?;
        }
        Ok(None)
    })
}
