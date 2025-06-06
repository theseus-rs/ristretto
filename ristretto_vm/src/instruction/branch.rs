use crate::Result;
use crate::frame::ExecutionResult;
use crate::frame::ExecutionResult::{Continue, ContinueAtPosition, Return};
use crate::local_variables::LocalVariables;
use crate::operand_stack::OperandStack;
use indexmap::IndexMap;

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.if_cond>
#[inline]
pub(crate) fn ifeq(stack: &mut OperandStack, address: u16) -> Result<ExecutionResult> {
    if stack.pop_int()? == 0 {
        return Ok(ContinueAtPosition(usize::from(address)));
    }
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.if_cond>
#[inline]
pub(crate) fn ifne(stack: &mut OperandStack, address: u16) -> Result<ExecutionResult> {
    if stack.pop_int()? != 0 {
        return Ok(ContinueAtPosition(usize::from(address)));
    }
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.if_cond>
#[inline]
pub(crate) fn iflt(stack: &mut OperandStack, address: u16) -> Result<ExecutionResult> {
    if stack.pop_int()? < 0 {
        return Ok(ContinueAtPosition(usize::from(address)));
    }
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.if_cond>
#[inline]
pub(crate) fn ifge(stack: &mut OperandStack, address: u16) -> Result<ExecutionResult> {
    if stack.pop_int()? >= 0 {
        return Ok(ContinueAtPosition(usize::from(address)));
    }
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.if_cond>
#[inline]
pub(crate) fn ifgt(stack: &mut OperandStack, address: u16) -> Result<ExecutionResult> {
    if stack.pop_int()? > 0 {
        return Ok(ContinueAtPosition(usize::from(address)));
    }
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.if_cond>
#[inline]
pub(crate) fn ifle(stack: &mut OperandStack, address: u16) -> Result<ExecutionResult> {
    if stack.pop_int()? <= 0 {
        return Ok(ContinueAtPosition(usize::from(address)));
    }
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.if_icmp_cond>
#[inline]
pub(crate) fn if_icmpeq(stack: &mut OperandStack, address: u16) -> Result<ExecutionResult> {
    let value2 = stack.pop_int()?;
    let value1 = stack.pop_int()?;
    if value1 == value2 {
        return Ok(ContinueAtPosition(usize::from(address)));
    }
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.if_icmp_cond>
#[inline]
pub(crate) fn if_icmpne(stack: &mut OperandStack, address: u16) -> Result<ExecutionResult> {
    let value2 = stack.pop_int()?;
    let value1 = stack.pop_int()?;
    if value1 != value2 {
        return Ok(ContinueAtPosition(usize::from(address)));
    }
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.if_icmp_cond>
#[inline]
pub(crate) fn if_icmplt(stack: &mut OperandStack, address: u16) -> Result<ExecutionResult> {
    let value2 = stack.pop_int()?;
    let value1 = stack.pop_int()?;
    if value1 < value2 {
        return Ok(ContinueAtPosition(usize::from(address)));
    }
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.if_icmp_cond>
#[inline]
pub(crate) fn if_icmpge(stack: &mut OperandStack, address: u16) -> Result<ExecutionResult> {
    let value2 = stack.pop_int()?;
    let value1 = stack.pop_int()?;
    if value1 >= value2 {
        return Ok(ContinueAtPosition(usize::from(address)));
    }
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.if_icmp_cond>
#[inline]
pub(crate) fn if_icmpgt(stack: &mut OperandStack, address: u16) -> Result<ExecutionResult> {
    let value2 = stack.pop_int()?;
    let value1 = stack.pop_int()?;
    if value1 > value2 {
        return Ok(ContinueAtPosition(usize::from(address)));
    }
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.if_icmp_cond>
#[inline]
pub(crate) fn if_icmple(stack: &mut OperandStack, address: u16) -> Result<ExecutionResult> {
    let value2 = stack.pop_int()?;
    let value1 = stack.pop_int()?;
    if value1 <= value2 {
        return Ok(ContinueAtPosition(usize::from(address)));
    }
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.if_acmp_cond>
#[inline]
pub(crate) fn if_acmpeq(stack: &mut OperandStack, address: u16) -> Result<ExecutionResult> {
    let value2 = stack.pop_object()?;
    let value1 = stack.pop_object()?;
    if value1 == value2 {
        Ok(ContinueAtPosition(usize::from(address)))
    } else {
        Ok(Continue)
    }
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.if_acmp_cond>
#[inline]
pub(crate) fn if_acmpne(stack: &mut OperandStack, address: u16) -> Result<ExecutionResult> {
    let value2 = stack.pop_object()?;
    let value1 = stack.pop_object()?;
    if value1 != value2 {
        Ok(ContinueAtPosition(usize::from(address)))
    } else {
        Ok(Continue)
    }
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.goto>
#[expect(clippy::unnecessary_wraps)]
#[inline]
pub(crate) fn goto(address: u16) -> Result<ExecutionResult> {
    Ok(ContinueAtPosition(usize::from(address)))
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.goto_w>
/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.wide>
#[inline]
pub(crate) fn goto_w(address: i32) -> Result<ExecutionResult> {
    Ok(ContinueAtPosition(usize::try_from(address)?))
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.jsr>
#[inline]
pub(crate) fn jsr(stack: &mut OperandStack, address: u16) -> Result<ExecutionResult> {
    let address = i32::from(address);
    stack.push_int(address)?;
    Ok(ContinueAtPosition(usize::try_from(address)?))
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.jsr>
/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.wide>
#[inline]
pub(crate) fn jsr_w(stack: &mut OperandStack, address: i32) -> Result<ExecutionResult> {
    stack.push_int(address)?;
    Ok(ContinueAtPosition(usize::try_from(address)?))
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.ret>
#[inline]
pub(crate) fn ret(locals: &LocalVariables, index: u8) -> Result<ExecutionResult> {
    let address = locals.get_int(usize::from(index))?;
    Ok(ContinueAtPosition(usize::try_from(address)?))
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.ret>
/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.wide>
#[inline]
pub(crate) fn ret_w(locals: &LocalVariables, index: u16) -> Result<ExecutionResult> {
    let address = locals.get_int(usize::from(index))?;
    Ok(ContinueAtPosition(usize::try_from(address)?))
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.tableswitch>
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
    let address = if key < low || key > high {
        usize::try_from(default)?
    } else {
        let index = usize::try_from(key - low)?;
        usize::try_from(offsets[index])?
    };
    Ok(ContinueAtPosition(program_counter + address))
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.lookupswitch>
#[inline]
pub(crate) fn lookupswitch(
    stack: &mut OperandStack,
    program_counter: usize,
    default: i32,
    pairs: &IndexMap<i32, i32>,
) -> Result<ExecutionResult> {
    let key = stack.pop_int()?;
    let address = match pairs.get(&key) {
        Some(offset) => usize::try_from(*offset)?,
        None => usize::try_from(default)?,
    };
    Ok(ContinueAtPosition(program_counter + address))
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.return>
#[expect(clippy::unnecessary_wraps)]
#[inline]
pub(crate) fn r#return() -> Result<ExecutionResult> {
    Ok(Return(None))
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.ifnull>
#[inline]
pub(crate) fn ifnull(stack: &mut OperandStack, address: u16) -> Result<ExecutionResult> {
    if stack.pop_object()?.is_none() {
        return Ok(ContinueAtPosition(usize::from(address)));
    }
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.ifnonnull>
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
    use crate::VM;
    use crate::java_object::JavaObject;
    use ristretto_classloader::Reference;

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
        let object = Reference::from(vec![42i8]);
        stack.push_object(Some(object.clone()))?;
        stack.push_object(Some(object.clone()))?;
        let result = if_acmpeq(stack, 3)?;
        assert_eq!(ContinueAtPosition(3), result);
        Ok(())
    }

    #[test]
    fn test_if_acmpeq_not_equal() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        let object = Reference::from(vec![42i8]);
        stack.push_object(None)?;
        stack.push_object(Some(object.clone()))?;
        let result = if_acmpeq(stack, 3)?;
        assert_eq!(Continue, result);
        stack.push_object(Some(object.clone()))?;
        stack.push_object(Some(object.clone()))?;
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
        let vm = VM::default().await?;
        let class = vm.class("java.lang.String").await?;
        let class1 = class.to_object(&vm).await?;
        let class2 = class.to_object(&vm).await?;

        let stack = &mut OperandStack::with_max_size(2);
        stack.push_object(Some(class1.try_into()?))?;
        stack.push_object(Some(class2.try_into()?))?;
        let result = if_acmpeq(stack, 3)?;
        assert_eq!(ContinueAtPosition(3), result);
        Ok(())
    }

    #[tokio::test]
    async fn test_if_acmpeq_class_not_equal() -> Result<()> {
        let vm = VM::default().await?;
        let class1 = vm.class("java.lang.Object").await?;
        let class1 = class1.to_object(&vm).await?;
        let class2 = vm.class("java.lang.String").await?;
        let class2 = class2.to_object(&vm).await?;

        let stack = &mut OperandStack::with_max_size(2);
        stack.push_object(Some(class1.try_into()?))?;
        stack.push_object(Some(class2.try_into()?))?;
        let result = if_acmpeq(stack, 3)?;
        assert_eq!(Continue, result);
        Ok(())
    }

    #[test]
    fn test_if_acmpne_equal() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        let object = Reference::from(vec![42i8]);
        stack.push_object(Some(object.clone()))?;
        stack.push_object(Some(object.clone()))?;
        let result = if_acmpne(stack, 3)?;
        assert_eq!(Continue, result);
        Ok(())
    }

    #[test]
    fn test_if_acmpne_not_equal() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        let object = Reference::from(vec![42i8]);
        stack.push_object(None)?;
        stack.push_object(Some(object.clone()))?;
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
        let vm = VM::default().await?;
        let class = vm.class("java.lang.String").await?;
        let class1 = class.to_object(&vm).await?;
        let class2 = class.to_object(&vm).await?;

        let stack = &mut OperandStack::with_max_size(2);
        stack.push_object(Some(class1.try_into()?))?;
        stack.push_object(Some(class2.try_into()?))?;
        let result = if_acmpne(stack, 3)?;
        assert_eq!(Continue, result);
        Ok(())
    }

    #[tokio::test]
    async fn test_if_acmpne_class_not_equal() -> Result<()> {
        let vm = VM::default().await?;
        let class1 = vm.class("java.lang.Object").await?;
        let class1 = class1.to_object(&vm).await?;
        let class2 = vm.class("java.lang.String").await?;
        let class2 = class2.to_object(&vm).await?;

        let stack = &mut OperandStack::with_max_size(2);
        stack.push_object(Some(class1.try_into()?))?;
        stack.push_object(Some(class2.try_into()?))?;
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

    #[test]
    fn test_return() -> Result<()> {
        let result = r#return()?;
        assert!(matches!(result, Return(None)));
        Ok(())
    }

    #[test]
    fn test_ifnull_not_null() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        let object = Reference::from(vec![42i8]);
        stack.push_object(Some(object))?;
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
        let object = Reference::from(vec![42i8]);
        stack.push_object(Some(object))?;
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
