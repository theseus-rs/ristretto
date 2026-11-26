use crate::Result;
use crate::frame::ExecutionResult;
use crate::frame::ExecutionResult::Continue;
use crate::frame::Frame;
use crate::operand_stack::OperandStack;

/// # References
///
/// - [JVMS §6.5.monitorenter](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.monitorenter)
#[inline]
pub(crate) async fn monitorenter(
    frame: &Frame,
    stack: &mut OperandStack,
) -> Result<ExecutionResult> {
    let object = stack.pop_object()?;
    let Some(reference) = object else {
        return Err(crate::JavaError::NullPointerException(Some(
            "monitorenter called on null".into(),
        ))
        .into());
    };

    frame.thread()?.enter_monitor(&reference).await?;

    Ok(Continue)
}

/// # References
///
/// - [JVMS §6.5.monitorexit](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.monitorexit)
#[inline]
pub(crate) async fn monitorexit(
    frame: &Frame,
    stack: &mut OperandStack,
) -> Result<ExecutionResult> {
    let object = stack.pop_object()?;
    let Some(reference) = object else {
        return Err(crate::JavaError::NullPointerException(Some(
            "monitorexit called on null".into(),
        ))
        .into());
    };

    frame.thread()?.exit_monitor(&reference)?;

    Ok(Continue)
}

#[cfg(test)]
mod tests {

    // Tests need full Frame/Thread setup which is not easily mockable here without integration test helpers.
    // Skipping unit tests here in favor of integration tests.
}
