use crate::frame::{ExecutionResult, Frame};
use crate::Error::{InternalError, Throwable};
use crate::{Error, Result, VM};
use ristretto_classloader::{Object, Reference};
use std::sync::Arc;

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.athrow>
#[inline]
pub(crate) async fn athrow(frame: &Frame) -> Result<ExecutionResult> {
    let stack = frame.stack();
    let Some(Reference::Object(throwable)) = stack.pop_object()? else {
        return Err(InternalError("Expected object on top of stack".to_string()));
    };
    // Return the exception to the caller and let the frame error handler deal with it
    Err(Throwable(throwable))
}

/// Process the throwable and return the next instruction to execute; if there is no exception
/// handler, the exception is returned as an error.
///
/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7.3>
/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.athrow>
pub(crate) fn process_throwable(frame: &Frame, throwable: Object) -> Result<usize> {
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

        let exception_class_name = constant_pool.try_get_class(exception_table_entry.catch_type)?;
        if throwable_class.is_assignable_from(exception_class_name)? {
            let stack = frame.stack();
            let handler_program_counter = usize::from(exception_table_entry.handler_pc);
            stack.push_object(Some(Reference::Object(throwable)))?;
            return Ok(handler_program_counter);
        }
    }

    // If no exception handler is found, an error containing the throwable is returned
    Err(Throwable(throwable))
}

/// Convert native Rust errors to Java throwables.
///
/// # Errors
/// if the error cannot be converted to a throwable
pub(crate) async fn convert_error_to_throwable(vm: Arc<VM>, error: Error) -> Result<Object> {
    let throwable = match error {
        Throwable(throwable) => throwable,
        error => {
            let class = vm.class("java/lang/InternalError").await?;
            let message = format!("{error}");
            let error_message = vm.string(&message).await?;
            let throwable = Object::new(class)?;
            let detail_message_field = throwable.field("detailMessage")?;
            detail_message_field.set_value(error_message)?;
            throwable
        }
    };
    Ok(throwable)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::VM;

    #[tokio::test]
    async fn test_process_throwable() -> Result<()> {
        let vm = VM::default().await?;
        let class = vm.class("java.lang.Integer").await?;
        let method = class.try_get_method("parseInt", "(Ljava/lang/String;)I")?;
        let value = vm.string("foo").await?;
        let result = vm.invoke(&class, &method, vec![value]).await;
        assert!(result.is_err());
        Ok(())
    }
}
