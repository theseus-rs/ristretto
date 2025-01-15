use crate::arguments::Arguments;
use crate::java_object::JavaObject;
use crate::native_methods::registry::{MethodRegistry, JAVA_11, JAVA_8};
use crate::thread::Thread;
use crate::Error::InternalError;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::{ConcurrentVec, Object, Reference, Value};
use std::sync::Arc;

const CLASS_NAME: &str = "java/lang/Throwable";

/// Register all native methods for `java.lang.Throwable`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= JAVA_8 {
        registry.register(
            CLASS_NAME,
            "getStackTraceDepth",
            "()I",
            get_stack_trace_depth,
        );
        registry.register(
            CLASS_NAME,
            "getStackTraceElement",
            "(I)Ljava/lang/StackTraceElement;",
            get_stack_trace_element,
        );
    }

    registry.register(
        CLASS_NAME,
        "fillInStackTrace",
        "(I)Ljava/lang/Throwable;",
        fill_in_stack_trace,
    );
}

#[async_recursion(?Send)]
async fn fill_in_stack_trace(
    thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Result<Option<Value>> {
    let _dummy = usize::try_from(arguments.pop_int()?)?;
    let object = arguments.pop_reference()?;
    let Some(Reference::Object(ref throwable)) = object else {
        return Err(InternalError("No throwable object found".to_string()));
    };

    let vm = thread.vm()?;
    let stack_element_class = thread.class("java/lang/StackTraceElement").await?;
    let stack_elements = ConcurrentVec::new();
    for frame in thread.frames().await?.iter().rev() {
        let class = frame.class();
        let class_name = class.name();
        if class_name == "java/lang/Throwable" {
            continue;
        }
        let class_name = class_name.to_object(&vm).await?;
        let stack_element_object = Object::new(stack_element_class.clone())?;
        stack_element_object.set_value("declaringClass", class_name)?;

        if let Some(source_file) = class.source_file() {
            let source_file = source_file.to_object(&vm).await?;
            stack_element_object.set_value("fileName", source_file)?;
        }

        let method = frame.method();
        let method_name = method.name().to_object(&vm).await?;
        stack_element_object.set_value("methodName", method_name)?;

        let program_counter = frame.program_counter();
        let line_number = method.line_number(program_counter);
        stack_element_object.set_value("lineNumber", Value::Int(i32::try_from(line_number)?))?;

        stack_elements.push(Some(Reference::Object(stack_element_object)))?;
    }

    let depth = i32::try_from(stack_elements.len()?)?;
    let stack_element_array_class = thread
        .class(format!("[L{stack_element_class};").as_str())
        .await?;
    let stack_trace = Value::Object(Some(Reference::Array(
        stack_element_array_class,
        stack_elements,
    )));
    throwable.set_value("backtrace", stack_trace)?;

    if vm.java_major_version() >= JAVA_11 {
        throwable.set_value("depth", Value::Int(depth))?;
    }

    Ok(Some(Value::Object(object)))
}

#[async_recursion(?Send)]
async fn get_stack_trace_depth(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.lang.Throwable.getStackTraceDepth()I")
}

#[async_recursion(?Send)]
async fn get_stack_trace_element(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.lang.Throwable.getStackTraceElement(I)Ljava/lang/StackTraceElement;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.lang.Throwable.getStackTraceDepth()I")]
    async fn test_get_stack_trace_depth() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_stack_trace_depth(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.Throwable.getStackTraceElement(I)Ljava/lang/StackTraceElement;"
    )]
    async fn test_get_stack_trace_element() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_stack_trace_element(thread, Arguments::default()).await;
    }
}
