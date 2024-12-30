use crate::frame::ExecutionResult;
use crate::frame::ExecutionResult::Continue;
use crate::operand_stack::OperandStack;
use crate::Error::InvalidStackValue;
use crate::Result;
use ristretto_classloader::{Class, Reference, Value};
use std::sync::Arc;

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.getfield>
#[inline]
pub(crate) fn getfield(
    stack: &mut OperandStack,
    class: &Arc<Class>,
    index: u16,
) -> Result<ExecutionResult> {
    let value = stack.pop()?;
    match value {
        Value::Object(Some(Reference::Object(object))) => {
            let constant_pool = class.constant_pool();
            let (_class_index, name_and_type_index) = constant_pool.try_get_field_ref(index)?;
            let (name_index, _descriptor_index) =
                constant_pool.try_get_name_and_type(*name_and_type_index)?;
            let field_name = constant_pool.try_get_utf8(*name_index)?;
            let value = object.value(field_name)?;
            stack.push(value)?;
            Ok(Continue)
        }
        _ => Err(InvalidStackValue {
            expected: "object".to_string(),
            actual: value.to_string(),
        }),
    }
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.putfield>
#[inline]
pub(crate) fn putfield(
    stack: &mut OperandStack,
    class: &Arc<Class>,
    index: u16,
) -> Result<ExecutionResult> {
    let value = stack.pop()?;
    let mut object_value = stack.pop()?;
    match object_value {
        Value::Object(Some(Reference::Object(ref mut object))) => {
            let constant_pool = class.constant_pool();
            let (_class_index, name_and_type_index) = constant_pool.try_get_field_ref(index)?;
            let (name_index, _descriptor_index) =
                constant_pool.try_get_name_and_type(*name_and_type_index)?;
            let field_name = constant_pool.try_get_utf8(*name_index)?;
            object.set_value(field_name, value)?;
            Ok(Continue)
        }
        _ => Err(InvalidStackValue {
            expected: "object".to_string(),
            actual: object_value.to_string(),
        }),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::frame::Frame;
    use crate::instruction::{dup, new};
    use crate::thread::Thread;
    use crate::VM;
    use ristretto_classfile::MethodAccessFlags;
    use ristretto_classloader::{Method, Value};
    use std::sync::Arc;

    async fn test_class_field(
        class_name: &str,
        field_name: &str,
        field_type: &str,
    ) -> Result<(Arc<VM>, Arc<Thread>, Frame, u16, u16)> {
        let (vm, thread, mut class) = crate::test::class().await?;
        let constant_pool = Arc::get_mut(&mut class).expect("class").constant_pool_mut();
        let class_index = constant_pool.add_class(class_name)?;
        let field_index = constant_pool.add_field_ref(class_index, field_name, field_type)?;
        let method = Method::new(
            MethodAccessFlags::STATIC,
            "test",
            "()V",
            10,
            10,
            Vec::new(),
            Vec::new(),
            Vec::new(),
        )?;
        let arguments = Vec::new();
        let frame = Frame::new(
            &Arc::downgrade(&thread),
            &class,
            &Arc::new(method),
            arguments,
        );
        Ok((vm, thread, frame, class_index, field_index))
    }

    async fn test_put_and_get_field() -> Result<()> {
        let (_vm, _thread, frame, class_index, field_index) =
            test_class_field("Child", "zero", "I").await?;
        let stack = &mut OperandStack::with_max_size(4);
        let class = frame.class();
        let result = new(&frame, stack, class_index).await?;
        assert_eq!(Continue, result);

        let result = dup(stack)?;
        assert_eq!(Continue, result);

        let result = dup(stack)?;
        assert_eq!(Continue, result);

        stack.push_int(42)?;
        let result = putfield(stack, class, field_index)?;
        assert_eq!(Continue, result);

        let result = getfield(stack, class, field_index)?;
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
            test_class_field("Child", "foo", "I").await?;
        let stack = &mut OperandStack::with_max_size(1);
        let result = new(&frame, stack, class_index).await?;
        assert_eq!(Continue, result);
        let class = frame.class();
        let result = getfield(stack, class, field_index);
        assert!(result.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_getfield_invalid_value() -> Result<()> {
        let (_vm, _thread, frame) = crate::test::frame().await?;
        let stack = &mut OperandStack::with_max_size(2);
        let class = frame.class();
        stack.push_object(None)?;
        let result = getfield(stack, class, 0);
        assert!(matches!(result, Err(InvalidStackValue {
            expected,
            actual
        }) if expected == "object" && actual == "Object(null)"));

        Ok(())
    }

    #[tokio::test]
    async fn test_putfield() -> Result<()> {
        test_put_and_get_field().await
    }

    #[tokio::test]
    async fn test_putfield_field_not_found() -> Result<()> {
        let (_vm, _thread, frame, class_index, field_index) =
            test_class_field("Child", "foo", "I").await?;
        let stack = &mut OperandStack::with_max_size(3);
        let class = frame.class();
        let result = new(&frame, stack, class_index).await?;
        assert_eq!(Continue, result);
        let result = dup(stack)?;
        assert_eq!(Continue, result);
        stack.push_int(42)?;
        let result = putfield(stack, class, field_index);
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
        let result = putfield(stack, class, 0);
        assert!(matches!(result, Err(InvalidStackValue {
            expected,
            actual
        }) if expected == "object" && actual == "Object(null)"));

        Ok(())
    }
}
