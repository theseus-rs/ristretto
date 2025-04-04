use crate::Error::{InvalidConstant, InvalidConstantIndex};
use crate::Result;
use crate::frame::ExecutionResult::Continue;
use crate::frame::{ExecutionResult, Frame};
use crate::java_object::JavaObject;
use crate::operand_stack::OperandStack;
use ristretto_classfile::Constant;
use ristretto_classloader::Value;

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.ldc>
#[inline]
pub(crate) async fn ldc(
    frame: &Frame,
    stack: &mut OperandStack,
    index: u8,
) -> Result<ExecutionResult> {
    let index = u16::from(index);
    load_constant(frame, stack, index).await
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.ldc_w>
#[inline]
pub(crate) async fn ldc_w(
    frame: &Frame,
    stack: &mut OperandStack,
    index: u16,
) -> Result<ExecutionResult> {
    load_constant(frame, stack, index).await
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.ldc2_w>
#[inline]
pub(crate) fn ldc2_w(
    frame: &Frame,
    stack: &mut OperandStack,
    index: u16,
) -> Result<ExecutionResult> {
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
            });
        }
    };
    stack.push(value)?;
    Ok(Continue)
}

/// Load the constant at the specified index onto the stack
///
/// # Errors
/// if the constant is not an integer, float, string or class
async fn load_constant(
    frame: &Frame,
    stack: &mut OperandStack,
    index: u16,
) -> Result<ExecutionResult> {
    let constant_pool = frame.class().constant_pool();
    let constant = constant_pool
        .get(index)
        .ok_or_else(|| InvalidConstantIndex(index))?;

    let value = match constant {
        Constant::Integer(value) => Value::Int(*value),
        Constant::Float(value) => Value::Float(*value),
        Constant::String(utf8_index) => {
            let utf8_value = constant_pool.try_get_utf8(*utf8_index)?;
            let thread = frame.thread()?;
            let vm = thread.vm()?;
            utf8_value.to_object(&vm).await?
        }
        Constant::Class(class_index) => {
            let class_name = constant_pool.try_get_utf8(*class_index)?;
            let thread = frame.thread()?;
            let vm = thread.vm()?;
            let class = thread.class(class_name).await?;
            class.to_object(&vm).await?
        }
        constant => {
            return Err(InvalidConstant {
                expected: "integer|float|string|class".to_string(),
                actual: format!("{constant:?}"),
            });
        }
    };
    stack.push(value)?;
    Ok(Continue)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_ldc() -> Result<()> {
        let (_vm, _thread, mut frame) = crate::test::frame().await?;
        let stack = &mut OperandStack::with_max_size(1);
        let class = frame.class_mut();
        let constant_pool = Arc::get_mut(class).expect("class").constant_pool_mut();
        let index = constant_pool.add_integer(42)?;
        let index = u8::try_from(index)?;
        let process_result = ldc(&frame, stack, index).await?;
        assert_eq!(process_result, Continue);
        assert_eq!(42, stack.pop_int()?);
        Ok(())
    }

    #[tokio::test]
    async fn test_ldc_w() -> Result<()> {
        let (_vm, _thread, mut frame) = crate::test::frame().await?;
        let stack = &mut OperandStack::with_max_size(1);
        let class = frame.class_mut();
        let constant_pool = Arc::get_mut(class).expect("class").constant_pool_mut();
        let index = constant_pool.add_integer(42)?;
        let process_result = ldc_w(&frame, stack, index).await?;
        assert_eq!(process_result, Continue);
        assert_eq!(42, stack.pop_int()?);
        Ok(())
    }

    #[tokio::test]
    async fn test_constant_integer() -> Result<()> {
        let (_vm, _thread, mut frame) = crate::test::frame().await?;
        let stack = &mut OperandStack::with_max_size(1);
        let class = frame.class_mut();
        let constant_pool = Arc::get_mut(class).expect("class").constant_pool_mut();
        let index = constant_pool.add_integer(42)?;
        let process_result = load_constant(&frame, stack, index).await?;
        assert_eq!(process_result, Continue);
        assert_eq!(42, stack.pop_int()?);
        Ok(())
    }

    #[tokio::test]
    async fn test_load_constant_float() -> Result<()> {
        let (_vm, _thread, mut frame) = crate::test::frame().await?;
        let stack = &mut OperandStack::with_max_size(1);
        let class = frame.class_mut();
        let constant_pool = Arc::get_mut(class).expect("class").constant_pool_mut();
        let index = constant_pool.add_float(42.1)?;
        let process_result = load_constant(&frame, stack, index).await?;
        assert_eq!(process_result, Continue);
        let value = stack.pop_float()? - 42.1f32;
        assert!(value.abs() < 0.1f32);
        Ok(())
    }

    #[tokio::test]
    async fn test_load_constant_string() -> Result<()> {
        let (_vm, _thread, mut frame) = crate::test::frame().await?;
        let stack = &mut OperandStack::with_max_size(1);
        let class = frame.class_mut();
        let constant_pool = Arc::get_mut(class).expect("class").constant_pool_mut();
        let index = constant_pool.add_string("foo")?;
        let process_result = load_constant(&frame, stack, index).await?;
        assert_eq!(process_result, Continue);
        let object = stack.pop_object()?.expect("object");
        assert_eq!("String(\"foo\")", format!("{object}"));
        Ok(())
    }

    #[tokio::test]
    async fn test_load_constant_class() -> Result<()> {
        let (_vm, _thread, mut frame) = crate::test::frame().await?;
        let stack = &mut OperandStack::with_max_size(1);
        let class = frame.class_mut();
        let constant_pool = Arc::get_mut(class).expect("class").constant_pool_mut();
        let index = constant_pool.add_class("java/lang/Object")?;
        let process_result = load_constant(&frame, stack, index).await?;
        assert_eq!(process_result, Continue);
        let object = stack.pop_object()?.expect("object");
        assert_eq!("Class(java.lang.Object)", format!("{object}"));
        Ok(())
    }

    #[tokio::test]
    async fn test_load_constant_invalid_index() -> Result<()> {
        let (_vm, _thread, frame) = crate::test::frame().await?;
        let stack = &mut OperandStack::with_max_size(1);
        let result = load_constant(&frame, stack, 42).await;
        assert!(matches!(result, Err(InvalidConstantIndex(42))));
        Ok(())
    }

    #[tokio::test]
    async fn test_load_constant_invalid_type() -> Result<()> {
        let (_vm, _thread, mut frame) = crate::test::frame().await?;
        let stack = &mut OperandStack::with_max_size(1);
        let class = frame.class_mut();
        let constant_pool = Arc::get_mut(class).expect("class").constant_pool_mut();
        let index = constant_pool.add_long(42)?;
        let result = load_constant(&frame, stack, index).await;
        assert!(matches!(
            result,
            Err(InvalidConstant {
                expected,
                actual
            }) if expected == "integer|float|string|class" && actual == "Long(42)"
        ));
        Ok(())
    }

    #[tokio::test]
    async fn test_ldc2_w_long() -> Result<()> {
        let (_vm, _thread, mut frame) = crate::test::frame().await?;
        let stack = &mut OperandStack::with_max_size(1);
        let class = frame.class_mut();
        let constant_pool = Arc::get_mut(class).expect("class").constant_pool_mut();
        let index = constant_pool.add_long(42)?;
        let result = ldc2_w(&frame, stack, index)?;
        assert_eq!(Continue, result);
        assert_eq!(42, stack.pop_long()?);
        Ok(())
    }

    #[tokio::test]
    async fn test_ldc2_w_double() -> Result<()> {
        let (_vm, _thread, mut frame) = crate::test::frame().await?;
        let stack = &mut OperandStack::with_max_size(1);
        let class = frame.class_mut();
        let constant_pool = Arc::get_mut(class).expect("class").constant_pool_mut();
        let index = constant_pool.add_double(42.1)?;
        let result = ldc2_w(&frame, stack, index)?;
        assert_eq!(Continue, result);
        let value = stack.pop_double()? - 42.1f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[tokio::test]
    async fn test_ldc2_w_invalid_index() -> Result<()> {
        let (_vm, _thread, frame) = crate::test::frame().await?;
        let stack = &mut OperandStack::with_max_size(1);
        let result = ldc2_w(&frame, stack, 42);
        assert!(matches!(result, Err(InvalidConstantIndex(42))));
        Ok(())
    }

    #[tokio::test]
    async fn test_ldc2_w_invalid_type() -> Result<()> {
        let (_vm, _thread, mut frame) = crate::test::frame().await?;
        let stack = &mut OperandStack::with_max_size(1);
        let class = frame.class_mut();
        let constant_pool = Arc::get_mut(class).expect("class").constant_pool_mut();
        let index = constant_pool.add_integer(42)?;
        let result = ldc2_w(&frame, stack, index);
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
