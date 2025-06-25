use crate::Error::InternalError;
use crate::Result;
use crate::java_object::JavaObject;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::{Any, LessThanOrEqual};
use ristretto_classfile::{JAVA_8, JAVA_11};
use ristretto_classloader::{Object, Reference, Value};
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("java/lang/Throwable.fillInStackTrace(I)Ljava/lang/Throwable;", Any)]
#[async_recursion(?Send)]
pub(crate) async fn fill_in_stack_trace(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _dummy = usize::try_from(parameters.pop_int()?)?;
    let object = parameters.pop_reference()?;
    let Some(Reference::Object(ref throwable)) = object else {
        return Err(InternalError("No throwable object found".to_string()));
    };

    let vm = thread.vm()?;
    let stack_element_class = thread.class("java/lang/StackTraceElement").await?;
    let mut stack_elements = Vec::new();
    for frame in thread.frames().await?.iter().rev() {
        let class = frame.class();
        let class_name = class.name();
        if class_name == "java/lang/Throwable" {
            continue;
        }
        let class_name = class_name.to_object(&thread).await?;
        let stack_element_object = Object::new(stack_element_class.clone())?;
        stack_element_object.set_value("declaringClass", class_name)?;

        if let Some(source_file) = class.source_file() {
            let source_file = source_file.to_object(&thread).await?;
            stack_element_object.set_value("fileName", source_file)?;
        }

        let method = frame.method();
        let method_name = method.name().to_object(&thread).await?;
        stack_element_object.set_value("methodName", method_name)?;

        let program_counter = frame.program_counter();
        let line_number = method.line_number(program_counter);
        stack_element_object.set_value("lineNumber", Value::Int(i32::try_from(line_number)?))?;

        stack_elements.push(Value::from(stack_element_object));
    }

    let depth = i32::try_from(stack_elements.len())?;
    let stack_element_array_class = thread
        .class(format!("[L{stack_element_class};").as_str())
        .await?;
    let stack_trace = Value::try_from((stack_element_array_class, stack_elements))?;
    throwable.set_value("backtrace", stack_trace)?;

    if vm.java_major_version() >= JAVA_11.java() {
        throwable.set_value("depth", Value::Int(depth))?;
    }

    Ok(Some(Value::Object(object)))
}

#[intrinsic_method("java/lang/Throwable.getStackTraceDepth()I", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn get_stack_trace_depth(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.Throwable.getStackTraceDepth()I")
}

#[intrinsic_method(
    "java/lang/Throwable.getStackTraceElement(I)Ljava/lang/StackTraceElement;",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_stack_trace_element(
    _thread: Arc<Thread>,
    _parameters: Parameters,
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
        let _ = get_stack_trace_depth(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.Throwable.getStackTraceElement(I)Ljava/lang/StackTraceElement;"
    )]
    async fn test_get_stack_trace_element() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_stack_trace_element(thread, Parameters::default()).await;
    }
}
