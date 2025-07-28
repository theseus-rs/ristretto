use crate::Error::{InternalError, PoisonedLock};
use crate::JavaError::ArrayIndexOutOfBoundsException;
use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::{Between, GreaterThan, GreaterThanOrEqual};
use ristretto_classfile::{JAVA_11, JAVA_17};
use ristretto_classloader::{Reference, Value};
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
    parameters: Parameters,
) -> Result<Option<Value>> {
    init_stack_trace_elements_1(thread, parameters).await
}

#[intrinsic_method(
    "java/lang/StackTraceElement.initStackTraceElements([Ljava/lang/StackTraceElement;Ljava/lang/Object;I)V",
    GreaterThan(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn init_stack_trace_elements_1(
    _thread: Arc<Thread>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let depth = usize::try_from(parameters.pop_int()?)?;
    let Some(Reference::Array(back_trace_array)) = parameters.pop_reference()? else {
        return Err(InternalError("No back trace object found".to_string()));
    };
    let Some(Reference::Array(stack_trace_array)) = parameters.pop_reference()? else {
        return Err(InternalError("No stack trace object found".to_string()));
    };
    for index in 0..depth {
        // Limit the scope of the read lock on the back_trace_array
        let value = {
            let back_trace_array = back_trace_array
                .elements
                .read()
                .map_err(|error| PoisonedLock(error.to_string()))?;
            let Some(value) = back_trace_array.get(index) else {
                return Err(InternalError("No back trace element found".to_string()));
            };
            value.clone()
        };

        let mut stack_trace_array = stack_trace_array
            .elements
            .write()
            .map_err(|error| PoisonedLock(error.to_string()))?;
        if let Some(element) = stack_trace_array.get_mut(index) {
            *element = value;
        } else {
            return Err(ArrayIndexOutOfBoundsException {
                index: i32::try_from(index)?,
                length: stack_trace_array.len(),
            }
            .into());
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
