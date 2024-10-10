use crate::frame::ExecutionResult::Return;
use crate::frame::{ExecutionResult, ExecutionResult::Continue};
use crate::local_variables::LocalVariables;
use crate::operand_stack::OperandStack;
use crate::Error::{ArrayIndexOutOfBounds, InvalidStackValue, NullPointer};
use crate::{Result, Value};
use ristretto_classloader::Reference;
use std::cmp::Ordering;

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.lconst_l>
#[inline]
pub(crate) fn lconst_0(stack: &mut OperandStack) -> Result<ExecutionResult> {
    stack.push_long(0)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.lconst_l>
#[inline]
pub(crate) fn lconst_1(stack: &mut OperandStack) -> Result<ExecutionResult> {
    stack.push_long(1)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.lload>
#[inline]
pub(crate) fn lload(
    locals: &LocalVariables,
    stack: &mut OperandStack,
    index: u8,
) -> Result<ExecutionResult> {
    let value = locals.get_long(usize::from(index))?;
    stack.push_long(value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.lload>
/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.wide>
#[inline]
pub(crate) fn lload_w(
    locals: &LocalVariables,
    stack: &mut OperandStack,
    index: u16,
) -> Result<ExecutionResult> {
    let value = locals.get_long(usize::from(index))?;
    stack.push_long(value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.lload_n>
#[inline]
pub(crate) fn lload_0(
    locals: &LocalVariables,
    stack: &mut OperandStack,
) -> Result<ExecutionResult> {
    let value = locals.get_long(0)?;
    stack.push_long(value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.lload_n>
#[inline]
pub(crate) fn lload_1(
    locals: &LocalVariables,
    stack: &mut OperandStack,
) -> Result<ExecutionResult> {
    let value = locals.get_long(1)?;
    stack.push_long(value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.lload_n>
#[inline]
pub(crate) fn lload_2(
    locals: &LocalVariables,
    stack: &mut OperandStack,
) -> Result<ExecutionResult> {
    let value = locals.get_long(2)?;
    stack.push_long(value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.lload_n>
#[inline]
pub(crate) fn lload_3(
    locals: &LocalVariables,
    stack: &mut OperandStack,
) -> Result<ExecutionResult> {
    let value = locals.get_long(3)?;
    stack.push_long(value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.lstore>
#[inline]
pub(crate) fn lstore(
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
    index: u8,
) -> Result<ExecutionResult> {
    let value = stack.pop_long()?;
    locals.set_long(usize::from(index), value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.lstore>
/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.wide>
#[inline]
pub(crate) fn lstore_w(
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
    index: u16,
) -> Result<ExecutionResult> {
    let value = stack.pop_long()?;
    locals.set_long(usize::from(index), value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.lstore_n>
#[inline]
pub(crate) fn lstore_0(
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<ExecutionResult> {
    let value = stack.pop_long()?;
    locals.set_long(0, value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.lstore_n>
#[inline]
pub(crate) fn lstore_1(
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<ExecutionResult> {
    let value = stack.pop_long()?;
    locals.set_long(1, value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.lstore_n>
#[inline]
pub(crate) fn lstore_2(
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<ExecutionResult> {
    let value = stack.pop_long()?;
    locals.set_long(2, value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.lstore_n>
#[inline]
pub(crate) fn lstore_3(
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<ExecutionResult> {
    let value = stack.pop_long()?;
    locals.set_long(3, value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.laload>
#[inline]
pub(crate) fn laload(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let index = stack.pop_int()?;
    match stack.pop_object()? {
        None => Err(NullPointer),
        Some(Reference::LongArray(array)) => {
            let index = usize::try_from(index)?;
            let Some(value) = array.get(index)? else {
                return Err(ArrayIndexOutOfBounds(index));
            };
            stack.push_long(value)?;
            Ok(Continue)
        }
        Some(object) => Err(InvalidStackValue {
            expected: "long array".to_string(),
            actual: object.to_string(),
        }),
    }
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.lastore>
#[inline]
pub(crate) fn lastore(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let value = stack.pop_long()?;
    let index = stack.pop_int()?;
    match stack.pop_object()? {
        None => Err(NullPointer),
        Some(Reference::LongArray(ref mut array)) => {
            let index = usize::try_from(index)?;
            if index >= array.capacity()? {
                return Err(ArrayIndexOutOfBounds(index));
            };
            array.set(index, value)?;
            Ok(Continue)
        }
        Some(object) => Err(InvalidStackValue {
            expected: "long array".to_string(),
            actual: object.to_string(),
        }),
    }
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.ladd>
#[inline]
pub(crate) fn ladd(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let value2 = stack.pop_long()?;
    let value1 = stack.pop_long()?;
    stack.push_long(i64::wrapping_add(value1, value2))?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.lsub>
#[inline]
pub(crate) fn lsub(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let value2 = stack.pop_long()?;
    let value1 = stack.pop_long()?;
    stack.push_long(i64::wrapping_sub(value1, value2))?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.lmul>
#[inline]
pub(crate) fn lmul(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let value2 = stack.pop_long()?;
    let value1 = stack.pop_long()?;
    stack.push_long(i64::wrapping_mul(value1, value2))?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.ldiv>
#[inline]
pub(crate) fn ldiv(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let value2 = stack.pop_long()?;
    let value1 = stack.pop_long()?;
    stack.push_long(i64::wrapping_div(value1, value2))?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.lrem>
#[inline]
pub(crate) fn lrem(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let value2 = stack.pop_long()?;
    let value1 = stack.pop_long()?;
    stack.push_long(i64::wrapping_rem(value1, value2))?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.lneg>
#[inline]
pub(crate) fn lneg(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let value = stack.pop_long()?;
    stack.push_long(-value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.lshl>
#[inline]
pub(crate) fn lshl(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let value2 = stack.pop_int()?;
    let value1 = stack.pop_long()?;
    stack.push_long(value1 << (value2 & 0x3f))?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.lshr>
#[inline]
pub(crate) fn lshr(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let value2 = stack.pop_int()?;
    let value1 = stack.pop_long()?;
    stack.push_long(value1 >> (value2 & 0x3f))?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.lushr>
#[inline]
pub(crate) fn lushr(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let value2 = stack.pop_int()?;
    let value1 = stack.pop_long()?;
    let result = if value1 > 0 {
        value1 >> (value2 & 0x1f)
    } else {
        #[expect(clippy::cast_sign_loss)]
        let value1 = value1 as u64;
        let result = value1 >> value2;
        #[expect(clippy::cast_possible_wrap)]
        let result = result as i64;
        result
    };
    stack.push_long(result)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.land>
#[inline]
pub(crate) fn land(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let value2 = stack.pop_long()?;
    let value1 = stack.pop_long()?;
    stack.push_long(value1 & value2)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.lor>
#[inline]
pub(crate) fn lor(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let value2 = stack.pop_long()?;
    let value1 = stack.pop_long()?;
    stack.push_long(value1 | value2)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.lxor>
#[inline]
pub(crate) fn lxor(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let value2 = stack.pop_long()?;
    let value1 = stack.pop_long()?;
    stack.push_long(value1 ^ value2)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.lcmp>
#[inline]
pub(crate) fn lcmp(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let value2 = stack.pop_long()?;
    let value1 = stack.pop_long()?;
    let cmp = match value1.cmp(&value2) {
        Ordering::Greater => 1,
        Ordering::Less => -1,
        Ordering::Equal => 0,
    };
    stack.push_int(cmp)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.lreturn>
#[inline]
pub(crate) fn lreturn(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let value = stack.pop_long()?;
    Ok(Return(Some(Value::Long(value))))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error::InvalidOperand;
    use ristretto_classloader::ConcurrentVec;

    #[test]
    fn test_lconst_0() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        let result = lconst_0(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(0, stack.pop_long()?);
        Ok(())
    }

    #[test]
    fn test_lconst_1() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        let result = lconst_1(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(1, stack.pop_long()?);
        Ok(())
    }

    #[test]
    fn test_lload() -> Result<()> {
        let mut locals = LocalVariables::with_max_size(1);
        locals.set_long(0, 42)?;
        let stack = &mut OperandStack::with_max_size(1);
        let result = lload(&locals, stack, 0)?;
        assert_eq!(Continue, result);
        assert_eq!(42, stack.pop_long()?);
        Ok(())
    }

    #[test]
    fn test_lload_w() -> Result<()> {
        let mut locals = LocalVariables::with_max_size(1);
        locals.set_long(0, 42)?;
        let stack = &mut OperandStack::with_max_size(1);
        let result = lload_w(&locals, stack, 0)?;
        assert_eq!(Continue, result);
        assert_eq!(42, stack.pop_long()?);
        Ok(())
    }

    #[test]
    fn test_lload_0() -> Result<()> {
        let mut locals = LocalVariables::with_max_size(1);
        locals.set_long(0, 42)?;
        let stack = &mut OperandStack::with_max_size(1);
        let result = lload_0(&locals, stack)?;
        assert_eq!(Continue, result);
        assert_eq!(42, stack.pop_long()?);
        Ok(())
    }

    #[test]
    fn test_lload_1() -> Result<()> {
        let mut locals = LocalVariables::with_max_size(2);
        locals.set_long(1, 42)?;
        let stack = &mut OperandStack::with_max_size(1);
        let result = lload_1(&locals, stack)?;
        assert_eq!(Continue, result);
        assert_eq!(42, stack.pop_long()?);
        Ok(())
    }

    #[test]
    fn test_lload_2() -> Result<()> {
        let mut locals = LocalVariables::with_max_size(3);
        locals.set_long(2, 42)?;
        let stack = &mut OperandStack::with_max_size(1);
        let result = lload_2(&locals, stack)?;
        assert_eq!(Continue, result);
        assert_eq!(42, stack.pop_long()?);
        Ok(())
    }

    #[test]
    fn test_lload_3() -> Result<()> {
        let mut locals = LocalVariables::with_max_size(4);
        locals.set_long(3, 42)?;
        let stack = &mut OperandStack::with_max_size(1);
        let result = lload_3(&locals, stack)?;
        assert_eq!(Continue, result);
        assert_eq!(42, stack.pop_long()?);
        Ok(())
    }

    #[test]
    fn test_lstore() -> Result<()> {
        let locals = &mut LocalVariables::with_max_size(1);
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_long(42)?;
        let result = lstore(locals, stack, 0)?;
        assert_eq!(Continue, result);
        assert_eq!(42, locals.get_long(0)?);
        Ok(())
    }

    #[test]
    fn test_lstore_w() -> Result<()> {
        let locals = &mut LocalVariables::with_max_size(1);
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_long(42)?;
        let result = lstore_w(locals, stack, 0)?;
        assert_eq!(Continue, result);
        assert_eq!(42, locals.get_long(0)?);
        Ok(())
    }

    #[test]
    fn test_lstore_0() -> Result<()> {
        let locals = &mut LocalVariables::with_max_size(1);
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_long(42)?;
        let result = lstore_0(locals, stack)?;
        assert_eq!(Continue, result);
        assert_eq!(42, locals.get_long(0)?);
        Ok(())
    }

    #[test]
    fn test_lstore_1() -> Result<()> {
        let locals = &mut LocalVariables::with_max_size(2);
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_long(42)?;
        let result = lstore_1(locals, stack)?;
        assert_eq!(Continue, result);
        assert_eq!(42, locals.get_long(1)?);
        Ok(())
    }

    #[test]
    fn test_lstore_2() -> Result<()> {
        let locals = &mut LocalVariables::with_max_size(3);
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_long(42)?;
        let result = lstore_2(locals, stack)?;
        assert_eq!(Continue, result);
        assert_eq!(42, locals.get_long(2)?);
        Ok(())
    }

    #[test]
    fn test_lstore_3() -> Result<()> {
        let locals = &mut LocalVariables::with_max_size(4);
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_long(42)?;
        let result = lstore_3(locals, stack)?;
        assert_eq!(Continue, result);
        assert_eq!(42, locals.get_long(3)?);
        Ok(())
    }

    #[test]
    fn test_laload() -> crate::Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        let array = Reference::LongArray(ConcurrentVec::from(vec![42]));
        stack.push_object(Some(array))?;
        stack.push_int(0)?;
        let result = laload(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(42, stack.pop_long()?);
        Ok(())
    }

    #[test]
    fn test_laload_invalid_value() -> crate::Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        let object = Reference::IntArray(ConcurrentVec::from(vec![42]));
        stack.push_object(Some(object))?;
        stack.push_int(2)?;
        let result = laload(stack);
        assert!(matches!(
            result,
            Err(InvalidStackValue {
                expected,
                actual
            }) if expected == "long array" && actual == "int[42]"
        ));
        Ok(())
    }

    #[test]
    fn test_laload_invalid_index() -> crate::Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        let array = Reference::LongArray(ConcurrentVec::from(vec![42]));
        stack.push_object(Some(array))?;
        stack.push_int(2)?;
        let result = laload(stack);
        assert!(matches!(result, Err(ArrayIndexOutOfBounds(2))));
        Ok(())
    }

    #[test]
    fn test_laload_null_pointer() -> crate::Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_object(None)?;
        stack.push_int(0)?;
        let result = laload(stack);
        assert!(matches!(result, Err(NullPointer)));
        Ok(())
    }

    #[test]
    fn test_lastore() -> crate::Result<()> {
        let stack = &mut OperandStack::with_max_size(3);
        let array = Reference::LongArray(ConcurrentVec::from(vec![3]));
        stack.push_object(Some(array))?;
        stack.push_int(0)?;
        stack.push_long(42)?;
        let result = lastore(stack)?;
        assert_eq!(Continue, result);
        Ok(())
    }

    #[test]
    fn test_lastore_invalid_value() -> crate::Result<()> {
        let stack = &mut OperandStack::with_max_size(3);
        let object = Reference::IntArray(ConcurrentVec::from(vec![42]));
        stack.push_object(Some(object))?;
        stack.push_int(2)?;
        stack.push_long(42)?;
        let result = lastore(stack);
        assert!(matches!(
            result,
            Err(InvalidStackValue {
                expected,
                actual
            }) if expected == "long array" && actual == "int[42]"
        ));
        Ok(())
    }

    #[test]
    fn test_lastore_invalid_index() -> crate::Result<()> {
        let stack = &mut OperandStack::with_max_size(3);
        let array = Reference::LongArray(ConcurrentVec::from(vec![3]));
        stack.push_object(Some(array))?;
        stack.push_int(2)?;
        stack.push_long(42)?;
        let result = lastore(stack);
        assert!(matches!(result, Err(ArrayIndexOutOfBounds(2))));
        Ok(())
    }

    #[test]
    fn test_lastore_null_pointer() -> crate::Result<()> {
        let stack = &mut OperandStack::with_max_size(3);
        stack.push_object(None)?;
        stack.push_int(0)?;
        stack.push_long(42)?;
        let result = lastore(stack);
        assert!(matches!(result, Err(NullPointer)));
        Ok(())
    }

    #[test]
    fn test_ladd() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_long(1)?;
        stack.push_long(2)?;
        let result = ladd(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(3, stack.pop_long()?);
        Ok(())
    }

    #[test]
    fn test_ladd_overflow() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_long(i64::MAX)?;
        stack.push_long(1)?;
        let result = ladd(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(i64::MIN, stack.pop_long()?);
        Ok(())
    }

    #[test]
    fn test_lsub() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_long(2)?;
        stack.push_long(1)?;
        let result = lsub(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(1, stack.pop_long()?);
        Ok(())
    }

    #[test]
    fn test_lsub_underflow() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_long(i64::MIN)?;
        stack.push_long(1)?;
        let result = lsub(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(i64::MAX, stack.pop_long()?);
        Ok(())
    }

    #[test]
    fn test_lmul() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_long(2)?;
        stack.push_long(3)?;
        let result = lmul(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(6, stack.pop_long()?);
        Ok(())
    }

    #[test]
    fn test_ldiv() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_long(6)?;
        stack.push_long(3)?;
        let result = ldiv(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(2, stack.pop_long()?);
        Ok(())
    }

    #[test]
    fn test_lrem() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_long(1)?;
        stack.push_long(2)?;
        let result = lrem(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(1, stack.pop_long()?);
        Ok(())
    }

    #[test]
    fn test_lneg() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_long(1)?;
        let result = lneg(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(-1, stack.pop_long()?);
        Ok(())
    }

    #[test]
    fn test_lshl() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_long(4)?;
        stack.push_int(1)?;
        let result = lshl(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(8, stack.pop_long()?);
        Ok(())
    }

    #[test]
    fn test_lshr() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_long(4)?;
        stack.push_int(1)?;
        let result = lshr(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(2, stack.pop_long()?);
        Ok(())
    }

    #[test]
    fn test_lushr() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_long(4)?;
        stack.push_int(1)?;
        let result = lushr(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(2, stack.pop_long()?);
        Ok(())
    }

    #[test]
    fn test_lushr_negative_value1() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_long(-1)?;
        stack.push_int(60)?;
        let result = lushr(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(15, stack.pop_long()?);
        Ok(())
    }

    #[test]
    fn test_land() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_long(2)?;
        stack.push_long(3)?;
        let result = land(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(2, stack.pop_long()?);
        Ok(())
    }

    #[test]
    fn test_lor() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_long(2)?;
        stack.push_long(4)?;
        let result = lor(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(6, stack.pop_long()?);
        Ok(())
    }

    #[test]
    fn test_lxor() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_long(2)?;
        stack.push_long(3)?;
        let result = lxor(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(1, stack.pop_long()?);
        Ok(())
    }

    #[test]
    fn test_lcmp_equal() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_long(1)?;
        stack.push_long(1)?;
        let result = lcmp(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(0, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_lcmp_greater_than() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_long(2)?;
        stack.push_long(1)?;
        let result = lcmp(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(1, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_lcmp_less_than() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_long(1)?;
        stack.push_long(2)?;
        let result = lcmp(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(-1, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_lreturn() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_long(42)?;
        let result = lreturn(stack)?;
        assert!(matches!(result, Return(Some(Value::Long(42)))));
        Ok(())
    }

    #[test]
    fn test_lreturn_invalid_type() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_object(None)?;
        let result = lreturn(stack);
        assert!(matches!(
            result,
            Err(InvalidOperand {
                expected,
                actual
            }) if expected == "long" && actual == "object(null)"
        ));
        Ok(())
    }
}
