use crate::Error::InternalError;
use crate::Result;
use crate::java_object::JavaObject;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use parking_lot::RwLock;
use ristretto_classfile::VersionSpecification::{Between, GreaterThan, GreaterThanOrEqual};
use ristretto_classfile::{JAVA_11, JAVA_17};
use ristretto_classloader::{Object, Reference, Value};
use ristretto_gc::Gc;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "java/lang/StackTraceElement.initStackTraceElement(Ljava/lang/StackTraceElement;Ljava/lang/StackFrameInfo;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn init_stack_trace_element(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "java.lang.StackTraceElement.initStackTraceElement(Ljava/lang/StackTraceElement;Ljava/lang/StackFrameInfo;)V"
    )
}

#[intrinsic_method(
    "java/lang/StackTraceElement.initStackTraceElements([Ljava/lang/StackTraceElement;Ljava/lang/Throwable;)V",
    Between(JAVA_11, JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn init_stack_trace_elements_0(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    // For Java 11-17: parameters are (stack_trace_elements[], throwable)
    let throwable = parameters.pop()?;
    let throwable_ref = throwable.as_object_ref()?;
    let back_trace = throwable_ref.value("backtrace")?;
    let depth = throwable_ref.value("depth")?.as_i32()?;

    let Some(stack_trace_ref) = parameters.pop_reference()? else {
        return Err(InternalError("No stack trace object found".to_string()));
    };

    let depth = usize::try_from(depth)?;
    init_stack_trace_elements_impl(thread, stack_trace_ref, back_trace, depth).await
}

#[intrinsic_method(
    "java/lang/StackTraceElement.initStackTraceElements([Ljava/lang/StackTraceElement;Ljava/lang/Object;I)V",
    GreaterThan(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn init_stack_trace_elements_1(
    thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    // For Java 17+: parameters are (stack_trace_elements[], backtrace, depth)
    let depth = usize::try_from(parameters.pop_int()?)?;
    let back_trace = parameters.pop()?;
    let Some(stack_trace_ref) = parameters.pop_reference()? else {
        return Err(InternalError("No stack trace object found".to_string()));
    };

    init_stack_trace_elements_impl(thread, stack_trace_ref, back_trace, depth).await
}

/// Common implementation for `init_stack_trace_elements`.
///
/// The backtrace is an Object[] where each element is an Object[] containing:
/// \[0\] = Class object
/// \[1\] = method name (String)
/// \[2\] = method descriptor (String)
/// \[3\] = Integer (program counter/BCI)
#[expect(clippy::too_many_lines)]
async fn init_stack_trace_elements_impl(
    thread: Arc<Thread>,
    stack_trace_ref: Gc<RwLock<Reference>>,
    back_trace: Value,
    depth: usize,
) -> Result<Option<Value>> {
    if back_trace.is_null() {
        return Err(InternalError("No back trace object found".to_string()));
    }

    let stack_element_class = thread.class("java/lang/StackTraceElement").await?;

    for index in 0..depth {
        // Extract all frame data from backtrace before any await points
        let (class_name, method_name_value, descriptor_value, pc_int) = {
            let back_trace_guard = back_trace.as_reference()?;
            let Reference::Array(back_trace_array) = &*back_trace_guard else {
                return Err(InternalError(
                    "Back trace object is not an array".to_string(),
                ));
            };
            let back_trace_elements = &back_trace_array.elements;
            let Some(frame_info) = back_trace_elements.get(index) else {
                return Err(InternalError(format!(
                    "No back trace element found at index {index}, array len = {}",
                    back_trace_elements.len()
                )));
            };

            if frame_info.is_null() {
                return Err(InternalError(format!(
                    "Frame info at index {index} is null"
                )));
            }

            // Each frame_info is an Object[] with [class, method_name, descriptor, pc]
            let frame_guard = frame_info.as_reference()?;
            let Reference::Array(frame_array) = &*frame_guard else {
                return Err(InternalError("Frame info is not an array".to_string()));
            };

            // Extract class info
            let class_value = frame_array
                .elements
                .first()
                .ok_or_else(|| InternalError("Frame info missing class".to_string()))?
                .clone();

            if class_value.is_null() {
                return Err(InternalError("Class value is null".to_string()));
            }

            let class_guard = class_value.as_reference()?;
            let Reference::Object(class_obj) = &*class_guard else {
                return Err(InternalError("Class value is not an object".to_string()));
            };

            // Get the class name from the Class object
            let class_name_value = class_obj.value("name")?;
            let class_name = class_name_value.as_string()?;

            // Extract method name
            let method_name_value = frame_array
                .elements
                .get(1)
                .ok_or_else(|| InternalError("Frame info missing method name".to_string()))?
                .clone();

            // Extract descriptor
            let descriptor_value = frame_array
                .elements
                .get(2)
                .ok_or_else(|| InternalError("Frame info missing descriptor".to_string()))?
                .clone();

            // Extract program counter value
            let pc_value = frame_array
                .elements
                .get(3)
                .ok_or_else(|| InternalError("Frame info missing program counter".to_string()))?
                .clone();

            let pc_int = if pc_value.is_null() {
                None
            } else {
                let pc_guard = pc_value.as_reference()?;
                let Reference::Object(pc_obj) = &*pc_guard else {
                    return Err(InternalError("PC value is not an object".to_string()));
                };
                Some(pc_obj.value("value")?.as_i32()?)
            };

            (class_name, method_name_value, descriptor_value, pc_int)
        };

        // Now we can do async operations without holding any guards
        let class_name_obj = class_name.to_object(&thread).await?;

        // Get the actual class to find source file
        let actual_class = thread.class(&class_name).await?;
        let source_file = if let Some(sf) = actual_class.source_file() {
            sf.to_object(&thread).await?
        } else {
            Value::Object(None)
        };

        // Get method to find line number
        let method_name_str = method_name_value.as_string()?;
        let descriptor = descriptor_value.as_string()?;

        // Get line number from the method
        let line_number = if let Some(method) = actual_class.method(&method_name_str, &descriptor) {
            if let Some(pc) = pc_int {
                let pc_usize = usize::try_from(pc)?;
                i32::try_from(method.line_number(pc_usize))?
            } else {
                -1
            }
        } else {
            -1
        };

        // Create StackTraceElement
        let mut stack_element = Object::new(stack_element_class.clone())?;
        stack_element.set_value("declaringClass", class_name_obj)?;
        stack_element.set_value("methodName", method_name_value)?;
        stack_element.set_value("fileName", source_file)?;
        stack_element.set_value("lineNumber", Value::Int(line_number))?;

        // Store in stack trace array
        let mut stack_trace_guard = stack_trace_ref.write();
        let Reference::Array(stack_trace_array) = &mut *stack_trace_guard else {
            return Err(InternalError(
                "Stack trace object is not an array".to_string(),
            ));
        };

        let stack_trace_elements = &mut stack_trace_array.elements;
        if let Some(element) = stack_trace_elements.get_mut(index) {
            *element = Value::from(stack_element);
        } else {
            return Err(InternalError(format!(
                "Stack trace array too small: index {index}, len {}",
                stack_trace_elements.len()
            )));
        }
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.StackTraceElement.initStackTraceElement(Ljava/lang/StackTraceElement;Ljava/lang/StackFrameInfo;)V"
    )]
    async fn test_init_stack_trace_element() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_stack_trace_element(thread, Parameters::default()).await;
    }
}
