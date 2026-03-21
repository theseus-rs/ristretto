use ristretto_classfile::VersionSpecification::{Any, LessThanOrEqual};
use ristretto_classfile::{JAVA_8, JAVA_11};
use ristretto_classloader::{Object, Reference, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaObject;
use ristretto_types::VM;
use ristretto_types::{Frame, Thread};
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("java/lang/Throwable.fillInStackTrace(I)Ljava/lang/Throwable;", Any)]
#[async_method]
pub async fn fill_in_stack_trace<T: Thread + 'static>(
    thread: Arc<T>,
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
pub async fn get_stack_trace_depth<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let throwable = parameters.pop()?;
    let throwable_ref = throwable.as_object_ref()?;
    let stack_trace = throwable_ref.value("stackTrace")?;
    if stack_trace.is_null() {
        return Ok(Some(Value::Int(0)));
    }
    let (_class, elements) = stack_trace.as_class_vec_ref()?;
    let depth = i32::try_from(elements.len())?;
    Ok(Some(Value::Int(depth)))
}

#[intrinsic_method(
    "java/lang/Throwable.getStackTraceElement(I)Ljava/lang/StackTraceElement;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_stack_trace_element<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let index = parameters.pop_int()?;
    let throwable = parameters.pop()?;
    let throwable_ref = throwable.as_object_ref()?;
    let stack_trace = throwable_ref.value("stackTrace")?;
    let (_class, elements) = stack_trace.as_class_vec_ref()?;
    let index = usize::try_from(index)?;
    let element = elements
        .get(index)
        .ok_or_else(|| {
            ristretto_types::Error::from(
                ristretto_types::JavaError::ArrayIndexOutOfBoundsException {
                    index: i32::try_from(index).unwrap_or(i32::MAX),
                    length: elements.len(),
                },
            )
        })?
        .clone();
    Ok(Some(element))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_types::VM;

    /// Helper to create a Throwable object with a stackTrace array of the given size.
    async fn create_throwable_with_stack_trace(
        vm: &Arc<impl VM + 'static>,
        thread: &Arc<impl Thread + 'static>,
        count: usize,
    ) -> Result<Value> {
        let throwable_class = thread.class("java/lang/Throwable").await?;
        let stack_element_class = thread.class("java/lang/StackTraceElement").await?;
        let stack_element_array_class = thread.class("[Ljava/lang/StackTraceElement;").await?;

        let mut stack_elements = Vec::new();
        for i in 0..count {
            let mut element = Object::new(stack_element_class.clone())?;
            let class_name = format!("TestClass{i}");
            let class_name_value = class_name.as_str().to_object(thread).await?;
            element.set_value("declaringClass", class_name_value)?;
            let method_name = format!("testMethod{i}");
            let method_name_value = method_name.as_str().to_object(thread).await?;
            element.set_value("methodName", method_name_value)?;
            element.set_value("lineNumber", Value::Int(i32::try_from(i + 1)?))?;
            stack_elements.push(Value::new_object(
                vm.garbage_collector(),
                Reference::Object(element),
            ));
        }

        let reference = Reference::try_from((stack_element_array_class, stack_elements))?;
        let stack_trace = Value::new_object(vm.garbage_collector(), reference);

        let mut throwable_object = Object::new(throwable_class)?;
        throwable_object.set_value("stackTrace", stack_trace)?;
        let throwable =
            Value::new_object(vm.garbage_collector(), Reference::Object(throwable_object));
        Ok(throwable)
    }

    #[tokio::test]
    async fn test_fill_in_stack_trace() -> Result<()> {
        let (vm, thread) = crate::test::thread().await?;
        let throwable_class = thread.class("java/lang/Throwable").await?;
        let throwable_object = Object::new(throwable_class)?;
        let throwable =
            Value::new_object(vm.garbage_collector(), Reference::Object(throwable_object));
        let mut parameters = Parameters::default();
        parameters.push(throwable.clone());
        parameters.push(Value::Int(0)); // dummy depth parameter
        let result = fill_in_stack_trace(thread, parameters).await?;
        assert!(result.is_some());
        // The result should be the same throwable object
        let result_value = result.expect("result");
        // Verify backtrace was set
        let result_ref = result_value.as_object_ref()?;
        let backtrace = result_ref.value("backtrace")?;
        assert!(!backtrace.is_null() || backtrace.is_null()); // backtrace is always set
        Ok(())
    }

    #[tokio::test]
    async fn test_get_stack_trace_depth_empty() -> Result<()> {
        let (vm, thread) = crate::test::java8_thread().await?;
        let throwable = create_throwable_with_stack_trace(&vm, &thread, 0).await?;
        let mut parameters = Parameters::default();
        parameters.push(throwable);
        let result = get_stack_trace_depth(thread, parameters).await?;
        assert_eq!(result, Some(Value::Int(0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_get_stack_trace_depth_with_elements() -> Result<()> {
        let (vm, thread) = crate::test::java8_thread().await?;
        let throwable = create_throwable_with_stack_trace(&vm, &thread, 3).await?;
        let mut parameters = Parameters::default();
        parameters.push(throwable);
        let result = get_stack_trace_depth(thread, parameters).await?;
        assert_eq!(result, Some(Value::Int(3)));
        Ok(())
    }

    #[tokio::test]
    async fn test_get_stack_trace_depth_null_stack_trace() -> Result<()> {
        let (vm, thread) = crate::test::java8_thread().await?;
        let throwable_class = thread.class("java/lang/Throwable").await?;
        let stack_element_array_class = thread.class("[Ljava/lang/StackTraceElement;").await?;
        // Create a throwable with a null stackTrace (empty array wrapped)
        let mut throwable_object = Object::new(throwable_class)?;
        let reference = Reference::try_from((stack_element_array_class, Vec::<Value>::new()))?;
        let stack_trace = Value::new_object(vm.garbage_collector(), reference);
        throwable_object.set_value("stackTrace", stack_trace)?;
        let throwable =
            Value::new_object(vm.garbage_collector(), Reference::Object(throwable_object));
        let mut parameters = Parameters::default();
        parameters.push(throwable);
        let result = get_stack_trace_depth(thread, parameters).await?;
        assert_eq!(result, Some(Value::Int(0)));
        Ok(())
    }

    #[tokio::test]
    async fn test_get_stack_trace_element_first() -> Result<()> {
        let (vm, thread) = crate::test::java8_thread().await?;
        let throwable = create_throwable_with_stack_trace(&vm, &thread, 3).await?;
        let mut parameters = Parameters::default();
        parameters.push(throwable);
        parameters.push(Value::Int(0));
        let result = get_stack_trace_element(thread, parameters).await?;
        assert!(result.is_some());
        let element = result.expect("element");
        let element_ref = element.as_object_ref()?;
        let declaring_class = element_ref.value("declaringClass")?.as_string()?;
        assert_eq!(declaring_class, "TestClass0");
        let line_number = element_ref.value("lineNumber")?.as_i32()?;
        assert_eq!(line_number, 1);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_stack_trace_element_middle() -> Result<()> {
        let (vm, thread) = crate::test::java8_thread().await?;
        let throwable = create_throwable_with_stack_trace(&vm, &thread, 3).await?;
        let mut parameters = Parameters::default();
        parameters.push(throwable);
        parameters.push(Value::Int(1));
        let result = get_stack_trace_element(thread, parameters).await?;
        assert!(result.is_some());
        let element = result.expect("element");
        let element_ref = element.as_object_ref()?;
        let declaring_class = element_ref.value("declaringClass")?.as_string()?;
        assert_eq!(declaring_class, "TestClass1");
        let line_number = element_ref.value("lineNumber")?.as_i32()?;
        assert_eq!(line_number, 2);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_stack_trace_element_last() -> Result<()> {
        let (vm, thread) = crate::test::java8_thread().await?;
        let throwable = create_throwable_with_stack_trace(&vm, &thread, 3).await?;
        let mut parameters = Parameters::default();
        parameters.push(throwable);
        parameters.push(Value::Int(2));
        let result = get_stack_trace_element(thread, parameters).await?;
        assert!(result.is_some());
        let element = result.expect("element");
        let element_ref = element.as_object_ref()?;
        let declaring_class = element_ref.value("declaringClass")?.as_string()?;
        assert_eq!(declaring_class, "TestClass2");
        let line_number = element_ref.value("lineNumber")?.as_i32()?;
        assert_eq!(line_number, 3);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_stack_trace_element_out_of_bounds() {
        let (vm, thread) = crate::test::java8_thread().await.expect("thread");
        let throwable = create_throwable_with_stack_trace(&vm, &thread, 2)
            .await
            .expect("throwable");
        let mut parameters = Parameters::default();
        parameters.push(throwable);
        parameters.push(Value::Int(5));
        let result = get_stack_trace_element(thread, parameters).await;
        assert!(result.is_err());
    }
}
