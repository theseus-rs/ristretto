use crate::Error::InvalidStackValue;
use crate::JavaError::{ArrayIndexOutOfBoundsException, NullPointerException};
use crate::frame::ExecutionResult::Return;
use crate::frame::{ExecutionResult, ExecutionResult::Continue};
use crate::java_error::JavaError::ArithmeticException;
use crate::local_variables::LocalVariables;
use crate::operand_stack::OperandStack;
use crate::{Result, Value};
use ristretto_classloader::Reference;

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.iconst_i>
#[inline]
pub(crate) fn iconst_m1(stack: &mut OperandStack) -> Result<ExecutionResult> {
    stack.push_int(-1)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.iconst_i>
#[inline]
pub(crate) fn iconst_0(stack: &mut OperandStack) -> Result<ExecutionResult> {
    stack.push_int(0)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.iconst_i>
#[inline]
pub(crate) fn iconst_1(stack: &mut OperandStack) -> Result<ExecutionResult> {
    stack.push_int(1)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.iconst_i>
#[inline]
pub(crate) fn iconst_2(stack: &mut OperandStack) -> Result<ExecutionResult> {
    stack.push_int(2)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.iconst_i>
#[inline]
pub(crate) fn iconst_3(stack: &mut OperandStack) -> Result<ExecutionResult> {
    stack.push_int(3)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.iconst_i>
#[inline]
pub(crate) fn iconst_4(stack: &mut OperandStack) -> Result<ExecutionResult> {
    stack.push_int(4)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.iconst_i>
#[inline]
pub(crate) fn iconst_5(stack: &mut OperandStack) -> Result<ExecutionResult> {
    stack.push_int(5)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.iload>
#[inline]
pub(crate) fn iload(
    locals: &LocalVariables,
    stack: &mut OperandStack,
    index: u8,
) -> Result<ExecutionResult> {
    let value = locals.get_int(usize::from(index))?;
    stack.push_int(value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.iload>
/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.wide>
#[inline]
pub(crate) fn iload_w(
    locals: &LocalVariables,
    stack: &mut OperandStack,
    index: u16,
) -> Result<ExecutionResult> {
    let value = locals.get_int(usize::from(index))?;
    stack.push_int(value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.iload_n>
#[inline]
pub(crate) fn iload_0(
    locals: &LocalVariables,
    stack: &mut OperandStack,
) -> Result<ExecutionResult> {
    let value = locals.get_int(0)?;
    stack.push_int(value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.iload_n>
#[inline]
pub(crate) fn iload_1(
    locals: &LocalVariables,
    stack: &mut OperandStack,
) -> Result<ExecutionResult> {
    let value = locals.get_int(1)?;
    stack.push_int(value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.iload_n>
#[inline]
pub(crate) fn iload_2(
    locals: &LocalVariables,
    stack: &mut OperandStack,
) -> Result<ExecutionResult> {
    let value = locals.get_int(2)?;
    stack.push_int(value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.iload_n>
#[inline]
pub(crate) fn iload_3(
    locals: &LocalVariables,
    stack: &mut OperandStack,
) -> Result<ExecutionResult> {
    let value = locals.get_int(3)?;
    stack.push_int(value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.istore>
#[inline]
pub(crate) fn istore(
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
    index: u8,
) -> Result<ExecutionResult> {
    let value = stack.pop_int()?;
    locals.set_int(usize::from(index), value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.istore>
/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.wide>
#[inline]
pub(crate) fn istore_w(
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
    index: u16,
) -> Result<ExecutionResult> {
    let value = stack.pop_int()?;
    locals.set_int(usize::from(index), value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.istore_n>
#[inline]
pub(crate) fn istore_0(
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<ExecutionResult> {
    let value = stack.pop_int()?;
    locals.set_int(0, value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.istore_n>
#[inline]
pub(crate) fn istore_1(
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<ExecutionResult> {
    let value = stack.pop_int()?;
    locals.set_int(1, value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.istore_n>
#[inline]
pub(crate) fn istore_2(
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<ExecutionResult> {
    let value = stack.pop_int()?;
    locals.set_int(2, value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.istore_n>
#[inline]
pub(crate) fn istore_3(
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<ExecutionResult> {
    let value = stack.pop_int()?;
    locals.set_int(3, value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.iaload>
#[inline]
pub(crate) fn iaload(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let index = stack.pop_int()?;
    match stack.pop_object()? {
        None => Err(NullPointerException("array cannot be null".to_string()).into()),
        Some(Reference::IntArray(array)) => {
            let array = array.read();
            let original_index = index;
            let length = array.len();
            let index = usize::try_from(index).map_err(|_| ArrayIndexOutOfBoundsException {
                index: original_index,
                length,
            })?;
            let Some(value) = array.get(index) else {
                return Err(ArrayIndexOutOfBoundsException {
                    index: original_index,
                    length,
                }
                .into());
            };
            stack.push_int(*value)?;
            Ok(Continue)
        }
        Some(object) => Err(InvalidStackValue {
            expected: "int array".to_string(),
            actual: object.to_string(),
        }),
    }
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.iastore>
#[inline]
pub(crate) fn iastore(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let value = stack.pop_int()?;
    let index = stack.pop_int()?;
    match stack.pop_object()? {
        None => Err(NullPointerException("array cannot be null".to_string()).into()),
        Some(Reference::IntArray(array)) => {
            let mut array = array.write();
            let length = array.capacity();
            let original_index = index;
            let index = usize::try_from(index).map_err(|_| ArrayIndexOutOfBoundsException {
                index: original_index,
                length,
            })?;
            if let Some(element) = array.get_mut(index) {
                *element = value;
            } else {
                return Err(ArrayIndexOutOfBoundsException {
                    index: original_index,
                    length,
                }
                .into());
            }
            Ok(Continue)
        }
        Some(object) => Err(InvalidStackValue {
            expected: "int array".to_string(),
            actual: object.to_string(),
        }),
    }
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.iadd>
#[inline]
pub(crate) fn iadd(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let value2 = stack.pop_int()?;
    let value1 = stack.pop_int()?;
    stack.push_int(i32::wrapping_add(value1, value2))?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.isub>
#[inline]
pub(crate) fn isub(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let value2 = stack.pop_int()?;
    let value1 = stack.pop_int()?;
    stack.push_int(i32::wrapping_sub(value1, value2))?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.imul>
#[inline]
pub(crate) fn imul(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let value2 = stack.pop_int()?;
    let value1 = stack.pop_int()?;
    stack.push_int(i32::wrapping_mul(value1, value2))?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.idiv>
#[inline]
pub(crate) fn idiv(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let value2 = stack.pop_int()?;
    let value1 = stack.pop_int()?;

    if value2 == 0 {
        return Err(ArithmeticException("/ by zero".to_string()).into());
    }

    stack.push_int(value1.wrapping_div(value2))?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.irem>
#[inline]
pub(crate) fn irem(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let value2 = stack.pop_int()?;
    let value1 = stack.pop_int()?;

    if value2 == 0 {
        return Err(ArithmeticException("/ by zero".to_string()).into());
    }

    stack.push_int(value1.wrapping_rem(value2))?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.ineg>
#[inline]
pub(crate) fn ineg(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let value = stack.pop_int()?;
    stack.push_int(value.wrapping_neg())?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.ishl>
#[inline]
pub(crate) fn ishl(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let value2 = stack.pop_int()?;
    let value1 = stack.pop_int()?;
    stack.push_int(value1 << (value2 & 0x1f))?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.ishr>
#[inline]
pub(crate) fn ishr(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let value2 = stack.pop_int()?;
    let value1 = stack.pop_int()?;
    stack.push_int(value1 >> (value2 & 0x1f))?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.iushr>
#[inline]
pub(crate) fn iushr(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let value2 = stack.pop_int()?;
    let value1 = stack.pop_int()?;
    let result = if value1 > 0 {
        value1 >> (value2 & 0x1f)
    } else {
        #[expect(clippy::cast_sign_loss)]
        let value1 = value1 as u32;
        let result = value1 >> (value2 & 0x1f);
        #[expect(clippy::cast_possible_wrap)]
        let result = result as i32;
        result
    };
    stack.push_int(result)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.iand>
#[inline]
pub(crate) fn iand(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let value2 = stack.pop_int()?;
    let value1 = stack.pop_int()?;
    stack.push_int(value1 & value2)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.ior>
#[inline]
pub(crate) fn ior(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let value2 = stack.pop_int()?;
    let value1 = stack.pop_int()?;
    stack.push_int(value1 | value2)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.ixor>
#[inline]
pub(crate) fn ixor(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let value2 = stack.pop_int()?;
    let value1 = stack.pop_int()?;
    stack.push_int(value1 ^ value2)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.iinc>
#[inline]
pub(crate) fn iinc(
    locals: &mut LocalVariables,
    index: u8,
    constant: i8,
) -> Result<ExecutionResult> {
    let index = usize::from(index);
    let local = locals.get_int(index)?;
    locals.set_int(index, local.wrapping_add(i32::from(constant)))?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.iinc>
/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.wide>
#[inline]
pub(crate) fn iinc_w(
    locals: &mut LocalVariables,
    index: u16,
    constant: i16,
) -> Result<ExecutionResult> {
    let index = usize::from(index);
    let local = locals.get_int(index)?;
    locals.set_int(index, local.wrapping_add(i32::from(constant)))?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.ireturn>
#[inline]
pub(crate) fn ireturn(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let value = stack.pop_int()?;
    Ok(Return(Some(Value::Int(value))))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error::{InvalidOperand, JavaError};
    use crate::java_error::JavaError::ArithmeticException;

    #[test]
    fn test_iconst_m1() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        let result = iconst_m1(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(-1, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_iconst_0() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        let result = iconst_0(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(0, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_iconst_1() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        let result = iconst_1(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(1, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_iconst_2() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        let result = iconst_2(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(2, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_iconst_3() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        let result = iconst_3(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(3, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_iconst_4() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        let result = iconst_4(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(4, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_iconst_5() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        let result = iconst_5(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(5, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_iload() -> Result<()> {
        let mut locals = LocalVariables::with_max_size(1);
        locals.set_int(0, 42)?;
        let stack = &mut OperandStack::with_max_size(1);
        let result = iload(&locals, stack, 0)?;
        assert_eq!(Continue, result);
        assert_eq!(42, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_iload_w() -> Result<()> {
        let mut locals = LocalVariables::with_max_size(1);
        locals.set_int(0, 42)?;
        let stack = &mut OperandStack::with_max_size(1);
        let result = iload_w(&locals, stack, 0)?;
        assert_eq!(Continue, result);
        assert_eq!(42, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_iload_0() -> Result<()> {
        let mut locals = LocalVariables::with_max_size(1);
        locals.set_int(0, 42)?;
        let stack = &mut OperandStack::with_max_size(1);
        let result = iload_0(&locals, stack)?;
        assert_eq!(Continue, result);
        assert_eq!(42, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_iload_1() -> Result<()> {
        let mut locals = LocalVariables::with_max_size(2);
        locals.set_int(1, 42)?;
        let stack = &mut OperandStack::with_max_size(1);
        let result = iload_1(&locals, stack)?;
        assert_eq!(Continue, result);
        assert_eq!(42, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_iload_2() -> Result<()> {
        let mut locals = LocalVariables::with_max_size(3);
        locals.set_int(2, 42)?;
        let stack = &mut OperandStack::with_max_size(1);
        let result = iload_2(&locals, stack)?;
        assert_eq!(Continue, result);
        assert_eq!(42, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_iload_3() -> Result<()> {
        let mut locals = LocalVariables::with_max_size(4);
        locals.set_int(3, 42)?;
        let stack = &mut OperandStack::with_max_size(1);
        let result = iload_3(&locals, stack)?;
        assert_eq!(Continue, result);
        assert_eq!(42, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_istore() -> Result<()> {
        let locals = &mut LocalVariables::with_max_size(1);
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(42)?;
        let result = istore(locals, stack, 0)?;
        assert_eq!(Continue, result);
        assert_eq!(42, locals.get_int(0)?);
        Ok(())
    }

    #[test]
    fn test_istore_w() -> Result<()> {
        let locals = &mut LocalVariables::with_max_size(1);
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(42)?;
        let result = istore_w(locals, stack, 0)?;
        assert_eq!(Continue, result);
        assert_eq!(42, locals.get_int(0)?);
        Ok(())
    }

    #[test]
    fn test_istore_0() -> Result<()> {
        let locals = &mut LocalVariables::with_max_size(1);
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(42)?;
        let result = istore_0(locals, stack)?;
        assert_eq!(Continue, result);
        assert_eq!(42, locals.get_int(0)?);
        Ok(())
    }

    #[test]
    fn test_istore_1() -> Result<()> {
        let locals = &mut LocalVariables::with_max_size(2);
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(42)?;
        let result = istore_1(locals, stack)?;
        assert_eq!(Continue, result);
        assert_eq!(42, locals.get_int(1)?);
        Ok(())
    }

    #[test]
    fn test_istore_2() -> Result<()> {
        let locals = &mut LocalVariables::with_max_size(3);
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(42)?;
        let result = istore_2(locals, stack)?;
        assert_eq!(Continue, result);
        assert_eq!(42, locals.get_int(2)?);
        Ok(())
    }

    #[test]
    fn test_istore_3() -> Result<()> {
        let locals = &mut LocalVariables::with_max_size(4);
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(42)?;
        let result = istore_3(locals, stack)?;
        assert_eq!(Continue, result);
        assert_eq!(42, locals.get_int(3)?);
        Ok(())
    }

    #[test]
    fn test_iaload() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        let array = Value::from(vec![42i32]);
        stack.push(array)?;
        stack.push_int(0)?;
        let result = iaload(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(42, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_iaload_invalid_value() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        let object = Value::from(vec![42i8]);
        stack.push(object)?;
        stack.push_int(2)?;
        let result = iaload(stack);
        assert!(matches!(
            result,
            Err(InvalidStackValue {
                expected,
                actual
            }) if expected == "int array" && actual == "byte[42]"
        ));
        Ok(())
    }

    #[test]
    fn test_iaload_invalid_index_negative() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        let array = Value::from(vec![42i32]);
        stack.push(array)?;
        stack.push_int(-1)?;
        let result = iaload(stack);
        assert!(matches!(
            result,
            Err(JavaError(ArrayIndexOutOfBoundsException { index, length }))
            if index == -1 && length == 1
        ));
        Ok(())
    }

    #[test]
    fn test_iaload_invalid_index() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        let array = Value::from(vec![42i32]);
        stack.push(array)?;
        stack.push_int(2)?;
        let result = iaload(stack);
        assert!(matches!(
            result,
            Err(JavaError(ArrayIndexOutOfBoundsException { index, length }))
            if index == 2 && length == 1
        ));
        Ok(())
    }

    #[test]
    fn test_iaload_null_pointer() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_object(None)?;
        stack.push_int(0)?;
        let result = iaload(stack);
        assert!(matches!(result, Err(JavaError(NullPointerException(_)))));
        Ok(())
    }

    #[test]
    fn test_iastore() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(3);
        let array = Value::from(vec![3i32]);
        stack.push(array)?;
        stack.push_int(0)?;
        stack.push_int(42)?;
        let result = iastore(stack)?;
        assert_eq!(Continue, result);
        Ok(())
    }

    #[test]
    fn test_iastore_invalid_value() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(3);
        let object = Value::from(vec![42i8]);
        stack.push(object)?;
        stack.push_int(2)?;
        stack.push_int(42)?;
        let result = iastore(stack);
        assert!(matches!(
            result,
            Err(InvalidStackValue {
                expected,
                actual
            }) if expected == "int array" && actual == "byte[42]"
        ));
        Ok(())
    }

    #[test]
    fn test_iastore_invalid_index_negative() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(3);
        let array = Value::from(vec![3i32]);
        stack.push(array)?;
        stack.push_int(-1)?;
        stack.push_int(42)?;
        let result = iastore(stack);
        assert!(matches!(
            result,
            Err(JavaError(ArrayIndexOutOfBoundsException { index, length }))
            if index == -1 && length == 1
        ));
        Ok(())
    }

    #[test]
    fn test_iastore_invalid_index() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(3);
        let array = Value::from(vec![3i32]);
        stack.push(array)?;
        stack.push_int(2)?;
        stack.push_int(42)?;
        let result = iastore(stack);
        assert!(matches!(
            result,
            Err(JavaError(ArrayIndexOutOfBoundsException { index, length }))
            if index == 2 && length == 1
        ));
        Ok(())
    }

    #[test]
    fn test_iastore_null_pointer() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(3);
        stack.push_object(None)?;
        stack.push_int(0)?;
        stack.push_int(42)?;
        let result = iastore(stack);
        assert!(matches!(result, Err(JavaError(NullPointerException(_)))));
        Ok(())
    }

    #[test]
    fn test_iadd() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_int(1)?;
        stack.push_int(2)?;
        let result = iadd(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(3, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_iadd_overflow() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_int(i32::MAX)?;
        stack.push_int(1)?;
        let result = iadd(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(i32::MIN, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_isub() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_int(2)?;
        stack.push_int(1)?;
        let result = isub(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(1, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_isub_overflow() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_int(i32::MIN)?;
        stack.push_int(1)?;
        let result = isub(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(i32::MAX, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_imul() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_int(2)?;
        stack.push_int(3)?;
        let result = imul(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(6, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_idiv() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_int(6)?;
        stack.push_int(3)?;
        let result = idiv(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(2, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_idiv_divide_by_zero() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_int(1)?;
        stack.push_int(0)?;
        let result = idiv(stack);
        assert!(matches!(result, Err(JavaError(ArithmeticException(_)))));
        Ok(())
    }

    #[test]
    fn test_irem() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_int(1)?;
        stack.push_int(2)?;
        let result = irem(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(1, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_irem_divide_by_zero() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_int(1)?;
        stack.push_int(0)?;
        let result = irem(stack);
        assert!(matches!(result, Err(JavaError(ArithmeticException(_)))));
        Ok(())
    }

    #[test]
    fn test_ineg() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(1)?;
        let result = ineg(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(-1, stack.pop_int()?);
        Ok(())
    }
    #[test]
    fn test_ineg_minimum() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(i32::MIN)?;
        let result = ineg(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(i32::MIN, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_ishl() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_int(4)?;
        stack.push_int(1)?;
        let result = ishl(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(8, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_ishr() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_int(4)?;
        stack.push_int(1)?;
        let result = ishr(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(2, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_iushr() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_int(4)?;
        stack.push_int(1)?;
        let result = iushr(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(2, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_iushr_negative_value1() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_int(-1)?;
        stack.push_int(28)?;
        let result = iushr(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(15, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_iand() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_int(2)?;
        stack.push_int(3)?;
        let result = iand(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(2, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_ior() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_int(2)?;
        stack.push_int(4)?;
        let result = ior(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(6, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_ixor() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_int(2)?;
        stack.push_int(3)?;
        let result = ixor(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(1, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_iinc() -> Result<()> {
        let locals = &mut LocalVariables::with_max_size(1);
        locals.set_int(0, 1)?;
        let result = iinc(locals, 0, 2)?;
        assert_eq!(Continue, result);
        assert_eq!(3, locals.get_int(0)?);
        Ok(())
    }

    #[test]
    fn test_iinc_w() -> Result<()> {
        let locals = &mut LocalVariables::with_max_size(1);
        locals.set_int(0, 1)?;
        let result = iinc_w(locals, 0, 2)?;
        assert_eq!(Continue, result);
        assert_eq!(3, locals.get_int(0)?);
        Ok(())
    }

    #[test]
    fn test_ireturn() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(42)?;
        let result = ireturn(stack)?;
        assert!(matches!(result, Return(Some(Value::Int(42)))));
        Ok(())
    }

    #[test]
    fn test_ireturn_invalid_type() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_object(None)?;
        let result = ireturn(stack);
        assert!(matches!(
            result,
            Err(InvalidOperand {
                expected,
                actual
            }) if expected == "int" && actual == "Object(null)"
        ));
        Ok(())
    }
}
