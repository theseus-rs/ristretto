use crate::Result;
use crate::frame::ExecutionResult;
use crate::frame::ExecutionResult::{Continue, ContinueAtPosition, Return};
use crate::local_variables::LocalVariables;
use crate::operand_stack::OperandStack;
use indexmap::IndexMap;

/// # References
///
/// - [JVMS §6.5.if_cond](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.if_cond)
#[inline]
pub(crate) fn ifeq(stack: &mut OperandStack, address: u16) -> Result<ExecutionResult> {
    if stack.pop_int()? == 0 {
        return Ok(ContinueAtPosition(usize::from(address)));
    }
    Ok(Continue)
}

/// # References
///
/// - [JVMS §6.5.if_cond](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.if_cond)
#[inline]
pub(crate) fn ifne(stack: &mut OperandStack, address: u16) -> Result<ExecutionResult> {
    if stack.pop_int()? != 0 {
        return Ok(ContinueAtPosition(usize::from(address)));
    }
    Ok(Continue)
}

/// # References
///
/// - [JVMS §6.5.if_cond](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.if_cond)
#[inline]
pub(crate) fn iflt(stack: &mut OperandStack, address: u16) -> Result<ExecutionResult> {
    if stack.pop_int()? < 0 {
        return Ok(ContinueAtPosition(usize::from(address)));
    }
    Ok(Continue)
}

/// # References
///
/// - [JVMS §6.5.if_cond](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.if_cond)
#[inline]
pub(crate) fn ifge(stack: &mut OperandStack, address: u16) -> Result<ExecutionResult> {
    if stack.pop_int()? >= 0 {
        return Ok(ContinueAtPosition(usize::from(address)));
    }
    Ok(Continue)
}

/// # References
///
/// - [JVMS §6.5.if_cond](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.if_cond)
#[inline]
pub(crate) fn ifgt(stack: &mut OperandStack, address: u16) -> Result<ExecutionResult> {
    if stack.pop_int()? > 0 {
        return Ok(ContinueAtPosition(usize::from(address)));
    }
    Ok(Continue)
}

/// # References
///
/// - [JVMS §6.5.if_cond](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.if_cond)
#[inline]
pub(crate) fn ifle(stack: &mut OperandStack, address: u16) -> Result<ExecutionResult> {
    if stack.pop_int()? <= 0 {
        return Ok(ContinueAtPosition(usize::from(address)));
    }
    Ok(Continue)
}

/// # References
///
/// - [JVMS §6.5.if_icmp_cond](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.if_icmp_cond)
#[inline]
pub(crate) fn if_icmpeq(stack: &mut OperandStack, address: u16) -> Result<ExecutionResult> {
    let value2 = stack.pop_int()?;
    let value1 = stack.pop_int()?;
    if value1 == value2 {
        return Ok(ContinueAtPosition(usize::from(address)));
    }
    Ok(Continue)
}

/// # References
///
/// - [JVMS §6.5.if_icmp_cond](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.if_icmp_cond)
#[inline]
pub(crate) fn if_icmpne(stack: &mut OperandStack, address: u16) -> Result<ExecutionResult> {
    let value2 = stack.pop_int()?;
    let value1 = stack.pop_int()?;
    if value1 != value2 {
        return Ok(ContinueAtPosition(usize::from(address)));
    }
    Ok(Continue)
}

/// # References
///
/// - [JVMS §6.5.if_icmp_cond](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.if_icmp_cond)
#[inline]
pub(crate) fn if_icmplt(stack: &mut OperandStack, address: u16) -> Result<ExecutionResult> {
    let value2 = stack.pop_int()?;
    let value1 = stack.pop_int()?;
    if value1 < value2 {
        return Ok(ContinueAtPosition(usize::from(address)));
    }
    Ok(Continue)
}

/// # References
///
/// - [JVMS §6.5.if_icmp_cond](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.if_icmp_cond)
#[inline]
pub(crate) fn if_icmpge(stack: &mut OperandStack, address: u16) -> Result<ExecutionResult> {
    let value2 = stack.pop_int()?;
    let value1 = stack.pop_int()?;
    if value1 >= value2 {
        return Ok(ContinueAtPosition(usize::from(address)));
    }
    Ok(Continue)
}

/// # References
///
/// - [JVMS §6.5.if_icmp_cond](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.if_icmp_cond)
#[inline]
pub(crate) fn if_icmpgt(stack: &mut OperandStack, address: u16) -> Result<ExecutionResult> {
    let value2 = stack.pop_int()?;
    let value1 = stack.pop_int()?;
    if value1 > value2 {
        return Ok(ContinueAtPosition(usize::from(address)));
    }
    Ok(Continue)
}

