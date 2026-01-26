use crate::Result;
use crate::java_object::JavaObject;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::VersionSpecification::{Any, LessThanOrEqual};
use ristretto_classfile::{JAVA_8, JAVA_11};
use ristretto_classloader::{Object, Reference, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("java/lang/Throwable.fillInStackTrace(I)Ljava/lang/Throwable;", Any)]
#[async_method]
pub(crate) async fn fill_in_stack_trace(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _dummy = usize::try_from(parameters.pop_int()?)?;
    let throwable = parameters.pop()?;
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
        let mut stack_element_object = Object::new(stack_element_class.clone())?;
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

        stack_elements.push(Value::new_object(
            vm.garbage_collector(),
            Reference::Object(stack_element_object),
        ));
    }

    let stack_element_array_class = thread
        .class(format!("[L{stack_element_class};").as_str())
        .await?;
    let reference = Reference::try_from((stack_element_array_class, stack_elements))?;
    let stack_trace = Value::new_object(vm.garbage_collector(), reference);

    // Create the backtrace
    let _object_class = thread.class("java/lang/Object").await?;
    let object_array_class = thread.class("[Ljava/lang/Object;").await?;
    let integer_class = thread.class("java/lang/Integer").await?;
    let mut backtrace_elements = Vec::new();

    let throwable_class = {
        let obj = throwable.as_object_ref()?;
        obj.class().clone()
    };
    let mut skipping = true;

    for frame in thread.frames().await?.iter().rev() {
        let class = frame.class();
        let method = frame.method();
        let method_name_str = method.name();

        if skipping {
            if method_name_str == "fillInStackTrace" {
                continue;
            }
            if method_name_str == "<init>"
                && (Arc::ptr_eq(class, &throwable_class)
                    || throwable_class.is_subclass_of(class)?)
            {
                continue;
            }
            if class.name() == "java/lang/Throwable" {
                continue;
            }
            skipping = false;
        }

        let method_name = method_name_str.to_object(&thread).await?;
        let method_descriptor = method.descriptor().to_object(&thread).await?;
        let program_counter = frame.program_counter();
        let mut program_counter_value = Object::new(integer_class.clone())?;
        program_counter_value.set_value("value", Value::Int(i32::try_from(program_counter)?))?;

        let frame_info = vec![
            class.to_object(&thread).await?,
            method_name,
            method_descriptor,
            Value::new_object(
                vm.garbage_collector(),
                Reference::Object(program_counter_value),
            ),
        ];
        let reference = Reference::try_from((object_array_class.clone(), frame_info))?;
        let frame_info_array = Value::new_object(vm.garbage_collector(), reference);
        backtrace_elements.push(frame_info_array);
    }
    let reference = Reference::try_from((object_array_class, backtrace_elements.clone()))?;
    let backtrace = Value::new_object(vm.garbage_collector(), reference);
    let backtrace_depth = i32::try_from(backtrace_elements.len())?;

    {
        let mut throwable = throwable.as_object_mut()?;
        // Store standard stack trace in the public field if possible (standard JDK). Ignoring error
        // if field doesn't exist to maintain compatibility with varying JDKs
        let _ = throwable.set_value("stackTrace", stack_trace.clone());
        throwable.set_value("backtrace", backtrace)?;

        if vm.java_major_version() >= JAVA_11.java() {
            throwable.set_value("depth", Value::Int(backtrace_depth))?;
        }
    }

    Ok(Some(throwable))
}

#[intrinsic_method("java/lang/Throwable.getStackTraceDepth()I", LessThanOrEqual(JAVA_8))]
#[async_method]
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
#[async_method]
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
