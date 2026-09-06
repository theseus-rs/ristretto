#[cfg(test)]
use crate::frame::ExecutionResult::Continue;
use crate::frame::{ExecutionResult, Frame};
use crate::operand_stack::OperandStack;
use crate::{Result, instruction};
use ristretto_classfile::attributes::Instruction;

/// The `getfield` instruction fetches a field value from an object instance.
///
/// # References
///
/// - [JVMS §6.5.getfield](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.getfield)
#[inline]
pub(crate) async fn getfield(
    frame: &Frame,
    stack: &mut OperandStack,
    index: u16,
) -> Result<ExecutionResult> {
    instruction::execute_field(frame, stack, index, Instruction::Getfield(index)).await
}

/// The `putfield` instruction sets a field value in an object instance.
///
/// # References
///
/// - [JVMS §6.5.putfield](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.putfield)
#[inline]
pub(crate) async fn putfield(
    frame: &Frame,
    stack: &mut OperandStack,
    index: u16,
) -> Result<ExecutionResult> {
    instruction::execute_field(frame, stack, index, Instruction::Putfield(index)).await
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::VM;
    use crate::frame::Frame;
    use crate::instruction::{dup, new};
    use crate::thread::Thread;
    use ristretto_classloader::Value;
    use std::sync::Arc;

    async fn test_class_field(
        class_name: &str,
        field_name: &str,
        field_type: &str,
    ) -> Result<(Arc<VM>, Arc<Thread>, Frame, u16, u16)> {
        let (vm, thread, mut frame) = crate::test::frame().await?;
        let class = frame.class_mut();
        let constant_pool = Arc::get_mut(class).expect("class").constant_pool_mut();
        let class_index = constant_pool.add_class(class_name)?;
        let field_index = constant_pool.add_field_ref(class_index, field_name, field_type)?;
        Ok((vm, thread, frame, class_index, field_index))
    }

    async fn test_put_and_get_field() -> Result<()> {
        let (_vm, _thread, frame, class_index, field_index) =
            test_class_field("java.lang.Integer", "value", "I").await?;
        let stack = &mut OperandStack::with_max_size(4);
        let result = new(&frame, stack, class_index).await?;
        assert_eq!(Continue, result);

        let result = dup(stack)?;
        assert_eq!(Continue, result);

        let result = dup(stack)?;
        assert_eq!(Continue, result);

        stack.push_int(42)?;
        let result = putfield(&frame, stack, field_index).await?;
        assert_eq!(Continue, result);

        let result = getfield(&frame, stack, field_index).await?;
        assert_eq!(Continue, result);
        let value = stack.pop()?;
        assert_eq!(Value::Int(42), value);
        Ok(())
    }

    #[tokio::test]
    async fn test_getfield() -> Result<()> {
        test_put_and_get_field().await
    }

    #[tokio::test]
    async fn test_getfield_field_not_found() -> Result<()> {
        let (_vm, _thread, frame, class_index, field_index) =
            test_class_field("java.lang.Integer", "foo", "I").await?;
        let stack = &mut OperandStack::with_max_size(1);
        let result = new(&frame, stack, class_index).await?;
        assert_eq!(Continue, result);
        let result = getfield(&frame, stack, field_index).await;
        assert!(result.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_getfield_invalid_value() -> Result<()> {
        let (_vm, _thread, frame) = crate::test::frame().await?;
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_object(None)?;
        let result = getfield(&frame, stack, 0).await;
        assert!(result.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_putfield() -> Result<()> {
        test_put_and_get_field().await
    }

    #[tokio::test]
    async fn test_putfield_field_not_found() -> Result<()> {
        let (_vm, _thread, frame, class_index, field_index) =
            test_class_field("java.lang.Integer", "foo", "I").await?;
        let stack = &mut OperandStack::with_max_size(3);
        let result = new(&frame, stack, class_index).await?;
        assert_eq!(Continue, result);
        let result = dup(stack)?;
        assert_eq!(Continue, result);
        stack.push_int(42)?;
        let result = putfield(&frame, stack, field_index).await;
        assert!(result.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_putfield_invalid_value() -> Result<()> {
        let (_vm, _thread, frame) = crate::test::frame().await?;
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_object(None)?;
        stack.push_int(42)?;
        let result = putfield(&frame, stack, 0).await;
        assert!(result.is_err());
        Ok(())
    }
}
