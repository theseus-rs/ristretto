use crate::frame::ExecutionResult;
use crate::frame::ExecutionResult::Continue;
use crate::operand_stack::OperandStack;
use crate::Error::InvalidStackValue;
use crate::Result;
use ristretto_classfile::ConstantPool;
use ristretto_classloader::Reference;

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.getfield>
#[inline]
pub(crate) fn getfield(
    stack: &mut OperandStack,
    constant_pool: &ConstantPool,
    index: u16,
) -> Result<ExecutionResult> {
    match stack.pop_object()? {
        Some(Reference::Object(object)) => {
            let (_class_index, name_and_type_index) = constant_pool.try_get_field_ref(index)?;
            let (name_index, _descriptor_index) =
                constant_pool.try_get_name_and_type(*name_and_type_index)?;
            let field_name = constant_pool.try_get_utf8(*name_index)?;
            let field = object.field(field_name)?;
            let value = field.value()?;
            stack.push(value)?;
            Ok(Continue)
        }
        Some(object) => Err(InvalidStackValue {
            expected: "object".to_string(),
            actual: object.to_string(),
        }),
        None => Err(InvalidStackValue {
            expected: "object".to_string(),
            actual: "null".to_string(),
        }),
    }
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.putfield>
#[inline]
pub(crate) fn putfield(
    stack: &mut OperandStack,
    constant_pool: &ConstantPool,
    index: u16,
) -> Result<ExecutionResult> {
    let value = stack.pop()?;
    let mut object = stack.pop_object()?;
    match object {
        Some(Reference::Object(ref mut object)) => {
            let (_class_index, name_and_type_index) = constant_pool.try_get_field_ref(index)?;
            let (name_index, _descriptor_index) =
                constant_pool.try_get_name_and_type(*name_and_type_index)?;
            let field_name = constant_pool.try_get_utf8(*name_index)?;
            let field = object.field(field_name)?;
            field.set_value(value)?;
            Ok(Continue)
        }
        Some(object) => Err(InvalidStackValue {
            expected: "object".to_string(),
            actual: object.to_string(),
        }),
        None => Err(InvalidStackValue {
            expected: "object".to_string(),
            actual: "null".to_string(),
        }),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::call_stack::CallStack;
    use crate::frame::Frame;
    use crate::instruction::{dup, new};
    use crate::VM;
    use ristretto_classfile::MethodAccessFlags;
    use ristretto_classloader::{Method, Value};
    use std::sync::Arc;

    fn test_class_field(
        class_name: &str,
        field_name: &str,
        field_type: &str,
    ) -> Result<(VM, CallStack, Frame, u16, u16)> {
        let (vm, _call_stack, mut class) = crate::test::class()?;
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
        )?;
        let arguments = Vec::new();
        let call_stack = CallStack::new();
        let frame = Frame::new(&class, &Arc::new(method), arguments)?;
        Ok((vm, call_stack, frame, class_index, field_index))
    }

    fn test_put_and_get_field() -> Result<()> {
        let (vm, mut call_stack, mut frame, class_index, field_index) =
            test_class_field("Child", "zero", "I")?;
        let stack = &mut frame.stack;
        let constant_pool = frame.class.constant_pool();
        let result = new(&vm, &mut call_stack, stack, constant_pool, class_index)?;
        assert_eq!(Continue, result);

        let result = dup(stack)?;
        assert_eq!(Continue, result);

        let result = dup(stack)?;
        assert_eq!(Continue, result);

        stack.push_int(42)?;
        let result = putfield(stack, constant_pool, field_index)?;
        assert_eq!(Continue, result);

        let result = getfield(stack, constant_pool, field_index)?;
        assert_eq!(Continue, result);
        let value = frame.stack.pop()?;
        assert_eq!(Value::Int(42), value);
        Ok(())
    }

    #[test]
    fn test_getfield() -> Result<()> {
        test_put_and_get_field()
    }

    #[test]
    fn test_getfield_field_not_found() -> Result<()> {
        let (vm, mut call_stack, mut frame, class_index, field_index) =
            test_class_field("Child", "foo", "I")?;
        let stack = &mut frame.stack;
        let constant_pool = frame.class.constant_pool();
        let result = new(&vm, &mut call_stack, stack, constant_pool, class_index)?;
        assert_eq!(Continue, result);
        let result = getfield(stack, constant_pool, field_index);
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_getfield_invalid_value() -> Result<()> {
        let (_vm, _call_stack, frame) = crate::test::frame()?;
        let stack = &mut OperandStack::with_max_size(2);
        let constant_pool = frame.class.constant_pool();
        stack.push_object(None)?;
        let result = getfield(stack, constant_pool, 0);
        assert!(matches!(result, Err(InvalidStackValue {
            expected,
            actual
        }) if expected == "object" && actual == "null"));

        Ok(())
    }

    #[test]
    fn test_putfield() -> Result<()> {
        test_put_and_get_field()
    }

    #[test]
    fn test_putfield_field_not_found() -> Result<()> {
        let (vm, mut call_stack, mut frame, class_index, field_index) =
            test_class_field("Child", "foo", "I")?;
        let stack = &mut OperandStack::with_max_size(2);
        let constant_pool = frame.class.constant_pool();
        let result = new(&vm, &mut call_stack, stack, constant_pool, class_index)?;
        assert_eq!(Continue, result);
        let result = dup(stack)?;
        assert_eq!(Continue, result);
        frame.stack.push_int(42)?;
        let result = putfield(stack, constant_pool, field_index);
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_putfield_invalid_value() -> Result<()> {
        let (_vm, _call_stack, frame) = crate::test::frame()?;
        let stack = &mut OperandStack::with_max_size(2);
        let constant_pool = frame.class.constant_pool();
        stack.push_object(None)?;
        stack.push_int(42)?;
        let result = putfield(stack, constant_pool, 0);
        assert!(matches!(result, Err(InvalidStackValue {
            expected,
            actual
        }) if expected == "object" && actual == "null"));

        Ok(())
    }
}
