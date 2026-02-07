use crate::Result;
use crate::frame::ExecutionResult;
use crate::frame::ExecutionResult::Continue;
use crate::frame::Frame;
use crate::operand_stack::OperandStack;
use ristretto_intrinsics::get_monitor_id;

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

    let monitor_id = get_monitor_id(&reference.read());
    if let Some(id) = monitor_id {
        let thread = frame.thread()?;
        let vm = thread.vm()?;
        let monitor = vm.monitor_registry().monitor(id);
        monitor.acquire(thread.id()).await?;
    }

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

    let monitor_id = get_monitor_id(&reference.read());
    if let Some(id) = monitor_id {
        let thread = frame.thread()?;
        let vm = thread.vm()?;
        let monitor = vm.monitor_registry().monitor(id);
        // Monitor::release returns bool (true if fully released).
        // monitorexit just decrements count.
        monitor.release(thread.id())?;
    }

    Ok(Continue)
}

#[cfg(test)]
mod tests {

    // Tests need full Frame/Thread setup which is not easily mockable here without integration test helpers.
    // Skipping unit tests here in favor of integration tests.
}
