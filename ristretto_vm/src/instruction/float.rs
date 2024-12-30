use crate::frame::ExecutionResult::Return;
use crate::frame::{ExecutionResult, ExecutionResult::Continue};
use crate::java_error::JavaError::ArithmeticException;
use crate::local_variables::LocalVariables;
use crate::operand_stack::OperandStack;
use crate::Error::InvalidStackValue;
use crate::JavaError::{ArrayIndexOutOfBoundsException, NullPointerException};
use crate::{Result, Value};
use ristretto_classloader::Reference;

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.fconst_f>
#[inline]
pub(crate) fn fconst_0(stack: &mut OperandStack) -> Result<ExecutionResult> {
    stack.push_float(0f32)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.fconst_f>
#[inline]
pub(crate) fn fconst_1(stack: &mut OperandStack) -> Result<ExecutionResult> {
    stack.push_float(1f32)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.fconst_f>
#[inline]
pub(crate) fn fconst_2(stack: &mut OperandStack) -> Result<ExecutionResult> {
    stack.push_float(2f32)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.fload>
#[inline]
pub(crate) fn fload(
    locals: &LocalVariables,
    stack: &mut OperandStack,
    index: u8,
) -> Result<ExecutionResult> {
    let value = locals.get_float(usize::from(index))?;
    stack.push_float(value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.fload>
/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.wide>
#[inline]
pub(crate) fn fload_w(
    locals: &LocalVariables,
    stack: &mut OperandStack,
    index: u16,
) -> Result<ExecutionResult> {
    let value = locals.get_float(usize::from(index))?;
    stack.push_float(value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.fload_n>
#[inline]
pub(crate) fn fload_0(
    locals: &LocalVariables,
    stack: &mut OperandStack,
) -> Result<ExecutionResult> {
    let value = locals.get_float(0)?;
    stack.push_float(value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.fload_n>
#[inline]
pub(crate) fn fload_1(
    locals: &LocalVariables,
    stack: &mut OperandStack,
) -> Result<ExecutionResult> {
    let value = locals.get_float(1)?;
    stack.push_float(value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.fload_n>
#[inline]
pub(crate) fn fload_2(
    locals: &LocalVariables,
    stack: &mut OperandStack,
) -> Result<ExecutionResult> {
    let value = locals.get_float(2)?;
    stack.push_float(value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.fload_n>
#[inline]
pub(crate) fn fload_3(
    locals: &LocalVariables,
    stack: &mut OperandStack,
) -> Result<ExecutionResult> {
    let value = locals.get_float(3)?;
    stack.push_float(value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.fstore>
#[inline]
pub(crate) fn fstore(
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
    index: u8,
) -> Result<ExecutionResult> {
    let value = stack.pop_float()?;
    locals.set_float(usize::from(index), value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.fstore>
/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.wide>
#[inline]
pub(crate) fn fstore_w(
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
    index: u16,
) -> Result<ExecutionResult> {
    let value = stack.pop_float()?;
    locals.set_float(usize::from(index), value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.fstore_n>
#[inline]
pub(crate) fn fstore_0(
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<ExecutionResult> {
    let value = stack.pop_float()?;
    locals.set_float(0, value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.fstore_n>
#[inline]
pub(crate) fn fstore_1(
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<ExecutionResult> {
    let value = stack.pop_float()?;
    locals.set_float(1, value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.fstore_n>
#[inline]
pub(crate) fn fstore_2(
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<ExecutionResult> {
    let value = stack.pop_float()?;
    locals.set_float(2, value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.fstore_n>
#[inline]
pub(crate) fn fstore_3(
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<ExecutionResult> {
    let value = stack.pop_float()?;
    locals.set_float(3, value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.faload>
#[inline]
pub(crate) fn faload(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let index = stack.pop_int()?;
    match stack.pop_object()? {
        None => Err(NullPointerException("array cannot be null".to_string()).into()),
        Some(Reference::FloatArray(array)) => {
            let index = usize::try_from(index)?;
            let Some(value) = array.get(index)? else {
                let length = array.len()?;
                return Err(ArrayIndexOutOfBoundsException { index, length }.into());
            };
            stack.push_float(value)?;
            Ok(Continue)
        }
        Some(object) => Err(InvalidStackValue {
            expected: "float array".to_string(),
            actual: object.to_string(),
        }),
    }
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.fastore>
#[inline]
pub(crate) fn fastore(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let value = stack.pop_float()?;
    let index = stack.pop_int()?;
    match stack.pop_object()? {
        None => Err(NullPointerException("array cannot be null".to_string()).into()),
        Some(Reference::FloatArray(ref mut array)) => {
            let index = usize::try_from(index)?;
            let length = array.capacity()?;
            if index >= length {
                return Err(ArrayIndexOutOfBoundsException { index, length }.into());
            };
            array.set(index, value)?;
            Ok(Continue)
        }
        Some(object) => Err(InvalidStackValue {
            expected: "float array".to_string(),
            actual: object.to_string(),
        }),
    }
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.fadd>
#[inline]
pub(crate) fn fadd(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let value2 = stack.pop_float()?;
    let value1 = stack.pop_float()?;
    stack.push_float(value1 + value2)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.fsub>
#[inline]
pub(crate) fn fsub(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let value2 = stack.pop_float()?;
    let value1 = stack.pop_float()?;
    stack.push_float(value1 - value2)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.fmul>
#[inline]
pub(crate) fn fmul(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let value2 = stack.pop_float()?;
    let value1 = stack.pop_float()?;
    stack.push_float(value1 * value2)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.fdiv>
#[inline]
pub(crate) fn fdiv(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let value2 = stack.pop_float()?;
    let value1 = stack.pop_float()?;

    if value2 == 0.0 {
        return Err(ArithmeticException("/ by zero".to_string()).into());
    };

    stack.push_float(value1 / value2)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.frem>
#[inline]
pub(crate) fn frem(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let value2 = stack.pop_float()?;
    let value1 = stack.pop_float()?;

    if value2 == 0.0 {
        return Err(ArithmeticException("/ by zero".to_string()).into());
    };

    stack.push_float(value1 % value2)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.fneg>
#[inline]
pub(crate) fn fneg(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let value = stack.pop_float()?;
    stack.push_float(-value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.fcmpl>
#[inline]
pub(crate) fn fcmpl(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let value2 = stack.pop_float()?;
    let value1 = stack.pop_float()?;
    let cmp = if f32::is_nan(value1) || f32::is_nan(value2) {
        -1
    } else if value1 > value2 {
        1
    } else if value1 < value2 {
        -1
    } else {
        0
    };
    stack.push_int(cmp)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.fcmpg>
#[inline]
pub(crate) fn fcmpg(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let value2 = stack.pop_float()?;
    let value1 = stack.pop_float()?;
    let cmp = if f32::is_nan(value1) || f32::is_nan(value2) || value1 > value2 {
        1
    } else if value1 < value2 {
        -1
    } else {
        0
    };
    stack.push_int(cmp)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.freturn>
#[inline]
pub(crate) fn freturn(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let value = stack.pop_float()?;
    Ok(Return(Some(Value::Float(value))))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::java_error::JavaError::ArithmeticException;
    use crate::Error::{InvalidOperand, JavaError};

    #[test]
    fn test_fconst_0() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        let result = fconst_0(stack)?;
        assert_eq!(Continue, result);
        let value = stack.pop_float()? - 0f32;
        assert!(value.abs() < 0.1f32);
        Ok(())
    }

    #[test]
    fn test_fconst_1() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        let result = fconst_1(stack)?;
        assert_eq!(Continue, result);
        let value = stack.pop_float()? - 1f32;
        assert!(value.abs() < 0.1f32);
        Ok(())
    }

    #[test]
    fn test_fconst_2() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        let result = fconst_2(stack)?;
        assert_eq!(Continue, result);
        let value = stack.pop_float()? - 2f32;
        assert!(value.abs() < 0.1f32);
        Ok(())
    }

    #[test]
    fn test_fload() -> Result<()> {
        let mut locals = LocalVariables::with_max_size(1);
        locals.set_float(0, 42.1)?;
        let stack = &mut OperandStack::with_max_size(1);
        let result = fload(&locals, stack, 0)?;
        assert_eq!(Continue, result);
        let value = stack.pop_float()? - 42.1f32;
        assert!(value.abs() < 0.1f32);
        Ok(())
    }

    #[test]
    fn test_fload_w() -> Result<()> {
        let mut locals = LocalVariables::with_max_size(1);
        locals.set_float(0, 42.1)?;
        let stack = &mut OperandStack::with_max_size(1);
        let result = fload_w(&locals, stack, 0)?;
        assert_eq!(Continue, result);
        let value = stack.pop_float()? - 42.1f32;
        assert!(value.abs() < 0.1f32);
        Ok(())
    }

    #[test]
    fn test_fload_0() -> Result<()> {
        let mut locals = LocalVariables::with_max_size(1);
        locals.set_float(0, 42.1)?;
        let stack = &mut OperandStack::with_max_size(1);
        let result = fload_0(&locals, stack)?;
        assert_eq!(Continue, result);
        let value = stack.pop_float()? - 42.1f32;
        assert!(value.abs() < 0.1f32);
        Ok(())
    }

    #[test]
    fn test_fload_1() -> Result<()> {
        let mut locals = LocalVariables::with_max_size(2);
        locals.set_float(1, 42.1)?;
        let stack = &mut OperandStack::with_max_size(1);
        let result = fload_1(&locals, stack)?;
        assert_eq!(Continue, result);
        let value = stack.pop_float()? - 42.1f32;
        assert!(value.abs() < 0.1f32);
        Ok(())
    }

    #[test]
    fn test_fload_2() -> Result<()> {
        let mut locals = LocalVariables::with_max_size(3);
        locals.set_float(2, 42.1)?;
        let stack = &mut OperandStack::with_max_size(1);
        let result = fload_2(&locals, stack)?;
        assert_eq!(Continue, result);
        let value = stack.pop_float()? - 42.1f32;
        assert!(value.abs() < 0.1f32);
        Ok(())
    }

    #[test]
    fn test_fload_3() -> Result<()> {
        let mut locals = LocalVariables::with_max_size(4);
        locals.set_float(3, 42.1)?;
        let stack = &mut OperandStack::with_max_size(1);
        let result = fload_3(&locals, stack)?;
        assert_eq!(Continue, result);
        let value = stack.pop_float()? - 42.1f32;
        assert!(value.abs() < 0.1f32);
        Ok(())
    }

    #[test]
    fn test_fstore() -> Result<()> {
        let locals = &mut LocalVariables::with_max_size(1);
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_float(42.1)?;
        let result = fstore(locals, stack, 0)?;
        assert_eq!(Continue, result);
        let value = locals.get_float(0)? - 42.1f32;
        assert!(value.abs() < 0.1f32);
        Ok(())
    }

    #[test]
    fn test_fstore_w() -> Result<()> {
        let locals = &mut LocalVariables::with_max_size(1);
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_float(42.1)?;
        let result = fstore_w(locals, stack, 0)?;
        assert_eq!(Continue, result);
        let value = locals.get_float(0)? - 42.1f32;
        assert!(value.abs() < 0.1f32);
        Ok(())
    }

    #[test]
    fn test_fstore_0() -> Result<()> {
        let locals = &mut LocalVariables::with_max_size(1);
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_float(42.1)?;
        let result = fstore_0(locals, stack)?;
        assert_eq!(Continue, result);
        let value = locals.get_float(0)? - 42.1f32;
        assert!(value.abs() < 0.1f32);
        Ok(())
    }

    #[test]
    fn test_fstore_1() -> Result<()> {
        let locals = &mut LocalVariables::with_max_size(2);
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_float(42.1)?;
        let result = fstore_1(locals, stack)?;
        assert_eq!(Continue, result);
        let value = locals.get_float(1)? - 42.1f32;
        assert!(value.abs() < 0.1f32);
        Ok(())
    }

    #[test]
    fn test_fstore_2() -> Result<()> {
        let locals = &mut LocalVariables::with_max_size(3);
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_float(42.1)?;
        let result = fstore_2(locals, stack)?;
        assert_eq!(Continue, result);
        let value = locals.get_float(2)? - 42.1f32;
        assert!(value.abs() < 0.1f32);
        Ok(())
    }

    #[test]
    fn test_fstore_3() -> Result<()> {
        let locals = &mut LocalVariables::with_max_size(4);
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_float(42.1)?;
        let result = fstore_3(locals, stack)?;
        assert_eq!(Continue, result);
        let value = locals.get_float(3)? - 42.1f32;
        assert!(value.abs() < 0.1f32);
        Ok(())
    }

    #[test]
    fn test_faload() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        let array = Reference::from(vec![42f32]);
        stack.push_object(Some(array))?;
        stack.push_int(0)?;
        let result = faload(stack)?;
        assert_eq!(Continue, result);
        let value = stack.pop_float()? - 42f32;
        assert!(value.abs() < 0.1f32);
        Ok(())
    }

    #[test]
    fn test_faload_invalid_value() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        let object = Reference::from(vec![42i32]);
        stack.push_object(Some(object))?;
        stack.push_int(2)?;
        let result = faload(stack);
        assert!(matches!(
            result,
            Err(InvalidStackValue {
                expected,
                actual
            }) if expected == "float array" && actual == "int[42]"
        ));
        Ok(())
    }

    #[test]
    fn test_faload_invalid_index() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        let array = Reference::from(vec![42f32]);
        stack.push_object(Some(array))?;
        stack.push_int(2)?;
        let result = faload(stack);
        assert!(matches!(
            result,
            Err(JavaError(ArrayIndexOutOfBoundsException { index, length }))
            if index == 2 && length == 1
        ));
        Ok(())
    }

    #[test]
    fn test_faload_null_pointer() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_object(None)?;
        stack.push_int(0)?;
        let result = faload(stack);
        assert!(matches!(result, Err(JavaError(NullPointerException(_)))));
        Ok(())
    }

    #[test]
    fn test_fastore() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(3);
        let array = Reference::from(vec![3f32]);
        stack.push_object(Some(array))?;
        stack.push_int(0)?;
        stack.push_float(42f32)?;
        let result = fastore(stack)?;
        assert_eq!(Continue, result);
        Ok(())
    }

    #[test]
    fn test_fastore_invalid_value() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(3);
        let object = Reference::from(vec![42i32]);
        stack.push_object(Some(object))?;
        stack.push_int(2)?;
        stack.push_float(42f32)?;
        let result = fastore(stack);
        assert!(matches!(
            result,
            Err(InvalidStackValue {
                expected,
                actual
            }) if expected == "float array" && actual == "int[42]"
        ));
        Ok(())
    }

    #[test]
    fn test_fastore_invalid_index() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(3);
        let array = Reference::from(vec![3f32]);
        stack.push_object(Some(array))?;
        stack.push_int(2)?;
        stack.push_float(42f32)?;
        let result = fastore(stack);
        assert!(matches!(
            result,
            Err(JavaError(ArrayIndexOutOfBoundsException { index, length }))
            if index == 2 && length == 1
        ));
        Ok(())
    }

    #[test]
    fn test_fastore_null_pointer() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(3);
        stack.push_object(None)?;
        stack.push_int(0)?;
        stack.push_float(42f32)?;
        let result = fastore(stack);
        assert!(matches!(result, Err(JavaError(NullPointerException(_)))));
        Ok(())
    }

    #[test]
    fn test_fadd() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_float(1f32)?;
        stack.push_float(2f32)?;
        let result = fadd(stack)?;
        assert_eq!(Continue, result);
        let value = stack.pop_float()? - 3f32;
        assert!(value.abs() < 0.1f32);
        Ok(())
    }

    #[test]
    fn test_fadd_overflow() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_float(f32::MAX)?;
        stack.push_float(1f32)?;
        let result = fadd(stack)?;
        assert_eq!(Continue, result);
        let value = stack.pop_float()? - f32::MAX;
        assert!(value.abs() < 0.1f32);
        Ok(())
    }

    #[test]
    fn test_fsub() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_float(2f32)?;
        stack.push_float(1f32)?;
        let result = fsub(stack)?;
        assert_eq!(Continue, result);
        let value = stack.pop_float()? - 1f32;
        assert!(value.abs() < 0.1f32);
        Ok(())
    }

    #[test]
    fn test_fsub_overflow() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_float(f32::MIN)?;
        stack.push_float(1f32)?;
        let result = fsub(stack)?;
        assert_eq!(Continue, result);
        let value = stack.pop_float()? - f32::MIN;
        assert!(value.abs() < 0.1f32);
        Ok(())
    }

    #[test]
    fn test_fmul() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_float(2f32)?;
        stack.push_float(3f32)?;
        let result = fmul(stack)?;
        assert_eq!(Continue, result);
        let value = stack.pop_float()? - 6f32;
        assert!(value.abs() < 0.1f32);
        Ok(())
    }

    #[test]
    fn test_fdiv() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_float(6f32)?;
        stack.push_float(3f32)?;
        let result = fdiv(stack)?;
        assert_eq!(Continue, result);
        let value = stack.pop_float()? - 2f32;
        assert!(value.abs() < 0.1f32);
        Ok(())
    }

    #[test]
    fn test_fdiv_divide_by_zero() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_float(1.0)?;
        stack.push_float(0.0)?;
        let result = fdiv(stack);
        assert!(matches!(result, Err(JavaError(ArithmeticException(_)))));
        Ok(())
    }

    #[test]
    fn test_frem() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_float(1f32)?;
        stack.push_float(2f32)?;
        let result = frem(stack)?;
        assert_eq!(Continue, result);
        let value = stack.pop_float()? - 1f32;
        assert!(value.abs() < 0.1f32);
        Ok(())
    }

    #[test]
    fn test_frem_divide_by_zero() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_float(1.0)?;
        stack.push_float(0.0)?;
        let result = frem(stack);
        assert!(matches!(result, Err(JavaError(ArithmeticException(_)))));
        Ok(())
    }

    #[test]
    fn test_fneg() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_float(1f32)?;
        let result = fneg(stack)?;
        assert_eq!(Continue, result);
        let value = stack.pop_float()? + 1f32;
        assert!(value.abs() < 0.1f32);
        Ok(())
    }

    #[test]
    fn test_fcmpl_equal() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_float(1.0)?;
        stack.push_float(1.0)?;
        let result = fcmpl(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(0, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_fcmpl_value1_nan() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_float(1.0)?;
        stack.push_float(f32::NAN)?;
        let result = fcmpl(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(-1, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_fcmpl_value2_nan() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_float(f32::NAN)?;
        stack.push_float(1.0)?;
        let result = fcmpl(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(-1, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_fcmpl_greater_than() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_float(2.0)?;
        stack.push_float(1.0)?;
        let result = fcmpl(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(1, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_fcmpl_less_than() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_float(1.0)?;
        stack.push_float(2.0)?;
        let result = fcmpl(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(-1, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_fcmpg_equal() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_float(1.0)?;
        stack.push_float(1.0)?;
        let result = fcmpg(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(0, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_fcmpg_value1_nan() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_float(1.0)?;
        stack.push_float(f32::NAN)?;
        let result = fcmpg(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(1, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_fcmpg_value2_nan() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_float(f32::NAN)?;
        stack.push_float(1.0)?;
        let result = fcmpg(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(1, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_fcmpg_greater_than() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_float(2.0)?;
        stack.push_float(1.0)?;
        let result = fcmpg(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(1, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_fcmpg_less_than() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_float(1.0)?;
        stack.push_float(2.0)?;
        let result = fcmpg(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(-1, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_freturn() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_float(42.1)?;
        let result = freturn(stack)?;
        assert!(matches!(result, Return(Some(Value::Float(42.1)))));
        Ok(())
    }

    #[test]
    fn test_freturn_invalid_type() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_object(None)?;
        let result = freturn(stack);
        assert!(matches!(
            result,
            Err(InvalidOperand {
                expected,
                actual
            }) if expected == "float" && actual == "Object(null)"
        ));
        Ok(())
    }
}