/// # References
///
/// - [JVMS §6.5.if_icmp_cond](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.if_icmp_cond)
#[inline]
pub(crate) fn if_icmple(stack: &mut OperandStack, address: u16) -> Result<ExecutionResult> {
    let value2 = stack.pop_int()?;
    let value1 = stack.pop_int()?;
    if value1 <= value2 {
        return Ok(ContinueAtPosition(usize::from(address)));
    }
    Ok(Continue)
}

/// # References
///
/// - [JVMS §6.5.if_acmp_cond](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.if_acmp_cond)
#[inline]
pub(crate) fn if_acmpeq(stack: &mut OperandStack, address: u16) -> Result<ExecutionResult> {
    let value2 = stack.pop_object()?;
    let value1 = stack.pop_object()?;

    match (value1, value2) {
        (None, None) => Ok(ContinueAtPosition(usize::from(address))),
        (Some(_), None) | (None, Some(_)) => Ok(Continue),
        (Some(value1), Some(value2)) => {
            if value1.ptr_eq(&value2) {
                Ok(ContinueAtPosition(usize::from(address)))
            } else {
                Ok(Continue)
            }
        }
    }
}

/// # References
///
/// - [JVMS §6.5.if_acmp_cond](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.if_acmp_cond)
#[inline]
pub(crate) fn if_acmpne(stack: &mut OperandStack, address: u16) -> Result<ExecutionResult> {
    let value2 = stack.pop_object()?;
    let value1 = stack.pop_object()?;

    match (value1, value2) {
        (None, None) => Ok(Continue),
        (Some(_), None) | (None, Some(_)) => Ok(ContinueAtPosition(usize::from(address))),
        (Some(value1), Some(value2)) => {
            if value1.ptr_eq(&value2) {
                Ok(Continue)
            } else {
                Ok(ContinueAtPosition(usize::from(address)))
            }
        }
    }
}

/// # References
///
/// - [JVMS §6.5.goto](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.goto)
#[expect(clippy::unnecessary_wraps)]
#[inline]
pub(crate) fn goto(address: u16) -> Result<ExecutionResult> {
    Ok(ContinueAtPosition(usize::from(address)))
}

/// # References
///
/// - [JVMS §6.5.goto_w](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.goto_w)
/// - [JVMS §6.5.wide](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.wide)
#[inline]
pub(crate) fn goto_w(address: i32) -> Result<ExecutionResult> {
    Ok(ContinueAtPosition(usize::try_from(address)?))
}

/// # References
///
/// - [JVMS §6.5.jsr](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.jsr)
#[inline]
pub(crate) fn jsr(stack: &mut OperandStack, address: u16) -> Result<ExecutionResult> {
    let address = i32::from(address);
    stack.push_int(address)?;
    Ok(ContinueAtPosition(usize::try_from(address)?))
}

/// # References
///
/// - [JVMS §6.5.jsr](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.jsr)
/// - [JVMS §6.5.wide](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.wide)
#[inline]
pub(crate) fn jsr_w(stack: &mut OperandStack, address: i32) -> Result<ExecutionResult> {
    stack.push_int(address)?;
    Ok(ContinueAtPosition(usize::try_from(address)?))
}

/// # References
///
/// - [JVMS §6.5.ret](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.ret)
#[inline]
pub(crate) fn ret(locals: &LocalVariables, index: u8) -> Result<ExecutionResult> {
    let address = locals.get_int(usize::from(index))?;
    Ok(ContinueAtPosition(usize::try_from(address)?))
}

/// # References
///
/// - [JVMS §6.5.ret](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.ret)
/// - [JVMS §6.5.wide](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.wide)
#[inline]
pub(crate) fn ret_w(locals: &LocalVariables, index: u16) -> Result<ExecutionResult> {
    let address = locals.get_int(usize::from(index))?;
    Ok(ContinueAtPosition(usize::try_from(address)?))
}

/// # References
///
/// - [JVMS §6.5.tableswitch](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.tableswitch)
#[expect(clippy::ptr_arg)]
#[inline]
pub(crate) fn tableswitch(
    stack: &mut OperandStack,
    program_counter: usize,
    default: i32,
    low: i32,
    high: i32,
    offsets: &Vec<i32>,
) -> Result<ExecutionResult> {
    let key = stack.pop_int()?;
    let offset = if key < low || key > high {
        default
    } else {
        let index = usize::try_from(key - low)?;
        offsets[index]
    };
    let pc = i64::try_from(program_counter)?;
    let address = usize::try_from(pc + i64::from(offset))?;
    Ok(ContinueAtPosition(address))
}

