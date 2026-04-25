use parking_lot::RwLock;
use ristretto_classfile::VersionSpecification::{Between, GreaterThan, GreaterThanOrEqual};
use ristretto_classfile::{JAVA_11, JAVA_17};
use ristretto_classloader::{Object, Reference, Value};
use ristretto_gc::Gc;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Error::InternalError;
use ristretto_types::JavaError;
use ristretto_types::JavaObject;
use ristretto_types::Thread;
use ristretto_types::VM;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "java/lang/StackTraceElement.initStackTraceElement(Ljava/lang/StackTraceElement;Ljava/lang/StackFrameInfo;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn init_stack_trace_element<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _sfi = parameters.pop_reference()?;
    let _element = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("java.lang.StackTraceElement.initStackTraceElement(Ljava/lang/StackTraceElement;Ljava/lang/StackFrameInfo;)V".to_string()).into())
}

#[intrinsic_method(
    "java/lang/StackTraceElement.initStackTraceElements([Ljava/lang/StackTraceElement;Ljava/lang/Throwable;)V",
    Between(JAVA_11, JAVA_17)
)]
#[async_method]
pub async fn init_stack_trace_elements_0<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    // For Java 11-17: parameters are (stack_trace_elements[], throwable)
    let throwable = parameters.pop()?;

    let (back_trace, depth) = {
        let throwable_ref = throwable.as_object_ref()?;
        let back_trace = throwable_ref.value("backtrace")?;
        let depth = throwable_ref.value("depth")?.as_i32()?;
        (back_trace, depth)
    };

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
#[async_method]
pub async fn init_stack_trace_elements_1<T: Thread + 'static>(
    thread: Arc<T>,
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
async fn init_stack_trace_elements_impl<T: Thread + 'static>(
    thread: Arc<T>,
    stack_trace_ref: Gc<RwLock<Reference>>,
    back_trace: Value,
    depth: usize,
) -> Result<Option<Value>> {
    if back_trace.is_null() {
        return Err(InternalError("No back trace object found".to_string()));
    }

    let stack_element_class = thread.class("java/lang/StackTraceElement").await?;

    for index in 0..depth {
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

        // Get the actual class to find source file and set declaringClassObject
        let actual_class = thread.class(&class_name).await?;
        let declaring_class_object = actual_class.to_object(&thread).await?;
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
        stack_element.set_value("declaringClassObject", declaring_class_object)?;
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
            *element = Value::new_object(
                thread.vm()?.garbage_collector(),
                Reference::Object(stack_element),
            );
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
    use ristretto_classloader::{Object, Reference};
    use ristretto_types::JavaObject;

    /// Create a backtrace Object[] with the given number of frames.
    /// Each frame references "java/lang/Object" class with method "hashCode()I".
    async fn create_backtrace(
        vm: &Arc<impl VM + 'static>,
        thread: &Arc<impl Thread + 'static>,
        count: usize,
    ) -> Result<Value> {
        let object_array_class = thread.class("[Ljava/lang/Object;").await?;
        let integer_class = thread.class("java/lang/Integer").await?;
        let obj_class = thread.class("java/lang/Object").await?;
        let class_obj = obj_class.to_object(thread).await?;

        let mut backtrace_elements = Vec::new();
        for i in 0..count {
            let method_name = "hashCode".to_object(thread).await?;
            let descriptor = "()I".to_object(thread).await?;

            let mut pc_obj = Object::new(integer_class.clone())?;
            pc_obj.set_value("value", Value::Int(i32::try_from(i)?))?;
            let pc_value = Value::new_object(vm.garbage_collector(), Reference::Object(pc_obj));

            let frame_info = vec![class_obj.clone(), method_name, descriptor, pc_value];
            let reference = Reference::try_from((object_array_class.clone(), frame_info))?;
            backtrace_elements.push(Value::new_object(vm.garbage_collector(), reference));
        }

        let reference = Reference::try_from((object_array_class, backtrace_elements))?;
        Ok(Value::new_object(vm.garbage_collector(), reference))
    }

    /// Create a pre-allocated `StackTraceElement[]` array with null entries.
    async fn create_stack_trace_array(
        vm: &Arc<impl VM + 'static>,
        thread: &Arc<impl Thread + 'static>,
        count: usize,
    ) -> Result<Value> {
        let stack_element_array_class = thread.class("[Ljava/lang/StackTraceElement;").await?;
        let elements = vec![Value::Object(None); count];
        let reference = Reference::try_from((stack_element_array_class, elements))?;
        Ok(Value::new_object(vm.garbage_collector(), reference))
    }

    /// Create a Throwable with backtrace and depth fields set.
    async fn create_throwable_with_backtrace(
        vm: &Arc<impl VM + 'static>,
        thread: &Arc<impl Thread + 'static>,
        count: usize,
    ) -> Result<Value> {
        let throwable_class = thread.class("java/lang/Throwable").await?;
        let backtrace = create_backtrace(vm, thread, count).await?;
        let depth = i32::try_from(count)?;

        let mut throwable_object = Object::new(throwable_class)?;
        throwable_object.set_value("backtrace", backtrace)?;
        throwable_object.set_value("depth", Value::Int(depth))?;
        Ok(Value::new_object(
            vm.garbage_collector(),
            Reference::Object(throwable_object),
        ))
    }

    #[tokio::test]
    async fn test_init_stack_trace_element() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_stack_trace_element(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "java.lang.StackTraceElement.initStackTraceElement(Ljava/lang/StackTraceElement;Ljava/lang/StackFrameInfo;)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_init_stack_trace_element_java11() {
        let (_vm, thread) = crate::test::java11_thread().await.expect("thread");
        let result = init_stack_trace_element(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_init_stack_trace_element_java17() {
        let (_vm, thread) = crate::test::java17_thread().await.expect("thread");
        let result = init_stack_trace_element(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_init_stack_trace_element_java21() {
        let (_vm, thread) = crate::test::java21_thread().await.expect("thread");
        let result = init_stack_trace_element(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_init_stack_trace_element_java25() {
        let (_vm, thread) = crate::test::java25_thread().await.expect("thread");
        let result = init_stack_trace_element(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_init_stack_trace_elements_0_empty_java11() -> Result<()> {
        let (vm, thread) = crate::test::java11_thread().await?;
        let stack_trace = create_stack_trace_array(&vm, &thread, 0).await?;
        let throwable = create_throwable_with_backtrace(&vm, &thread, 0).await?;

        let mut parameters = Parameters::default();
        parameters.push(stack_trace);
        parameters.push(throwable);
        let result = init_stack_trace_elements_0(thread, parameters).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_init_stack_trace_elements_0_empty_java17() -> Result<()> {
        let (vm, thread) = crate::test::java17_thread().await?;
        let stack_trace = create_stack_trace_array(&vm, &thread, 0).await?;
        let throwable = create_throwable_with_backtrace(&vm, &thread, 0).await?;

        let mut parameters = Parameters::default();
        parameters.push(stack_trace);
        parameters.push(throwable);
        let result = init_stack_trace_elements_0(thread, parameters).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_init_stack_trace_elements_0_single_frame_java11() -> Result<()> {
        let (vm, thread) = crate::test::java11_thread().await?;
        let stack_trace = create_stack_trace_array(&vm, &thread, 1).await?;
        let throwable = create_throwable_with_backtrace(&vm, &thread, 1).await?;

        let mut parameters = Parameters::default();
        parameters.push(stack_trace.clone());
        parameters.push(throwable);
        let result = init_stack_trace_elements_0(thread, parameters).await?;
        assert_eq!(result, None);

        let stack_trace_guard = stack_trace.as_reference()?;
        let Reference::Array(array) = &*stack_trace_guard else {
            panic!("Expected array");
        };
        assert!(!array.elements[0].is_null());
        let element_ref = array.elements[0].as_object_ref()?;
        let declaring_class = element_ref.value("declaringClass")?.as_string()?;
        assert_eq!(declaring_class, "java.lang.Object");
        Ok(())
    }

    #[tokio::test]
    async fn test_init_stack_trace_elements_0_single_frame_java17() -> Result<()> {
        let (vm, thread) = crate::test::java17_thread().await?;
        let stack_trace = create_stack_trace_array(&vm, &thread, 1).await?;
        let throwable = create_throwable_with_backtrace(&vm, &thread, 1).await?;

        let mut parameters = Parameters::default();
        parameters.push(stack_trace.clone());
        parameters.push(throwable);
        let result = init_stack_trace_elements_0(thread, parameters).await?;
        assert_eq!(result, None);

        let stack_trace_guard = stack_trace.as_reference()?;
        let Reference::Array(array) = &*stack_trace_guard else {
            panic!("Expected array");
        };
        assert!(!array.elements[0].is_null());
        Ok(())
    }

    #[tokio::test]
    async fn test_init_stack_trace_elements_0_null_backtrace() {
        let (vm, thread) = crate::test::java11_thread().await.expect("thread");
        let stack_trace = create_stack_trace_array(&vm, &thread, 1)
            .await
            .expect("stack trace");
        let throwable_class = thread
            .class("java/lang/Throwable")
            .await
            .expect("throwable class");
        let mut throwable_object = Object::new(throwable_class).expect("throwable object");
        throwable_object
            .set_value("backtrace", Value::Object(None))
            .expect("set backtrace");
        throwable_object
            .set_value("depth", Value::Int(1))
            .expect("set depth");
        let throwable =
            Value::new_object(vm.garbage_collector(), Reference::Object(throwable_object));

        let mut parameters = Parameters::default();
        parameters.push(stack_trace);
        parameters.push(throwable);
        let result = init_stack_trace_elements_0(thread, parameters).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_init_stack_trace_elements_0_multiple_frames_java11() -> Result<()> {
        let (vm, thread) = crate::test::java11_thread().await?;
        let count = 3;
        let stack_trace = create_stack_trace_array(&vm, &thread, count).await?;
        let throwable = create_throwable_with_backtrace(&vm, &thread, count).await?;

        let mut parameters = Parameters::default();
        parameters.push(stack_trace.clone());
        parameters.push(throwable);
        let result = init_stack_trace_elements_0(thread, parameters).await?;
        assert_eq!(result, None);

        let stack_trace_guard = stack_trace.as_reference()?;
        let Reference::Array(array) = &*stack_trace_guard else {
            panic!("Expected array");
        };
        for i in 0..count {
            assert!(
                !array.elements[i].is_null(),
                "element {i} should not be null"
            );
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_init_stack_trace_elements_0_null_stack_trace_ref() {
        let (vm, thread) = crate::test::java11_thread().await.expect("thread");
        let throwable = create_throwable_with_backtrace(&vm, &thread, 1)
            .await
            .expect("throwable");

        let mut parameters = Parameters::default();
        parameters.push_reference(None);
        parameters.push(throwable);
        let result = init_stack_trace_elements_0(thread, parameters).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_init_stack_trace_elements_1_empty_java21() -> Result<()> {
        let (vm, thread) = crate::test::java21_thread().await?;
        let stack_trace = create_stack_trace_array(&vm, &thread, 0).await?;
        let backtrace = create_backtrace(&vm, &thread, 0).await?;

        let mut parameters = Parameters::default();
        parameters.push(stack_trace);
        parameters.push(backtrace);
        parameters.push(Value::Int(0));
        let result = init_stack_trace_elements_1(thread, parameters).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_init_stack_trace_elements_1_empty_java25() -> Result<()> {
        let (vm, thread) = crate::test::java25_thread().await?;
        let stack_trace = create_stack_trace_array(&vm, &thread, 0).await?;
        let backtrace = create_backtrace(&vm, &thread, 0).await?;

        let mut parameters = Parameters::default();
        parameters.push(stack_trace);
        parameters.push(backtrace);
        parameters.push(Value::Int(0));
        let result = init_stack_trace_elements_1(thread, parameters).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_init_stack_trace_elements_1_single_frame_java21() -> Result<()> {
        let (vm, thread) = crate::test::java21_thread().await?;
        let stack_trace = create_stack_trace_array(&vm, &thread, 1).await?;
        let backtrace = create_backtrace(&vm, &thread, 1).await?;

        let mut parameters = Parameters::default();
        parameters.push(stack_trace.clone());
        parameters.push(backtrace);
        parameters.push(Value::Int(1));
        let result = init_stack_trace_elements_1(thread, parameters).await?;
        assert_eq!(result, None);

        let stack_trace_guard = stack_trace.as_reference()?;
        let Reference::Array(array) = &*stack_trace_guard else {
            panic!("Expected array");
        };
        assert!(!array.elements[0].is_null());
        let element_ref = array.elements[0].as_object_ref()?;
        let declaring_class = element_ref.value("declaringClass")?.as_string()?;
        assert_eq!(declaring_class, "java.lang.Object");
        let method_name = element_ref.value("methodName")?.as_string()?;
        assert_eq!(method_name, "hashCode");
        Ok(())
    }

    #[tokio::test]
    async fn test_init_stack_trace_elements_1_single_frame_java25() -> Result<()> {
        let (vm, thread) = crate::test::java25_thread().await?;
        let stack_trace = create_stack_trace_array(&vm, &thread, 1).await?;
        let backtrace = create_backtrace(&vm, &thread, 1).await?;

        let mut parameters = Parameters::default();
        parameters.push(stack_trace.clone());
        parameters.push(backtrace);
        parameters.push(Value::Int(1));
        let result = init_stack_trace_elements_1(thread, parameters).await?;
        assert_eq!(result, None);

        let stack_trace_guard = stack_trace.as_reference()?;
        let Reference::Array(array) = &*stack_trace_guard else {
            panic!("Expected array");
        };
        assert!(!array.elements[0].is_null());
        Ok(())
    }

    #[tokio::test]
    async fn test_init_stack_trace_elements_1_null_backtrace() {
        let (vm, thread) = crate::test::java21_thread().await.expect("thread");
        let stack_trace = create_stack_trace_array(&vm, &thread, 1)
            .await
            .expect("stack trace");

        let mut parameters = Parameters::default();
        parameters.push(stack_trace);
        parameters.push(Value::Object(None));
        parameters.push(Value::Int(1));
        let result = init_stack_trace_elements_1(thread, parameters).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_init_stack_trace_elements_1_multiple_frames_java21() -> Result<()> {
        let (vm, thread) = crate::test::java21_thread().await?;
        let count = 3;
        let stack_trace = create_stack_trace_array(&vm, &thread, count).await?;
        let backtrace = create_backtrace(&vm, &thread, count).await?;

        let mut parameters = Parameters::default();
        parameters.push(stack_trace.clone());
        parameters.push(backtrace);
        parameters.push(Value::Int(i32::try_from(count)?));
        let result = init_stack_trace_elements_1(thread, parameters).await?;
        assert_eq!(result, None);

        let stack_trace_guard = stack_trace.as_reference()?;
        let Reference::Array(array) = &*stack_trace_guard else {
            panic!("Expected array");
        };
        for i in 0..count {
            assert!(
                !array.elements[i].is_null(),
                "element {i} should not be null"
            );
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_init_stack_trace_elements_1_null_stack_trace_ref() {
        let (vm, thread) = crate::test::java21_thread().await.expect("thread");
        let backtrace = create_backtrace(&vm, &thread, 1).await.expect("backtrace");

        let mut parameters = Parameters::default();
        parameters.push_reference(None);
        parameters.push(backtrace);
        parameters.push(Value::Int(1));
        let result = init_stack_trace_elements_1(thread, parameters).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_init_stack_trace_elements_1_verifies_fields_java21() -> Result<()> {
        let (vm, thread) = crate::test::java21_thread().await?;
        let stack_trace = create_stack_trace_array(&vm, &thread, 1).await?;
        let backtrace = create_backtrace(&vm, &thread, 1).await?;

        let mut parameters = Parameters::default();
        parameters.push(stack_trace.clone());
        parameters.push(backtrace);
        parameters.push(Value::Int(1));
        let result = init_stack_trace_elements_1(thread, parameters).await?;
        assert_eq!(result, None);

        let stack_trace_guard = stack_trace.as_reference()?;
        let Reference::Array(array) = &*stack_trace_guard else {
            panic!("Expected array");
        };
        let element_ref = array.elements[0].as_object_ref()?;

        // Verify declaringClass is set
        let declaring_class = element_ref.value("declaringClass")?.as_string()?;
        assert_eq!(declaring_class, "java.lang.Object");

        // Verify declaringClassObject is not null
        let declaring_class_obj = element_ref.value("declaringClassObject")?;
        assert!(!declaring_class_obj.is_null());

        // Verify methodName is set
        let method_name = element_ref.value("methodName")?.as_string()?;
        assert_eq!(method_name, "hashCode");

        // Verify lineNumber is set (may be -1 for native/synthetic methods)
        let line_number = element_ref.value("lineNumber")?.as_i32()?;
        assert!(
            line_number >= -1,
            "line number should be >= -1, got {line_number}"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_init_stack_trace_elements_0_verifies_fields_java11() -> Result<()> {
        let (vm, thread) = crate::test::java11_thread().await?;
        let stack_trace = create_stack_trace_array(&vm, &thread, 1).await?;
        let throwable = create_throwable_with_backtrace(&vm, &thread, 1).await?;

        let mut parameters = Parameters::default();
        parameters.push(stack_trace.clone());
        parameters.push(throwable);
        let result = init_stack_trace_elements_0(thread, parameters).await?;
        assert_eq!(result, None);

        let stack_trace_guard = stack_trace.as_reference()?;
        let Reference::Array(array) = &*stack_trace_guard else {
            panic!("Expected array");
        };
        let element_ref = array.elements[0].as_object_ref()?;

        // Verify declaringClass
        let declaring_class = element_ref.value("declaringClass")?.as_string()?;
        assert_eq!(declaring_class, "java.lang.Object");

        // Verify declaringClassObject is not null
        let declaring_class_obj = element_ref.value("declaringClassObject")?;
        assert!(!declaring_class_obj.is_null());

        // Verify methodName
        let method_name = element_ref.value("methodName")?.as_string()?;
        assert_eq!(method_name, "hashCode");

        // Verify lineNumber
        let line_number = element_ref.value("lineNumber")?.as_i32()?;
        assert!(
            line_number >= -1,
            "line number should be >= -1, got {line_number}"
        );

        Ok(())
    }
}
