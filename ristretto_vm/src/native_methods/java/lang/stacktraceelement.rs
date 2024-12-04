use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Error::InternalError;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::{Reference, Value};
use std::sync::Arc;

const JAVA_18: Version = Version::Java18 { minor: 0 };

/// Register all native methods for `java.lang.StackTraceElement`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/StackTraceElement";
    let java_version = registry.java_version();

    if java_version <= &JAVA_18 {
        registry.register(
            class_name,
            "initStackTraceElements",
            "([Ljava/lang/StackTraceElement;Ljava/lang/Throwable;)V",
            init_stack_trace_elements,
        );
    } else {
        registry.register(
            class_name,
            "initStackTraceElements",
            "([Ljava/lang/StackTraceElement;Ljava/lang/Object;I)V",
            init_stack_trace_elements,
        );
    }

    registry.register(
        class_name,
        "initStackTraceElement",
        "(Ljava/lang/StackTraceElement;Ljava/lang/StackFrameInfo;)V",
        init_stack_trace_element,
    );
}

#[async_recursion(?Send)]
async fn init_stack_trace_element(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn init_stack_trace_elements(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let depth = usize::try_from(arguments.pop_int()?)?;
    let Some(Reference::Array(_class, back_trace)) = arguments.pop_reference()? else {
        return Err(InternalError("No back trace object found".to_string()));
    };
    let Some(Reference::Array(_class, stack_trace)) = arguments.pop_reference()? else {
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
