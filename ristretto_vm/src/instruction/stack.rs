use crate::frame::ExecutionResult;
use crate::frame::ExecutionResult::Continue;
use crate::operand_stack::OperandStack;
use crate::Result;

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.pop>
#[inline]
pub(crate) fn pop(stack: &OperandStack) -> Result<ExecutionResult> {
    let _ = stack.pop()?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.pop2>
#[inline]
pub(crate) fn pop2(stack: &OperandStack) -> Result<ExecutionResult> {
    let value = stack.pop()?;
    if value.is_category_1() {
        let _ = stack.pop()?;
    }
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.dup>
#[inline]
pub(crate) fn dup(stack: &OperandStack) -> Result<ExecutionResult> {
    let value = stack.pop()?;
    stack.push(value.clone())?;
    stack.push(value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.dup_x1>
#[inline]
pub(crate) fn dup_x1(stack: &OperandStack) -> Result<ExecutionResult> {
    let value1 = stack.pop()?;
    let value2 = stack.pop()?;
    stack.push(value1.clone())?;
    stack.push(value2)?;
    stack.push(value1)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.dup_x2>
#[inline]
pub(crate) fn dup_x2(stack: &OperandStack) -> Result<ExecutionResult> {
    let value1 = stack.pop()?;
    let value2 = stack.pop()?;
    if value2.is_category_1() {
        let value3 = stack.pop()?;
        stack.push(value1.clone())?;
        stack.push(value3)?;
        stack.push(value2)?;
        stack.push(value1)?;
    } else {
        stack.push(value1.clone())?;
        stack.push(value2)?;
        stack.push(value1)?;
    }
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.dup2>
#[inline]
pub(crate) fn dup2(stack: &OperandStack) -> Result<ExecutionResult> {
    let value1 = stack.pop()?;
    if value1.is_category_1() {
        let value2 = stack.pop()?;
        stack.push(value2.clone())?;
        stack.push(value1.clone())?;
        stack.push(value2)?;
        stack.push(value1)?;
    } else {
        stack.push(value1.clone())?;
        stack.push(value1)?;
    }
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.dup2_x1>
#[inline]
pub(crate) fn dup2_x1(stack: &OperandStack) -> Result<ExecutionResult> {
    let value1 = stack.pop()?;
    let value2 = stack.pop()?;
    if value1.is_category_1() {
        let value3 = stack.pop()?;
        stack.push(value2.clone())?;
        stack.push(value1.clone())?;
        stack.push(value3)?;
        stack.push(value2)?;
        stack.push(value1)?;
    } else {
        stack.push(value1.clone())?;
        stack.push(value2)?;
        stack.push(value1)?;
    }
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.dup2_x2>
#[inline]
pub(crate) fn dup2_x2(stack: &OperandStack) -> Result<ExecutionResult> {
    let value1 = stack.pop()?;
    let value2 = stack.pop()?;
    if value1.is_category_1() {
        let value3 = stack.pop()?;
        if value3.is_category_1() {
            let value4 = stack.pop()?;
            stack.push(value2.clone())?;
            stack.push(value1.clone())?;
            stack.push(value4)?;
        } else {
            stack.push(value1.clone())?;
        }
        stack.push(value3)?;
        stack.push(value2)?;
        stack.push(value1)?;
    } else {
        if value2.is_category_1() {
            let value3 = stack.pop()?;
            stack.push(value1.clone())?;
            stack.push(value3)?;
        } else {
            stack.push(value1.clone())?;
        }
        stack.push(value2)?;
        stack.push(value1)?;
    }
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.swap>
#[inline]
pub(crate) fn swap(stack: &OperandStack) -> Result<ExecutionResult> {
    // Swapping category 2 values (Double and Long) is not supported by the JVM specification and
    // there is no mention of what should happen in this case. We will just swap the values and
    // ignore the fact that category 2 values could be swapped here.
    let value1 = stack.pop()?;
    let value2 = stack.pop()?;
    stack.push(value1)?;
    stack.push(value2)?;
    Ok(Continue)
}

#[cfg(test)]
mod test {
    use super::*;
    use ristretto_classloader::Value;

    #[test]
    fn test_pop() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_object(None)?;
        let result = pop(stack)?;
        assert_eq!(Continue, result);
        assert!(stack.is_empty()?);
        Ok(())
    }

    #[test]
    fn test_pop2_form_1() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_object(None)?;
        stack.push_object(None)?;
        let result = pop2(stack)?;
        assert_eq!(Continue, result);
        assert!(stack.is_empty()?);
        Ok(())
    }

    #[test]
    fn test_pop2_form_2() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_long(42)?;
        let result = pop2(stack)?;
        assert_eq!(Continue, result);
        assert!(stack.is_empty()?);
        Ok(())
    }

    #[test]
    fn test_dup() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_object(None)?;
        let result = dup(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(Value::Object(None), stack.pop()?);
        assert_eq!(Value::Object(None), stack.pop()?);
        Ok(())
    }

    #[test]
    fn test_dup_x1() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(3);
        stack.push_int(2)?;
        stack.push_int(1)?;
        let result = dup_x1(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(1, stack.pop_int()?);
        assert_eq!(2, stack.pop_int()?);
        assert_eq!(1, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_dup_x2_form_1() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(4);
        stack.push_int(3)?;
        stack.push_int(2)?;
        stack.push_int(1)?;
        let result = dup_x2(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(1, stack.pop_int()?);
        assert_eq!(2, stack.pop_int()?);
        assert_eq!(3, stack.pop_int()?);
        assert_eq!(1, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_dup_x2_form_2() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(3);
        stack.push_long(2)?;
        stack.push_int(1)?;
        let result = dup_x2(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(1, stack.pop_int()?);
        assert_eq!(2, stack.pop_long()?);
        assert_eq!(1, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_dup2_form_1() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(4);
        stack.push_int(2)?;
        stack.push_int(1)?;
        let result = dup2(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(1, stack.pop_int()?);
        assert_eq!(2, stack.pop_int()?);
        assert_eq!(1, stack.pop_int()?);
        assert_eq!(2, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_dup2_form_2() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_long(1)?;
        let result = dup2(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(1, stack.pop_long()?);
        assert_eq!(1, stack.pop_long()?);
        Ok(())
    }

    #[test]
    fn test_dup2_x1_form_1() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(5);
        stack.push_int(3)?;
        stack.push_int(2)?;
        stack.push_int(1)?;
        let result = dup2_x1(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(1, stack.pop_int()?);
        assert_eq!(2, stack.pop_int()?);
        assert_eq!(3, stack.pop_int()?);
        assert_eq!(1, stack.pop_int()?);
        assert_eq!(2, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_dup2_x1_form_2() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(5);
        stack.push_int(2)?;
        stack.push_long(1)?;
        let result = dup2_x1(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(1, stack.pop_long()?);
        assert_eq!(2, stack.pop_int()?);
        assert_eq!(1, stack.pop_long()?);
        Ok(())
    }

    #[test]
    fn test_dup2_x2_form_1() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(6);
        stack.push_int(4)?;
        stack.push_int(3)?;
        stack.push_int(2)?;
        stack.push_int(1)?;
        let result = dup2_x2(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(1, stack.pop_int()?);
        assert_eq!(2, stack.pop_int()?);
        assert_eq!(3, stack.pop_int()?);
        assert_eq!(4, stack.pop_int()?);
        assert_eq!(1, stack.pop_int()?);
        assert_eq!(2, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_dup2_x2_form_2() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(4);
        stack.push_int(3)?;
        stack.push_int(2)?;
        stack.push_long(1)?;
        let result = dup2_x2(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(1, stack.pop_long()?);
        assert_eq!(2, stack.pop_int()?);
        assert_eq!(3, stack.pop_int()?);
        assert_eq!(1, stack.pop_long()?);
        Ok(())
    }

    #[test]
    fn test_dup2_x2_form_3() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(4);
        stack.push_long(3)?;
        stack.push_int(2)?;
        stack.push_int(1)?;
        let result = dup2_x2(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(1, stack.pop_int()?);
        assert_eq!(2, stack.pop_int()?);
        assert_eq!(3, stack.pop_long()?);
        assert_eq!(1, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_dup2_x2_form_4() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(3);
        stack.push_long(2)?;
        stack.push_long(1)?;
        let result = dup2_x2(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(1, stack.pop_long()?);
        assert_eq!(2, stack.pop_long()?);
        assert_eq!(1, stack.pop_long()?);
        Ok(())
    }

    #[test]
    fn test_swap() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_int(2)?;
        stack.push_int(1)?;
        let result = swap(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(2, stack.pop_int()?);
        assert_eq!(1, stack.pop_int()?);
        Ok(())
    }
}