/// # References
///
/// - [JVMS §6.5.lookupswitch](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.lookupswitch)
#[inline]
pub(crate) fn lookupswitch(
    stack: &mut OperandStack,
    program_counter: usize,
    default: i32,
    pairs: &IndexMap<i32, i32>,
) -> Result<ExecutionResult> {
    let key = stack.pop_int()?;
    let offset = match pairs.get(&key) {
        Some(offset) => *offset,
        None => default,
    };
    let pc = i64::try_from(program_counter)?;
    let address = usize::try_from(pc + i64::from(offset))?;
    Ok(ContinueAtPosition(address))
}

/// # References
///
/// - [JVMS §6.5.return](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.return)
#[expect(clippy::unnecessary_wraps)]
#[inline]
pub(crate) fn r#return() -> Result<ExecutionResult> {
    Ok(Return(None))
}

/// # References
///
/// - [JVMS §6.5.ifnull](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.ifnull)
#[inline]
pub(crate) fn ifnull(stack: &mut OperandStack, address: u16) -> Result<ExecutionResult> {
    if stack.pop_object()?.is_none() {
        return Ok(ContinueAtPosition(usize::from(address)));
    }
    Ok(Continue)
}

/// # References
///
/// - [JVMS §6.5.ifnonnull](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.ifnonnull)
#[inline]
pub(crate) fn ifnonnull(stack: &mut OperandStack, address: u16) -> Result<ExecutionResult> {
    if stack.pop_object()?.is_some() {
        return Ok(ContinueAtPosition(usize::from(address)));
    }
    Ok(Continue)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::java_object::JavaObject;
    use parking_lot::RwLock;
    use ristretto_classloader::Value;
    use ristretto_gc::Gc;

    #[test]
    fn test_ifeq_equal() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(0)?;
        let result = ifeq(stack, 3)?;
        assert_eq!(ContinueAtPosition(3), result);
        Ok(())
    }

    #[test]
    fn test_ifeq_not_equal() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(1)?;
        let result = ifeq(stack, 3)?;
        assert_eq!(Continue, result);
        Ok(())
    }

    #[test]
    fn test_ifne_equal() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(0)?;
        let result = ifne(stack, 3)?;
        assert_eq!(Continue, result);
        Ok(())
    }

    #[test]
    fn test_ifne_not_equal() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(1)?;
        let result = ifne(stack, 3)?;
        assert_eq!(ContinueAtPosition(3), result);
        Ok(())
    }

    #[test]
    fn test_iflt_equal() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(0)?;
        let result = iflt(stack, 3)?;
        assert_eq!(Continue, result);
        Ok(())
    }

    #[test]
    fn test_iflt_less_than() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(-1)?;
        let result = iflt(stack, 3)?;
        assert_eq!(ContinueAtPosition(3), result);
        Ok(())
    }

    #[test]
    fn test_ifge() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);

        // Test less than
        stack.push_int(-1)?;
        let result = ifge(stack, 3)?;
        assert_eq!(Continue, result);

        // Test equal
        stack.push_int(0)?;
        let result = ifge(stack, 3)?;
        assert_eq!(ContinueAtPosition(3), result);

        // Test greater than
        stack.push_int(1)?;
        let result = ifge(stack, 3)?;
        assert_eq!(ContinueAtPosition(3), result);
        Ok(())
    }

    #[test]
    fn test_ifge_equal() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(0)?;
        let result = ifge(stack, 3)?;
        assert_eq!(ContinueAtPosition(3), result);
        Ok(())
    }

    #[test]
    fn test_ifge_less_than() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(-1)?;
        let result = ifge(stack, 3)?;
        assert_eq!(Continue, result);
        Ok(())
    }

    #[test]
    fn test_ifge_greater_than() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(1)?;
        let result = ifge(stack, 3)?;
        assert_eq!(ContinueAtPosition(3), result);
        Ok(())
    }

    #[test]
    fn test_ifgt_equal() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(0)?;
        let result = ifgt(stack, 3)?;
        assert_eq!(Continue, result);
        Ok(())
    }

    #[test]
    fn test_ifgt_greater_than() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(1)?;
        let result = ifgt(stack, 3)?;
        assert_eq!(ContinueAtPosition(3), result);
        Ok(())
    }

    #[test]
    fn test_ifle_equal() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(0)?;
        let result = ifle(stack, 3)?;
        assert_eq!(ContinueAtPosition(3), result);
        Ok(())
    }

    #[test]
    fn test_ifle_less_than() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(-1)?;
        let result = ifle(stack, 3)?;
        assert_eq!(ContinueAtPosition(3), result);
        Ok(())
    }

    #[test]
    fn test_ifle_greater_than() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(1)?;
        let result = ifle(stack, 3)?;
        assert_eq!(Continue, result);
        Ok(())
    }

    #[test]
    fn test_if_icmpeq_equal() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_int(1)?;
        stack.push_int(1)?;
        let result = if_icmpeq(stack, 3)?;
        assert_eq!(ContinueAtPosition(3), result);
        Ok(())
    }

    #[test]
    fn test_if_icmpeq_not_equal() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_int(2)?;
        stack.push_int(1)?;
        let result = if_icmpeq(stack, 3)?;
        assert_eq!(Continue, result);
        Ok(())
    }

    #[test]
    fn test_if_icmpne_equal() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_int(1)?;
        stack.push_int(1)?;
        let result = if_icmpne(stack, 3)?;
        assert_eq!(Continue, result);
        Ok(())
    }

    #[test]
    fn test_if_icmpne_not_equal() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_int(2)?;
        stack.push_int(1)?;
        let result = if_icmpne(stack, 3)?;
        assert_eq!(ContinueAtPosition(3), result);
        Ok(())
    }

    #[test]
    fn test_if_icmplt_equal() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_int(1)?;
        stack.push_int(1)?;
        let result = if_icmplt(stack, 3)?;
        assert_eq!(Continue, result);
        Ok(())
    }
    #[test]
    fn test_if_icmplt_less_than() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_int(1)?;
        stack.push_int(2)?;
        let result = if_icmplt(stack, 3)?;
        assert_eq!(ContinueAtPosition(3), result);
        Ok(())
    }

    #[test]
    fn test_if_icmpge_equal() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_int(1)?;
        stack.push_int(1)?;
        let result = if_icmpge(stack, 3)?;
        assert_eq!(ContinueAtPosition(3), result);
        Ok(())
    }

    #[test]
    fn test_if_icmpge_less_than() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_int(1)?;
        stack.push_int(2)?;
        let result = if_icmpge(stack, 3)?;
        assert_eq!(Continue, result);
        Ok(())
    }

    #[test]
    fn test_if_icmpge_greater_than() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_int(2)?;
        stack.push_int(1)?;
        let result = if_icmpge(stack, 3)?;
        assert_eq!(ContinueAtPosition(3), result);
        Ok(())
    }

    #[test]
    fn test_if_icmpgt_equal() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_int(1)?;
        stack.push_int(1)?;
        let result = if_icmpgt(stack, 3)?;
        assert_eq!(Continue, result);
        Ok(())
    }

    #[test]
    fn test_if_icmpgt_greater_than() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_int(2)?;
        stack.push_int(1)?;
        let result = if_icmpgt(stack, 3)?;
        assert_eq!(ContinueAtPosition(3), result);
        Ok(())
    }

    #[test]
    fn test_if_icmple_equal() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_int(1)?;
        stack.push_int(1)?;
        let result = if_icmple(stack, 3)?;
        assert_eq!(ContinueAtPosition(3), result);
        Ok(())
    }

    #[test]
    fn test_if_icmple_greater_than() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_int(2)?;
        stack.push_int(1)?;
        let result = if_icmple(stack, 3)?;
        assert_eq!(Continue, result);
        Ok(())
    }

    #[test]
    fn test_if_icmple_less_than() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_int(1)?;
        stack.push_int(2)?;
        let result = if_icmple(stack, 3)?;
        assert_eq!(ContinueAtPosition(3), result);
        Ok(())
    }

    #[test]
    fn test_if_acmpeq_equal() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        let object = Value::from(vec![42i8]);
        stack.push(object.clone())?;
        stack.push(object.clone())?;
        let result = if_acmpeq(stack, 3)?;
        assert_eq!(ContinueAtPosition(3), result);
        Ok(())
    }

    #[test]
    fn test_if_acmpeq_not_equal() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        let object = Value::from(vec![42i8]);
        stack.push_object(None)?;
        stack.push(object.clone())?;
        let result = if_acmpeq(stack, 3)?;
        assert_eq!(Continue, result);
        stack.push(object.clone())?;
        stack.push(object.clone())?;
        let result = if_acmpeq(stack, 3)?;
        assert_eq!(ContinueAtPosition(3), result);
        Ok(())
    }

    #[test]
    fn test_if_acmpeq_null() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_object(None)?;
        stack.push_object(None)?;
        let result = if_acmpeq(stack, 3)?;
        assert_eq!(ContinueAtPosition(3), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_if_acmpeq_class_equal() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let class = thread.class("java.lang.String").await?;
        let class1 = class.to_object(&thread).await?;
        let class2 = class.to_object(&thread).await?;

        let stack = &mut OperandStack::with_max_size(2);
        stack.push(class1)?;
        stack.push(class2)?;
        let result = if_acmpeq(stack, 3)?;
        assert_eq!(ContinueAtPosition(3), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_if_acmpeq_class_not_equal() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let class1 = thread.class("java.lang.Object").await?;
        let class1 = class1.to_object(&thread).await?;
        let class2 = thread.class("java.lang.String").await?;
        let class2 = class2.to_object(&thread).await?;

        let stack = &mut OperandStack::with_max_size(2);
        stack.push(class1)?;
        stack.push(class2)?;
        let result = if_acmpeq(stack, 3)?;
        assert_eq!(Continue, result);
        Ok(())
    }

    #[test]
    fn test_if_acmpne_equal() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        let object = Value::from(vec![42i8]);
        stack.push(object.clone())?;
        stack.push(object.clone())?;
        let result = if_acmpne(stack, 3)?;
        assert_eq!(Continue, result);
        Ok(())
    }

    #[test]
    fn test_if_acmpne_not_equal() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        let object = Value::from(vec![42i8]);
        stack.push_object(None)?;
        stack.push(object.clone())?;
        let result = if_acmpne(stack, 3)?;
        assert_eq!(ContinueAtPosition(3), result);
        Ok(())
    }

    #[test]
    fn test_if_acmpne_null() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_object(None)?;
        stack.push_object(None)?;
        let result = if_acmpne(stack, 3)?;
        assert_eq!(Continue, result);
        Ok(())
    }

    #[tokio::test]
    async fn test_if_acmpne_class_equal() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let class = thread.class("java.lang.String").await?;
        let class1 = class.to_object(&thread).await?;
        let class2 = class.to_object(&thread).await?;

        let stack = &mut OperandStack::with_max_size(2);
        stack.push(class1)?;
        stack.push(class2)?;
        let result = if_acmpne(stack, 3)?;
        assert_eq!(Continue, result);
        Ok(())
    }

    #[tokio::test]
    async fn test_if_acmpne_class_not_equal() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let class1 = thread.class("java.lang.Object").await?;
        let class1 = class1.to_object(&thread).await?;
        let class2 = thread.class("java.lang.String").await?;
        let class2 = class2.to_object(&thread).await?;

        let stack = &mut OperandStack::with_max_size(2);
        let class1 = class1.as_reference()?.clone();
        stack.push_object(Some(Gc::new(RwLock::new(class1)).clone_gc()))?;
        let class2 = class2.as_reference()?.clone();
        stack.push_object(Some(Gc::new(RwLock::new(class2)).clone_gc()))?;
        let result = if_acmpne(stack, 3)?;
        assert_eq!(ContinueAtPosition(3), result);
        Ok(())
    }

    #[test]
    fn test_goto() -> Result<()> {
        let result = goto(3)?;
        assert_eq!(ContinueAtPosition(3), result);
        Ok(())
    }

    #[test]
    fn test_goto_w() -> Result<()> {
        let result = goto_w(3)?;
        assert_eq!(ContinueAtPosition(3), result);
        Ok(())
    }

    #[test]
    fn test_jsr() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        let result = jsr(stack, 3)?;
        assert_eq!(ContinueAtPosition(3), result);
        assert_eq!(3, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_jsr_w() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        let result = jsr_w(stack, 3)?;
        assert_eq!(ContinueAtPosition(3), result);
        assert_eq!(3, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_ret() -> Result<()> {
        let locals = &mut LocalVariables::with_max_size(1);
        locals.set_int(0, 3)?;
        let result = ret(locals, 0)?;
        assert_eq!(ContinueAtPosition(3), result);
        Ok(())
    }

    #[test]
    fn test_ret_w() -> Result<()> {
        let locals = &mut LocalVariables::with_max_size(1);
        locals.set_int(0, 3)?;
        let result = ret_w(locals, 0)?;
        assert_eq!(ContinueAtPosition(3), result);
        Ok(())
    }

    #[test]
    fn test_tableswitch() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(2)?;
        let program_counter = 10;
        let result = tableswitch(stack, program_counter, 14, 1, 3, &vec![11, 12, 13])?;
        assert!(matches!(result, ContinueAtPosition(22)));
        Ok(())
    }

    #[test]
    fn test_tableswitch_default() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(0)?;
        let program_counter = 10;
        let result = tableswitch(stack, program_counter, 14, 1, 3, &vec![11, 12, 13])?;
        assert!(matches!(result, ContinueAtPosition(24)));
        Ok(())
    }

    /// Test tableswitch with negative offset (backward jump)
    #[test]
    fn test_tableswitch_negative_offset() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(1)?; // Select case 1, which has offset -5
        let program_counter = 20;
        // low=0, high=2, offsets: case 0 -> +10, case 1 -> -5 (backward), case 2 -> +15
        let result = tableswitch(stack, program_counter, 100, 0, 2, &vec![10, -5, 15])?;
        // program_counter (20) + offset (-5) = 15
        assert_eq!(ContinueAtPosition(15), result);
        Ok(())
    }

    /// Test tableswitch with negative default offset (backward jump for default case)
    #[test]
    fn test_tableswitch_negative_default_offset() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(99)?; // Value outside range, will use default
        let program_counter = 20;
        // default offset is -10 (backward jump)
        let result = tableswitch(stack, program_counter, -10, 0, 2, &vec![10, 15, 20])?;
        // program_counter (20) + default offset (-10) = 10
        assert_eq!(ContinueAtPosition(10), result);
        Ok(())
    }

    #[test]
    fn test_lookupswitch() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(2)?;
        let program_counter = 10;
        let result = lookupswitch(
            stack,
            program_counter,
            14,
            &IndexMap::from([(1, 11), (2, 12), (3, 13)]),
        )?;
        assert!(matches!(result, ContinueAtPosition(22)));
        Ok(())
    }

    #[test]
    fn test_lookupswitch_default() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(0)?;
        let program_counter = 10;
        let result = lookupswitch(
            stack,
            program_counter,
            14,
            &IndexMap::from([(1, 11), (2, 12), (3, 13)]),
        )?;
        assert!(matches!(result, ContinueAtPosition(24)));
        Ok(())
    }

    /// Test lookupswitch with negative offset (backward jump)
    #[test]
    fn test_lookupswitch_negative_offset() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(42)?; // Select case 42, which has offset -8
        let program_counter = 20;
        let result = lookupswitch(
            stack,
            program_counter,
            100,                                    // default offset (forward)
            &IndexMap::from([(42, -8), (100, 15)]), // case 42 jumps backward
        )?;
        // program_counter (20) + offset (-8) = 12
        assert_eq!(ContinueAtPosition(12), result);
        Ok(())
    }

    /// Test lookupswitch with negative default offset (backward jump for default case)
    #[test]
    fn test_lookupswitch_negative_default_offset() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(0)?; // Value not in lookup table, will use default
        let program_counter = 20;
        let result = lookupswitch(
            stack,
            program_counter,
            -15, // default offset is backward jump
            &IndexMap::from([(42, 10), (100, 15)]),
        )?;
        // program_counter (20) + default offset (-15) = 5
        assert_eq!(ContinueAtPosition(5), result);
        Ok(())
    }

    #[test]
    fn test_return() -> Result<()> {
        let result = r#return()?;
        assert!(matches!(result, Return(None)));
        Ok(())
    }

    #[test]
    fn test_ifnull_not_null() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        let object = Value::from(vec![42i8]);
        stack.push(object)?;
        let result = ifnull(stack, 3)?;
        assert_eq!(Continue, result);
        Ok(())
    }

    #[test]
    fn test_ifnull_null() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_object(None)?;
        let result = ifnull(stack, 3)?;
        assert_eq!(ContinueAtPosition(3), result);
        Ok(())
    }

    #[test]
    fn test_ifnonnull_not_null() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        let object = Value::from(vec![42i8]);
        stack.push(object)?;
        let result = ifnonnull(stack, 3)?;
        assert_eq!(ContinueAtPosition(3), result);
        Ok(())
    }

    #[test]
    fn test_ifnonnull_null() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_object(None)?;
        let result = ifnonnull(stack, 3)?;
        assert_eq!(Continue, result);
        Ok(())
    }
}
