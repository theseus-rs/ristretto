use crate::arguments::Arguments;
use crate::call_stack::CallStack;
use crate::native_methods::registry::MethodRegistry;
use crate::Error::RuntimeError;
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
    call_stack: Arc<CallStack>,
    mut arguments: Arguments,
) -> Pin<Box<dyn Future<Output = Result<Option<Value>>>>> {
    Box::pin(async move {
        let _dummy = usize::try_from(arguments.pop_int()?)?;
        let object = arguments.pop_object()?;
        let Some(Reference::Object(ref throwable)) = object else {
            return Err(RuntimeError("No throwable object found".to_string()));
        };

        let vm = call_stack.vm()?;
        let stack_element_class = vm
            .load_class(&call_stack, "java/lang/StackTraceElement")
            .await?;
        let stack_elements = ConcurrentVec::new();
        for frame in call_stack.frames()?.iter().rev() {
            let class = frame.class();
            let class_name = class.name();
            if class_name == "java/lang/Throwable" {
                continue;
            }
            let class_name = vm.to_string_value(&call_stack, class_name).await?;
            let stack_element_object = Object::new(stack_element_class.clone())?;
            stack_element_object
                .field("declaringClass")?
                .set_value(class_name)?;

            if let Some(source_file) = class.source_file() {
                let source_file = vm.to_string_value(&call_stack, source_file).await?;
                stack_element_object
                    .field("fileName")?
                    .set_value(source_file)?;
            }

            let method = frame.method();
            let method_name = vm.to_string_value(&call_stack, method.name()).await?;
            stack_element_object
                .field("methodName")?
                .set_value(method_name)?;

            let program_counter = frame.program_counter();
            let line_number = method.line_number(program_counter);
            stack_element_object
                .field("lineNumber")?
                .set_value(Value::Int(i32::try_from(line_number)?))?;

            stack_elements.push(Some(Reference::Object(stack_element_object)))?;
        }

        let depth = i32::try_from(stack_elements.len()?)?;
        let stack_trace =
            Value::Object(Some(Reference::Array(stack_element_class, stack_elements)));
        throwable.field("backtrace")?.set_value(stack_trace)?;
        throwable.field("depth")?.set_value(Value::Int(depth))?;
        Ok(Some(Value::Object(object)))
    })
}
