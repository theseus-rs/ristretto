use crate::Result;
use crate::frame::ExecutionResult;
use crate::frame::ExecutionResult::Continue;
use crate::operand_stack::OperandStack;

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.monitorenter>
#[inline]
pub(crate) fn monitorenter(stack: &mut OperandStack) -> Result<ExecutionResult> {
    // The monitorenter instruction is not currently used by this implementation.
    let _ = stack.pop_object()?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.monitorexit>
#[inline]
pub(crate) fn monitorexit(stack: &mut OperandStack) -> Result<ExecutionResult> {
    // The monitorexit instruction is not currently used by this implementation.
    let _ = stack.pop_object()?;
    Ok(Continue)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monitorenter() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_object(None)?;
        assert_eq!(monitorenter(stack)?, Continue);
        Ok(())
    }

    #[test]
    fn test_monitorexit() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_object(None)?;
        assert_eq!(monitorexit(stack)?, Continue);
        Ok(())
    }
}
