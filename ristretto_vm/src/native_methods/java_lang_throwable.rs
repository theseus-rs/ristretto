use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Error::InternalError;
use crate::Result;
use ristretto_classloader::{ConcurrentVec, Object, Reference, Value};
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

fn fill_in_stack_trace(
    thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Pin<Box<dyn Future<Output = Result<Option<Value>>>>> {
    Box::pin(async move {
        let _dummy = usize::try_from(arguments.pop_int()?)?;
        let object = arguments.pop_object()?;
        let Some(Reference::Object(ref throwable)) = object else {
            return Err(InternalError("No throwable object found".to_string()));
        };

        let vm = thread.vm()?;
        let stack_element_class = vm
            .load_class(&thread, "java/lang/StackTraceElement")
            .await?;
        let stack_elements = ConcurrentVec::new();
        for frame in thread.frames().await?.iter().rev() {
            let class = frame.class();
            let class_name = class.name();
            if class_name == "java/lang/Throwable" {
                continue;
            }
            let class_name = vm.to_string_value(&thread, class_name).await?;
            let stack_element_object = Object::new(stack_element_class.clone())?;
            stack_element_object.set_value("declaringClass", class_name)?;

            if let Some(source_file) = class.source_file() {
                let source_file = vm.to_string_value(&thread, source_file).await?;
                stack_element_object.set_value("fileName", source_file)?;
            }

            let method = frame.method();
            let method_name = vm.to_string_value(&thread, method.name()).await?;
            stack_element_object.set_value("methodName", method_name)?;

            let program_counter = frame.program_counter();
            let line_number = method.line_number(program_counter);
            stack_element_object
                .set_value("lineNumber", Value::Int(i32::try_from(line_number)?))?;

            stack_elements.push(Some(Reference::Object(stack_element_object)))?;
        }

        let depth = i32::try_from(stack_elements.len()?)?;
        let stack_element_array_class = vm
            .load_class(&thread, format!("[L{stack_element_class};").as_str())
            .await?;
        let stack_trace = Value::Object(Some(Reference::Array(
            stack_element_array_class,
            stack_elements,
        )));
        throwable.set_value("backtrace", stack_trace)?;
        throwable.set_value("depth", Value::Int(depth))?;
        Ok(Some(Value::Object(object)))
    })
}
