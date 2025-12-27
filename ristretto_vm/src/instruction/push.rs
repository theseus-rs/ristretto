use crate::Result;
use crate::frame::ExecutionResult;
use crate::frame::ExecutionResult::Continue;
use crate::operand_stack::OperandStack;

/// # References
///
/// - [JVMS §6.5.bipush](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.bipush)
#[inline]
pub(crate) fn bipush(stack: &mut OperandStack, value: i8) -> Result<ExecutionResult> {
    stack.push_int(i32::from(value))?;
    Ok(Continue)
}

/// # References
///
/// - [JVMS §6.5.sipush](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.sipush)
#[inline]
pub(crate) fn sipush(stack: &mut OperandStack, value: i16) -> Result<ExecutionResult> {
    stack.push_int(i32::from(value))?;
    Ok(Continue)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bipush() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        let result = bipush(stack, 42)?;
        assert_eq!(Continue, result);
        assert_eq!(42, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_sipush() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        let result = sipush(stack, 42)?;
        assert_eq!(Continue, result);
        assert_eq!(42, stack.pop_int()?);
        Ok(())
    }
}
