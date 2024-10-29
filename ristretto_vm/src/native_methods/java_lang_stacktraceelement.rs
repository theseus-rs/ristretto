use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Error::InternalError;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::{Reference, Value};
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
#[async_recursion(?Send)]
async fn init_stack_trace_elements(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let depth = usize::try_from(arguments.pop_int()?)?;
    let Some(Reference::Array(_class, back_trace)) = arguments.pop_object()? else {
        return Err(InternalError("No back trace object found".to_string()));
    };
    let Some(Reference::Array(_class, stack_trace)) = arguments.pop_object()? else {
        return Err(InternalError("No stack trace object found".to_string()));
    };
    for index in 0..depth {
        let Some(value) = back_trace.get(index)? else {
            return Err(InternalError("No back trace element found".to_string()));
        };
        stack_trace.set(index, value)?;
    }
    Ok(None)
}
