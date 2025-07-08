use crate::Result;
use crate::frame::ExecutionResult::Continue;
use crate::frame::{ExecutionResult, Frame};
use crate::operand_stack::OperandStack;
use ristretto_classloader::{Class, Object};
use std::sync::Arc;

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.getfield>
#[inline]
pub(crate) async fn getfield(
    frame: &Frame,
    stack: &mut OperandStack,
    class: &Arc<Class>,
    index: u16,
) -> Result<ExecutionResult> {
    let thread = frame.thread()?;
    let object: Object = stack.pop()?.try_into()?;
    let constant_pool = class.constant_pool();
    let (class_index, name_and_type_index) = constant_pool.try_get_field_ref(index)?;
    let field_class_name = constant_pool.try_get_class(*class_index)?;
    let field_class = thread.class(field_class_name).await?;
    let (name_index, _descriptor_index) =
        constant_pool.try_get_name_and_type(*name_and_type_index)?;
    let field_name = constant_pool.try_get_utf8(*name_index)?;
    let value = object.value_in_class(&field_class, field_name)?;
    stack.push(value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.putfield>
#[inline]
pub(crate) async fn putfield(
    frame: &Frame,
    stack: &mut OperandStack,
    class: &Arc<Class>,
    index: u16,
) -> Result<ExecutionResult> {
    let thread = frame.thread()?;
    let value = stack.pop()?;
    let object: Object = stack.pop()?.try_into()?;
    let constant_pool = class.constant_pool();
    let (class_index, name_and_type_index) = constant_pool.try_get_field_ref(index)?;
    let field_class_name = constant_pool.try_get_class(*class_index)?;
    let field_class = thread.class(field_class_name).await?;
    let (name_index, _descriptor_index) =
        constant_pool.try_get_name_and_type(*name_and_type_index)?;
    let field_name = constant_pool.try_get_utf8(*name_index)?;
    object.set_value_in_class(&field_class, field_name, value)?;
    Ok(Continue)
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
        let class = frame.class();
        let result = new(&frame, stack, class_index).await?;
        assert_eq!(Continue, result);

        let result = dup(stack)?;
        assert_eq!(Continue, result);

        let result = dup(stack)?;
        assert_eq!(Continue, result);

        stack.push_int(42)?;
        let result = putfield(&frame, stack, class, field_index).await?;
        assert_eq!(Continue, result);

        let result = getfield(&frame, stack, class, field_index).await?;
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
        let class = frame.class();
        let result = getfield(&frame, stack, class, field_index).await;
        assert!(result.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_getfield_invalid_value() -> Result<()> {
        let (_vm, _thread, frame) = crate::test::frame().await?;
        let stack = &mut OperandStack::with_max_size(2);
        let class = frame.class();
        stack.push_object(None)?;
        let result = getfield(&frame, stack, class, 0).await;
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
        let class = frame.class();
        let result = new(&frame, stack, class_index).await?;
        assert_eq!(Continue, result);
        let result = dup(stack)?;
        assert_eq!(Continue, result);
        stack.push_int(42)?;
        let result = putfield(&frame, stack, class, field_index).await;
        assert!(result.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_putfield_invalid_value() -> Result<()> {
        let (_vm, _thread, frame) = crate::test::frame().await?;
        let stack = &mut OperandStack::with_max_size(2);
        let class = frame.class();
        stack.push_object(None)?;
        stack.push_int(42)?;
        let result = putfield(&frame, stack, class, 0).await;
        assert!(result.is_err());
        Ok(())
    }
}
