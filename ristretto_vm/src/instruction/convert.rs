use crate::frame::ExecutionResult;
use crate::frame::ExecutionResult::Continue;
use crate::operand_stack::OperandStack;

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.i2l>
#[inline]
pub(crate) fn i2l(stack: &mut OperandStack) -> crate::Result<ExecutionResult> {
    let value = stack.pop_int()?;
    stack.push_long(i64::from(value))?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.i2f>
#[inline]
pub(crate) fn i2f(stack: &mut OperandStack) -> crate::Result<ExecutionResult> {
    let value = stack.pop_int()?;
    #[expect(clippy::cast_precision_loss)]
    stack.push_float(value as f32)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.i2d>
#[inline]
pub(crate) fn i2d(stack: &mut OperandStack) -> crate::Result<ExecutionResult> {
    let value = stack.pop_int()?;
    stack.push_double(f64::from(value))?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.l2i>
#[inline]
pub(crate) fn l2i(stack: &mut OperandStack) -> crate::Result<ExecutionResult> {
    let value = stack.pop_long()?;
    #[expect(clippy::cast_possible_truncation)]
    stack.push_int(value as i32)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.l2f>
#[inline]
pub(crate) fn l2f(stack: &mut OperandStack) -> crate::Result<ExecutionResult> {
    let value = stack.pop_long()?;
    #[expect(clippy::cast_precision_loss)]
    stack.push_float(value as f32)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.l2d>
#[inline]
pub(crate) fn l2d(stack: &mut OperandStack) -> crate::Result<ExecutionResult> {
    let value = stack.pop_long()?;
    #[expect(clippy::cast_precision_loss)]
    stack.push_double(value as f64)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.f2i>
#[inline]
pub(crate) fn f2i(stack: &mut OperandStack) -> crate::Result<ExecutionResult> {
    let value = stack.pop_float()?;
    #[expect(clippy::cast_possible_truncation)]
    stack.push_int(value as i32)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.f2l>
#[inline]
pub(crate) fn f2l(stack: &mut OperandStack) -> crate::Result<ExecutionResult> {
    let value = stack.pop_float()?;
    #[expect(clippy::cast_possible_truncation)]
    stack.push_long(value as i64)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.f2d>
#[inline]
pub(crate) fn f2d(stack: &mut OperandStack) -> crate::Result<ExecutionResult> {
    let value = stack.pop_float()?;
    stack.push_double(f64::from(value))?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.d2i>
#[inline]
pub(crate) fn d2i(stack: &mut OperandStack) -> crate::Result<ExecutionResult> {
    let value = stack.pop_double()?;
    #[expect(clippy::cast_possible_truncation)]
    stack.push_int(value as i32)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.d2l>
#[inline]
pub(crate) fn d2l(stack: &mut OperandStack) -> crate::Result<ExecutionResult> {
    let value = stack.pop_double()?;
    #[expect(clippy::cast_possible_truncation)]
    stack.push_long(value as i64)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.d2f>
#[inline]
pub(crate) fn d2f(stack: &mut OperandStack) -> crate::Result<ExecutionResult> {
    let value = stack.pop_double()?;
    #[expect(clippy::cast_possible_truncation)]
    stack.push_float(value as f32)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.i2b>
#[inline]
pub(crate) fn i2b(stack: &mut OperandStack) -> crate::Result<ExecutionResult> {
    let value = stack.pop_int()?;
    #[expect(clippy::cast_possible_truncation)]
    let byte = value as i8;
    stack.push_int(i32::from(byte))?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.i2c>
#[inline]
pub(crate) fn i2c(stack: &mut OperandStack) -> crate::Result<ExecutionResult> {
    let value = stack.pop_int()?;
    #[expect(clippy::cast_possible_truncation)]
    #[expect(clippy::cast_sign_loss)]
    let char = value as u16;
    stack.push_int(i32::from(char))?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.i2s>
#[inline]
pub(crate) fn i2s(stack: &mut OperandStack) -> crate::Result<ExecutionResult> {
    let value = stack.pop_int()?;
    #[expect(clippy::cast_possible_truncation)]
    let short = value as i16;
    stack.push_int(i32::from(short))?;
    Ok(Continue)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_i2l() -> crate::Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(42)?;
        let result = i2l(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(42, stack.pop_long()?);
        Ok(())
    }

    #[test]
    fn test_i2f() -> crate::Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(42)?;
        let result = i2f(stack)?;
        assert_eq!(Continue, result);
        let value = stack.pop_float()? - 42f32;
        assert!(value.abs() < 0.1f32);
        Ok(())
    }

    #[test]
    fn test_i2d() -> crate::Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(42)?;
        let result = i2d(stack)?;
        assert_eq!(Continue, result);
        let value = stack.pop_double()? - 42f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[test]
    fn test_l2i() -> crate::Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_long(42)?;
        let result = l2i(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(42, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_l2f() -> crate::Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_long(42)?;
        let result = l2f(stack)?;
        assert_eq!(Continue, result);
        let value = stack.pop_float()? - 42f32;
        assert!(value.abs() < 0.1f32);
        Ok(())
    }

    #[test]
    fn test_l2d() -> crate::Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_long(42)?;
        let result = l2d(stack)?;
        assert_eq!(Continue, result);
        let value = stack.pop_double()? - 42f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[test]
    fn test_f2i() -> crate::Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_float(42.1)?;
        let result = f2i(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(42, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_f2l() -> crate::Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_float(42.1)?;
        let result = f2l(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(42, stack.pop_long()?);
        Ok(())
    }

    #[test]
    fn test_f2d() -> crate::Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_float(42.1)?;
        let result = f2d(stack)?;
        assert_eq!(Continue, result);
        let value = stack.pop_double()? - 42.1f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[test]
    fn test_d2i() -> crate::Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_double(42.1)?;
        let result = d2i(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(42, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_d2l() -> crate::Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_double(42.1)?;
        let result = d2l(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(42, stack.pop_long()?);
        Ok(())
    }

    #[test]
    fn test_d2f() -> crate::Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_double(42.1)?;
        let result = d2f(stack)?;
        assert_eq!(Continue, result);
        let value = stack.pop_float()? - 42.1f32;
        assert!(value.abs() < 0.1f32);
        Ok(())
    }

    #[test]
    fn test_i2b() -> crate::Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(42)?;
        let result = i2b(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(42, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_i2c() -> crate::Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(42)?;
        let result = i2c(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(42, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_i2s() -> crate::Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(42)?;
        let result = i2s(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(42, stack.pop_int()?);
        Ok(())
    }
}
