use crate::Result;
use crate::operand_stack::OperandStack;

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.monitorenter>
pub(crate) fn monitorenter(stack: &mut OperandStack) -> Result<()> {
    // The monitorenter instruction is not currently used by this implementation.
    let _ = stack.pop()?;
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.monitorexit>
pub(crate) fn monitorexit(stack: &mut OperandStack) -> Result<()> {
    // The monitorexit instruction is not currently used by this implementation.
    let _ = stack.pop()?;
    Ok(())
}
