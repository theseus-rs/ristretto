use crate::Error::{InternalError, JavaError, Throwable};
use crate::frame::{ExecutionResult, Frame};
use crate::operand_stack::OperandStack;
use crate::{Error, Result, VM};
use ristretto_classloader::{Object, Reference};
use std::sync::Arc;

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.athrow>
#[inline]
pub(crate) async fn athrow(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let Some(Reference::Object(throwable)) = stack.pop_object()? else {
        return Err(InternalError("Expected object on top of stack".to_string()));
    };
    // Return the exception to the caller and let the frame error handler deal with it
    Err(Throwable(throwable))
}

/// Process the throwable and return the next instruction to execute; if there is no exception
/// handler, the exception is returned as an error.
///
/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.7.3>
/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.athrow>
pub(crate) async fn process_throwable(
    frame: &Frame,
    stack: &mut OperandStack,
    throwable: Object,
) -> Result<usize> {
    let thread = frame.thread()?;
    let throwable_class = throwable.class();
    let class = frame.class();
    let constant_pool = class.constant_pool();
    let method = frame.method();
    let exception_table = method.exception_table();
    let program_counter = u16::try_from(frame.program_counter())?;

    for exception_table_entry in exception_table {
        if !exception_table_entry.range_pc.contains(&program_counter) {
            continue;
        }

        // If the catch_type is 0, the exception handler matches any exception.
        // This is used to implement finally blocks.
        // See: https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-4.html#jvms-4.7.3
        // See: https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-3.html#jvms-3.13
        let matching_exception_handler = if exception_table_entry.catch_type == 0 {
            true
        } else {
            let exception_class_name =
                constant_pool.try_get_class(exception_table_entry.catch_type)?;
            let exception_class = thread.class(exception_class_name).await?;
            exception_class.is_assignable_from(throwable_class)?
        };

        if matching_exception_handler {
            let handler_program_counter = usize::from(exception_table_entry.handler_pc);
            stack.push_object(Some(Reference::from(throwable)))?;
            return Ok(handler_program_counter);
        }
    }

    // If no exception handler is found, an error containing the throwable is returned
    Err(Throwable(throwable))
}

/// Convert native Rust errors to Java throwable.
///
/// # Errors
///
/// if the error cannot be converted to a throwable
pub(crate) async fn convert_error_to_throwable(vm: Arc<VM>, error: Error) -> Result<Object> {
    let (class_name, message) = match error {
        JavaError(java_error) => {
            let class_name = java_error.class_name().to_string();
            let message = java_error.message();
            (class_name, message)
        }
        Throwable(throwable) => return Ok(throwable),
        _ => ("java.lang.InternalError".to_string(), format!("{error}")),
    };

    let throwable = vm
        .object(class_name, "Ljava/lang/String;", &[message])
        .await?;
    let throwable: Object = throwable.try_into()?;
    Ok(throwable)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::java_object::JavaObject;

    #[tokio::test]
    async fn test_process_throwable() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let value = "foo".to_object(&thread).await?;
        let result = thread
            .invoke(
                "java.lang.Integer",
                "parseInt(Ljava/lang/String;)I",
                &[value],
            )
            .await;
        assert!(result.is_err());
        Ok(())
    }
}
