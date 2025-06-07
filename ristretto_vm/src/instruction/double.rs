use crate::Error::InvalidStackValue;
use crate::JavaError::{ArrayIndexOutOfBoundsException, NullPointerException};
use crate::frame::ExecutionResult::Return;
use crate::frame::{ExecutionResult, ExecutionResult::Continue};
use crate::java_error::JavaError::ArithmeticException;
use crate::local_variables::LocalVariables;
use crate::operand_stack::OperandStack;
use crate::{Result, Value};
use ristretto_classloader::Reference;

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dconst_d>
#[inline]
pub(crate) fn dconst_0(stack: &mut OperandStack) -> Result<ExecutionResult> {
    stack.push_double(0f64)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dconst_d>
#[inline]
pub(crate) fn dconst_1(stack: &mut OperandStack) -> Result<ExecutionResult> {
    stack.push_double(1f64)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dload>
#[inline]
pub(crate) fn dload(
    locals: &LocalVariables,
    stack: &mut OperandStack,
    index: u8,
) -> Result<ExecutionResult> {
    let value = locals.get_double(usize::from(index))?;
    stack.push_double(value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dload>
/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.wide>
#[inline]
pub(crate) fn dload_w(
    locals: &LocalVariables,
    stack: &mut OperandStack,
    index: u16,
) -> Result<ExecutionResult> {
    let value = locals.get_double(usize::from(index))?;
    stack.push_double(value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dload_n>
#[inline]
pub(crate) fn dload_0(
    locals: &LocalVariables,
    stack: &mut OperandStack,
) -> Result<ExecutionResult> {
    let value = locals.get_double(0)?;
    stack.push_double(value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dload_n>
#[inline]
pub(crate) fn dload_1(
    locals: &LocalVariables,
    stack: &mut OperandStack,
) -> Result<ExecutionResult> {
    let value = locals.get_double(1)?;
    stack.push_double(value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dload_n>
#[inline]
pub(crate) fn dload_2(
    locals: &LocalVariables,
    stack: &mut OperandStack,
) -> Result<ExecutionResult> {
    let value = locals.get_double(2)?;
    stack.push_double(value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dload_n>
#[inline]
pub(crate) fn dload_3(
    locals: &LocalVariables,
    stack: &mut OperandStack,
) -> Result<ExecutionResult> {
    let value = locals.get_double(3)?;
    stack.push_double(value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dstore>
#[inline]
pub(crate) fn dstore(
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
    index: u8,
) -> Result<ExecutionResult> {
    let value = stack.pop_double()?;
    locals.set_double(usize::from(index), value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dstore>
/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.wide>
#[inline]
pub(crate) fn dstore_w(
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
    index: u16,
) -> Result<ExecutionResult> {
    let value = stack.pop_double()?;
    locals.set_double(usize::from(index), value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dstore_n>
#[inline]
pub(crate) fn dstore_0(
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<ExecutionResult> {
    let value = stack.pop_double()?;
    locals.set_double(0, value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dstore_n>
#[inline]
pub(crate) fn dstore_1(
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<ExecutionResult> {
    let value = stack.pop_double()?;
    locals.set_double(1, value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dstore_n>
#[inline]
pub(crate) fn dstore_2(
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<ExecutionResult> {
    let value = stack.pop_double()?;
    locals.set_double(2, value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dstore_n>
#[inline]
pub(crate) fn dstore_3(
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<ExecutionResult> {
    let value = stack.pop_double()?;
    locals.set_double(3, value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.daload>
#[inline]
pub(crate) fn daload(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let index = stack.pop_int()?;
    match stack.pop_object()? {
        None => Err(NullPointerException("array cannot be null".to_string()).into()),
        Some(Reference::DoubleArray(array)) => {
            let index = usize::try_from(index)?;
            let Some(value) = array.get(index)? else {
                let length = array.len()?;
                return Err(ArrayIndexOutOfBoundsException { index, length }.into());
            };
            stack.push_double(value)?;
            Ok(Continue)
        }
        Some(object) => Err(InvalidStackValue {
            expected: "double array".to_string(),
            actual: object.to_string(),
        }),
    }
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dastore>
#[inline]
pub(crate) fn dastore(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let value = stack.pop_double()?;
    let index = stack.pop_int()?;
    match stack.pop_object()? {
        None => Err(NullPointerException("array cannot be null".to_string()).into()),
        Some(Reference::DoubleArray(ref mut array)) => {
            let index = usize::try_from(index)?;
            let length = array.capacity()?;
            if index >= length {
                return Err(ArrayIndexOutOfBoundsException { index, length }.into());
            }
            array.set(index, value)?;
            Ok(Continue)
        }
        Some(object) => Err(InvalidStackValue {
            expected: "double array".to_string(),
            actual: object.to_string(),
        }),
    }
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dadd>
#[inline]
pub(crate) fn dadd(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let value2 = stack.pop_double()?;
    let value1 = stack.pop_double()?;
    stack.push_double(value1 + value2)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dsub>
#[inline]
pub(crate) fn dsub(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let value2 = stack.pop_double()?;
    let value1 = stack.pop_double()?;
    stack.push_double(value1 - value2)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dmul>
#[inline]
pub(crate) fn dmul(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let value2 = stack.pop_double()?;
    let value1 = stack.pop_double()?;
    stack.push_double(value1 * value2)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.ddiv>
#[inline]
pub(crate) fn ddiv(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let value2 = stack.pop_double()?;
    let value1 = stack.pop_double()?;

    if value2 == 0.0 {
        return Err(ArithmeticException("/ by zero".to_string()).into());
    }

    stack.push_double(value1 / value2)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.drem>
#[inline]
pub(crate) fn drem(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let value2 = stack.pop_double()?;
    let value1 = stack.pop_double()?;

    if value2 == 0.0 {
        return Err(ArithmeticException("/ by zero".to_string()).into());
    }

    stack.push_double(value1 % value2)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dneg>
#[inline]
pub(crate) fn dneg(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let value = stack.pop_double()?;
    stack.push_double(-value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dcmp_op>
#[inline]
pub(crate) fn dcmpl(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let value2 = stack.pop_double()?;
    let value1 = stack.pop_double()?;
    let cmp = if f64::is_nan(value1) || f64::is_nan(value2) {
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

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dcmp_op>
#[inline]
pub(crate) fn dcmpg(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let value2 = stack.pop_double()?;
    let value1 = stack.pop_double()?;
    let cmp = if f64::is_nan(value1) || f64::is_nan(value2) || value1 > value2 {
        1
    } else if value1 < value2 {
        -1
    } else {
        0
    };
    stack.push_int(cmp)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dreturn>
#[inline]
pub(crate) fn dreturn(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let value = stack.pop_double()?;
    Ok(Return(Some(Value::Double(value))))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error::{InvalidOperand, JavaError};

    #[test]
    fn test_dconst_0() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        let result = dconst_0(stack)?;
        assert_eq!(Continue, result);
        let value = stack.pop_double()? - 0f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[test]
    fn test_dconst_1() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        let result = dconst_1(stack)?;
        assert_eq!(Continue, result);
        let value = stack.pop_double()? - 1f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[test]
    fn test_dload() -> Result<()> {
        let mut locals = LocalVariables::with_max_size(1);
        locals.set_double(0, 42.1)?;
        let stack = &mut OperandStack::with_max_size(1);
        let result = dload(&locals, stack, 0)?;
        assert_eq!(Continue, result);
        let value = stack.pop_double()? - 42.1f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[test]
    fn test_dload_w() -> Result<()> {
        let mut locals = LocalVariables::with_max_size(1);
        locals.set_double(0, 42.1)?;
        let stack = &mut OperandStack::with_max_size(1);
        let result = dload_w(&locals, stack, 0)?;
        assert_eq!(Continue, result);
        let value = stack.pop_double()? - 42.1f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[test]
    fn test_dload_0() -> Result<()> {
        let mut locals = LocalVariables::with_max_size(1);
        locals.set_double(0, 42.1)?;
        let stack = &mut OperandStack::with_max_size(1);
        let result = dload_0(&locals, stack)?;
        assert_eq!(Continue, result);
        let value = stack.pop_double()? - 42.1f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[test]
    fn test_dload_1() -> Result<()> {
        let mut locals = LocalVariables::with_max_size(2);
        locals.set_double(1, 42.1)?;
        let stack = &mut OperandStack::with_max_size(1);
        let result = dload_1(&locals, stack)?;
        assert_eq!(Continue, result);
        let value = stack.pop_double()? - 42.1f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[test]
    fn test_dload_2() -> Result<()> {
        let mut locals = LocalVariables::with_max_size(3);
        locals.set_double(2, 42.1)?;
        let stack = &mut OperandStack::with_max_size(1);
        let result = dload_2(&locals, stack)?;
        assert_eq!(Continue, result);
        let value = stack.pop_double()? - 42.1f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[test]
    fn test_dload_3() -> Result<()> {
        let mut locals = LocalVariables::with_max_size(4);
        locals.set_double(3, 42.1)?;
        let stack = &mut OperandStack::with_max_size(1);
        let result = dload_3(&locals, stack)?;
        assert_eq!(Continue, result);
        let value = stack.pop_double()? - 42.1f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[test]
    fn test_dstore() -> Result<()> {
        let locals = &mut LocalVariables::with_max_size(1);
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_double(42.1)?;
        let result = dstore(locals, stack, 0)?;
        assert_eq!(Continue, result);
        let value = locals.get_double(0)? - 42.1f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[test]
    fn test_dstore_w() -> Result<()> {
        let locals = &mut LocalVariables::with_max_size(1);
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_double(42.1)?;
        let result = dstore_w(locals, stack, 0)?;
        assert_eq!(Continue, result);
        let value = locals.get_double(0)? - 42.1f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[test]
    fn test_dstore_0() -> Result<()> {
        let locals = &mut LocalVariables::with_max_size(1);
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_double(42.1)?;
        let result = dstore_0(locals, stack)?;
        assert_eq!(Continue, result);
        let value = locals.get_double(0)? - 42.1f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[test]
    fn test_dstore_1() -> Result<()> {
        let locals = &mut LocalVariables::with_max_size(2);
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_double(42.1)?;
        let result = dstore_1(locals, stack)?;
        assert_eq!(Continue, result);
        let value = locals.get_double(1)? - 42.1f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[test]
    fn test_dstore_2() -> Result<()> {
        let locals = &mut LocalVariables::with_max_size(3);
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_double(42.1)?;
        let result = dstore_2(locals, stack)?;
        assert_eq!(Continue, result);
        let value = locals.get_double(2)? - 42.1f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[test]
    fn test_dstore_3() -> Result<()> {
        let locals = &mut LocalVariables::with_max_size(4);
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_double(42.1)?;
        let result = dstore_3(locals, stack)?;
        assert_eq!(Continue, result);
        let value = locals.get_double(3)? - 42.1f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[test]
    fn test_daload() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        let array = Reference::from(vec![42f64]);
        stack.push_object(Some(array))?;
        stack.push_int(0)?;
        let result = daload(stack)?;
        assert_eq!(Continue, result);
        let value = stack.pop_double()? - 42f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[test]
    fn test_daload_invalid_value() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        let object = Reference::from(vec![42i32]);
        stack.push_object(Some(object))?;
        stack.push_int(2)?;
        let result = daload(stack);
        assert!(matches!(
            result,
            Err(InvalidStackValue {
                expected,
                actual
            }) if expected == "double array" && actual == "int[42]"
        ));
        Ok(())
    }

    #[test]
    fn test_daload_invalid_index() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        let array = Reference::from(vec![42f64]);
        stack.push_object(Some(array))?;
        stack.push_int(2)?;
        let result = daload(stack);
        assert!(matches!(
            result,
            Err(JavaError(ArrayIndexOutOfBoundsException { index, length }))
            if index == 2 && length == 1
        ));
        Ok(())
    }

    #[test]
    fn test_daload_null_pointer() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_object(None)?;
        stack.push_int(0)?;
        let result = daload(stack);
        assert!(matches!(result, Err(JavaError(NullPointerException(_)))));
        Ok(())
    }

    #[test]
    fn test_dastore() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(3);
        let array = Reference::from(vec![3f64]);
        stack.push_object(Some(array))?;
        stack.push_int(0)?;
        stack.push_double(42f64)?;
        let result = dastore(stack)?;
        assert_eq!(Continue, result);
        Ok(())
    }

    #[test]
    fn test_dastore_invalid_value() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(3);
        let object = Reference::from(vec![42i32]);
        stack.push_object(Some(object))?;
        stack.push_int(2)?;
        stack.push_double(42f64)?;
        let result = dastore(stack);
        assert!(matches!(
            result,
            Err(InvalidStackValue {
                expected,
                actual
            }) if expected == "double array" && actual == "int[42]"
        ));
        Ok(())
    }

    #[test]
    fn test_dastore_invalid_index() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(3);
        let array = Reference::from(vec![3f64]);
        stack.push_object(Some(array))?;
        stack.push_int(2)?;
        stack.push_double(42f64)?;
        let result = dastore(stack);
        assert!(matches!(
            result,
            Err(JavaError(ArrayIndexOutOfBoundsException { index, length }))
            if index == 2 && length == 1
        ));
        Ok(())
    }

    #[test]
    fn test_dastore_null_pointer() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(3);
        stack.push_object(None)?;
        stack.push_int(0)?;
        stack.push_double(42f64)?;
        let result = dastore(stack);
        assert!(matches!(result, Err(JavaError(NullPointerException(_)))));
        Ok(())
    }

    #[test]
    fn test_dadd() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_double(1f64)?;
        stack.push_double(2f64)?;
        let result = dadd(stack)?;
        assert_eq!(Continue, result);
        let value = stack.pop_double()? - 3f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[test]
    fn test_dadd_overflow() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_double(f64::MAX)?;
        stack.push_double(1f64)?;
        let result = dadd(stack)?;
        assert_eq!(Continue, result);
        let value = stack.pop_double()? - f64::MAX;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[test]
    fn test_dsub() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_double(2f64)?;
        stack.push_double(1f64)?;
        let result = dsub(stack)?;
        assert_eq!(Continue, result);
        let value = stack.pop_double()? - 1f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[test]
    fn test_dsub_overflow() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_double(f64::MIN)?;
        stack.push_double(1f64)?;
        let result = dsub(stack)?;
        assert_eq!(Continue, result);
        let value = stack.pop_double()? - f64::MIN;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[test]
    fn test_dmul() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_double(2f64)?;
        stack.push_double(3f64)?;
        let result = dmul(stack)?;
        assert_eq!(Continue, result);
        let value = stack.pop_double()? - 6f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[test]
    fn test_ddiv() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_double(6f64)?;
        stack.push_double(3f64)?;
        let result = ddiv(stack)?;
        assert_eq!(Continue, result);
        let value = stack.pop_double()? - 2f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[test]
    fn test_ddiv_divide_by_zero() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_double(1.0)?;
        stack.push_double(0.0)?;
        let result = ddiv(stack);
        assert!(matches!(result, Err(JavaError(ArithmeticException(_)))));
        Ok(())
    }

    #[test]
    fn test_drem() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_double(1f64)?;
        stack.push_double(2f64)?;
        let result = drem(stack)?;
        assert_eq!(Continue, result);
        let value = stack.pop_double()? - 1f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[test]
    fn test_drem_divide_by_zero() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_double(1.0)?;
        stack.push_double(0.0)?;
        let result = drem(stack);
        assert!(matches!(result, Err(JavaError(ArithmeticException(_)))));
        Ok(())
    }

    #[test]
    fn test_dneg() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_double(1f64)?;
        let result = dneg(stack)?;
        assert_eq!(Continue, result);
        let value = stack.pop_double()? + 1f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[test]
    fn test_dcmpl_equal() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_double(1.0)?;
        stack.push_double(1.0)?;
        let result = dcmpl(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(0, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_dcmpl_value1_nan() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_double(1.0)?;
        stack.push_double(f64::NAN)?;
        let result = dcmpl(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(-1, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_dcmpl_value2_nan() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_double(f64::NAN)?;
        stack.push_double(1.0)?;
        let result = dcmpl(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(-1, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_dcmpl_greater_than() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_double(2.0)?;
        stack.push_double(1.0)?;
        let result = dcmpl(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(1, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_dcmpl_less_than() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_double(1.0)?;
        stack.push_double(2.0)?;
        let result = dcmpl(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(-1, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_dcmpg_equal() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_double(1.0)?;
        stack.push_double(1.0)?;
        let result = dcmpg(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(0, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_dcmpg_value1_nan() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_double(1.0)?;
        stack.push_double(f64::NAN)?;
        let result = dcmpg(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(1, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_dcmpg_value2_nan() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_double(f64::NAN)?;
        stack.push_double(1.0)?;
        let result = dcmpg(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(1, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_dcmpg_greater_than() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_double(2.0)?;
        stack.push_double(1.0)?;
        let result = dcmpg(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(1, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_dcmpg_less_than() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_double(1.0)?;
        stack.push_double(2.0)?;
        let result = dcmpg(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(-1, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_dreturn() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_double(42.1)?;
        let result = dreturn(stack)?;
        assert!(matches!(result, Return(Some(Value::Double(42.1)))));
        Ok(())
    }

    #[test]
    fn test_dreturn_invalid_type() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_object(None)?;
        let result = dreturn(stack);
        assert!(matches!(
            result,
            Err(InvalidOperand {
                expected,
                actual
            }) if expected == "double" && actual == "Object(null)"
        ));
        Ok(())
    }
}
