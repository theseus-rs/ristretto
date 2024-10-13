use crate::frame::ExecutionResult::Continue;
use crate::frame::{ExecutionResult, Frame};
use crate::Error::{InvalidConstant, InvalidConstantIndex};
use crate::Result;
use ristretto_classfile::Constant;
use ristretto_classloader::Value;

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.ldc>
#[inline]
pub(crate) fn ldc(frame: &mut Frame, index: u8) -> Result<ExecutionResult> {
    let index = u16::from(index);
    load_constant(frame, index)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.ldc_w>
#[inline]
pub(crate) fn ldc_w(frame: &mut Frame, index: u16) -> Result<ExecutionResult> {
    load_constant(frame, index)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.ldc2_w>
#[inline]
pub(crate) fn ldc2_w(frame: &mut Frame, index: u16) -> Result<ExecutionResult> {
    let constant_pool = frame.class().constant_pool();
    let constant = constant_pool
        .get(index)
        .ok_or_else(|| InvalidConstantIndex(index))?;

    let value = match constant {
        Constant::Long(value) => Value::Long(*value),
        Constant::Double(value) => Value::Double(*value),
        constant => {
            return Err(InvalidConstant {
                expected: "long|double".to_string(),
                actual: format!("{constant:?}"),
            })
        }
    };
    frame.stack_mut().push(value)?;
    Ok(Continue)
}

/// Load the constant at the specified index onto the stack
///
/// # Errors
/// if the constant is not an integer, float, string or class
fn load_constant(frame: &mut Frame, index: u16) -> Result<ExecutionResult> {
    let constant_pool = frame.class().constant_pool();
    let constant = constant_pool
        .get(index)
        .ok_or_else(|| InvalidConstantIndex(index))?;

    let value = match constant {
        Constant::Integer(value) => Value::Int(*value),
        Constant::Float(value) => Value::Float(*value),
        Constant::String(utf8_index) => {
            let utf8_value = constant_pool.try_get_utf8(*utf8_index)?;
            let call_stack = frame.call_stack()?;
            let vm = call_stack.vm()?;
            vm.to_string_value(&call_stack, utf8_value)?
        }
        Constant::Class(class_index) => {
            let class_name = constant_pool.try_get_utf8(*class_index)?;
            let call_stack = frame.call_stack()?;
            let vm = call_stack.vm()?;
            vm.to_class_value(&call_stack, class_name)?
        }
        constant => {
            return Err(InvalidConstant {
                expected: "integer|float|string|class".to_string(),
                actual: format!("{constant:?}"),
            })
        }
    };
    frame.stack_mut().push(value)?;
    Ok(Continue)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::sync::Arc;

    #[test]
    fn test_ldc() -> Result<()> {
        let (_vm, _call_stack, mut frame) = crate::test::frame()?;
        let class = frame.class_mut();
        let constant_pool = Arc::get_mut(class).expect("class").constant_pool_mut();
        let index = constant_pool.add_integer(42)?;
        let index = u8::try_from(index)?;
        let process_result = ldc(&mut frame, index)?;
        assert_eq!(process_result, Continue);
        assert_eq!(42, frame.stack_mut().pop_int()?);
        Ok(())
    }

    #[test]
    fn test_ldc_w() -> Result<()> {
        let (_vm, _call_stack, mut frame) = crate::test::frame()?;
        let class = frame.class_mut();
        let constant_pool = Arc::get_mut(class).expect("class").constant_pool_mut();
        let index = constant_pool.add_integer(42)?;
        let process_result = ldc_w(&mut frame, index)?;
        assert_eq!(process_result, Continue);
        assert_eq!(42, frame.stack_mut().pop_int()?);
        Ok(())
    }

    #[test]
    fn test_constant_integer() -> Result<()> {
        let (_vm, _call_stack, mut frame) = crate::test::frame()?;
        let class = frame.class_mut();
        let constant_pool = Arc::get_mut(class).expect("class").constant_pool_mut();
        let index = constant_pool.add_integer(42)?;
        let process_result = load_constant(&mut frame, index)?;
        assert_eq!(process_result, Continue);
        assert_eq!(42, frame.stack_mut().pop_int()?);
        Ok(())
    }

    #[test]
    fn test_load_constant_float() -> Result<()> {
        let (_vm, _call_stack, mut frame) = crate::test::frame()?;
        let class = frame.class_mut();
        let constant_pool = Arc::get_mut(class).expect("class").constant_pool_mut();
        let index = constant_pool.add_float(42.1)?;
        let process_result = load_constant(&mut frame, index)?;
        assert_eq!(process_result, Continue);
        let value = frame.stack_mut().pop_float()? - 42.1f32;
        assert!(value.abs() < 0.1f32);
        Ok(())
    }

    #[test]
    fn test_load_constant_string() -> Result<()> {
        let (_vm, _call_stack, mut frame) = crate::test::frame()?;
        let class = frame.class_mut();
        let constant_pool = Arc::get_mut(class).expect("class").constant_pool_mut();
        let index = constant_pool.add_string("foo")?;
        let process_result = load_constant(&mut frame, index)?;
        assert_eq!(process_result, Continue);
        let object = frame.stack_mut().pop_object()?.expect("object");
        assert_eq!("string(foo)", format!("{object}"));
        Ok(())
    }

    #[test]
    fn test_load_constant_class() -> Result<()> {
        let (_vm, _call_stack, mut frame) = crate::test::frame()?;
        let class = frame.class_mut();
        let constant_pool = Arc::get_mut(class).expect("class").constant_pool_mut();
        let index = constant_pool.add_class("java/lang/Object")?;
        let process_result = load_constant(&mut frame, index)?;
        assert_eq!(process_result, Continue);
        let object = frame.stack_mut().pop_object()?.expect("object");
        assert_eq!("class java/lang/Class", format!("{object}"));
        Ok(())
    }

    #[test]
    fn test_load_constant_invalid_index() -> Result<()> {
        let (_vm, _call_stack, mut frame) = crate::test::frame()?;
        let result = load_constant(&mut frame, 42);
        assert!(matches!(result, Err(InvalidConstantIndex(42))));
        Ok(())
    }

    #[test]
    fn test_load_constant_invalid_type() -> Result<()> {
        let (_vm, _call_stack, mut frame) = crate::test::frame()?;
        let class = frame.class_mut();
        let constant_pool = Arc::get_mut(class).expect("class").constant_pool_mut();
        let index = constant_pool.add_long(42)?;
        let result = load_constant(&mut frame, index);
        assert!(matches!(
            result,
            Err(InvalidConstant {
                expected,
                actual
            }) if expected == "integer|float|string|class" && actual == "Long(42)"
        ));
        Ok(())
    }

    #[test]
    fn test_ldc2_w_long() -> Result<()> {
        let (_vm, _call_stack, mut frame) = crate::test::frame()?;
        let class = frame.class_mut();
        let constant_pool = Arc::get_mut(class).expect("class").constant_pool_mut();
        let index = constant_pool.add_long(42)?;
        let result = ldc2_w(&mut frame, index)?;
        assert_eq!(Continue, result);
        assert_eq!(42, frame.stack_mut().pop_long()?);
        Ok(())
    }

    #[test]
    fn test_ldc2_w_double() -> Result<()> {
        let (_vm, _call_stack, mut frame) = crate::test::frame()?;
        let class = frame.class_mut();
        let constant_pool = Arc::get_mut(class).expect("class").constant_pool_mut();
        let index = constant_pool.add_double(42.1)?;
        let result = ldc2_w(&mut frame, index)?;
        assert_eq!(Continue, result);
        let value = frame.stack_mut().pop_double()? - 42.1f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[test]
    fn test_ldc2_w_invalid_index() -> Result<()> {
        let (_vm, _call_stack, mut frame) = crate::test::frame()?;
        let result = ldc2_w(&mut frame, 42);
        assert!(matches!(result, Err(InvalidConstantIndex(42))));
        Ok(())
    }

    #[test]
    fn test_ldc2_w_invalid_type() -> Result<()> {
        let (_vm, _call_stack, mut frame) = crate::test::frame()?;
        let class = frame.class_mut();
        let constant_pool = Arc::get_mut(class).expect("class").constant_pool_mut();
        let index = constant_pool.add_integer(42)?;
        let result = ldc2_w(&mut frame, index);
        assert!(matches!(
            result,
            Err(InvalidConstant {
                expected,
                actual
            }) if expected == "long|double" && actual == "Integer(42)"
        ));

        Ok(())
    }
}
