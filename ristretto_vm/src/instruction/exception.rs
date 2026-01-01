use crate::Error::{JavaError, Throwable};
use crate::JavaError::NullPointerException;
use crate::assignable::Assignable;
use crate::frame::{ExecutionResult, Frame};
use crate::operand_stack::OperandStack;
use crate::thread::Thread;
use crate::{Error, Result};
use ristretto_classloader::Value;

/// # References
///
/// - [JVMS §6.5.athrow](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.athrow)
#[inline]
pub(crate) async fn athrow(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let throwable = stack.pop()?;
    if throwable.is_null() {
        return Err(NullPointerException(Some("Cannot throw null".to_string())).into());
    }
    // Return the exception to the caller and let the frame error handler deal with it
    Err(Throwable(throwable))
}

/// Process the throwable and return the next instruction to execute; if there is no exception
/// handler, the exception is returned as an error.
///
/// # References
///
/// - [JVMS §4.7.3](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.3)
/// - [JVMS §6.5.athrow](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.athrow)
pub(crate) async fn process_throwable(
    frame: &Frame,
    stack: &mut OperandStack,
    throwable: Value,
) -> Result<usize> {
    let thread = frame.thread()?;
    let throwable_class = {
        let throwable_object = throwable.as_object_ref()?;
        throwable_object.class().clone()
    };
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
        // See: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-4.html#jvms-4.7.3
        // See: https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-3.html#jvms-3.13
        let matching_exception_handler = if exception_table_entry.catch_type == 0 {
            true
        } else {
            let exception_class_name =
                constant_pool.try_get_class(exception_table_entry.catch_type)?;
            let exception_class = thread.class(exception_class_name).await?;
            exception_class
                .is_assignable_from(&thread, &throwable_class)
                .await?
        };

        if matching_exception_handler {
            let handler_program_counter = usize::from(exception_table_entry.handler_pc);
            stack.clear();
            stack.push(throwable)?;
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
pub(crate) async fn convert_error_to_throwable(thread: &Thread, error: Error) -> Result<Value> {
    if let JavaError(NullPointerException(None)) = error {
        return thread
            .object(
                "java/lang/NullPointerException",
                "Ljava/lang/String;",
                &[Value::Object(None)],
            )
            .await;
    }

    let (class_name, message) = match error {
        JavaError(java_error) => {
            let class_name = java_error.class_name().to_string();
            let message = java_error.message();
            (class_name, message)
        }
        Throwable(throwable) => return Ok(throwable),
        _ => ("java.lang.InternalError".to_string(), format!("{error}")),
    };

    let throwable = thread
        .object(class_name, "Ljava/lang/String;", &[message])
        .await?;
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
