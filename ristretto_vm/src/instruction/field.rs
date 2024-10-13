use crate::frame::ExecutionResult;
use crate::frame::ExecutionResult::Continue;
use crate::operand_stack::OperandStack;
use crate::Error::InvalidStackValue;
use crate::Result;
use ristretto_classfile::ConstantPool;
use ristretto_classloader::{Reference, Value};

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.getfield>
#[inline]
pub(crate) fn getfield(
    stack: &OperandStack,
    constant_pool: &ConstantPool,
    index: u16,
) -> Result<ExecutionResult> {
    let value = stack.pop()?;
    match value {
        Value::Object(Some(Reference::Object(object))) => {
            let (_class_index, name_and_type_index) = constant_pool.try_get_field_ref(index)?;
            let (name_index, _descriptor_index) =
                constant_pool.try_get_name_and_type(*name_and_type_index)?;
            let field_name = constant_pool.try_get_utf8(*name_index)?;
            let field = object.field(field_name)?;
            let value = field.value()?;
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
    stack: &OperandStack,
    constant_pool: &ConstantPool,
    index: u16,
) -> Result<ExecutionResult> {
    let value = stack.pop()?;
    let mut object_value = stack.pop()?;
    match object_value {
        Value::Object(Some(Reference::Object(ref mut object))) => {
            let (_class_index, name_and_type_index) = constant_pool.try_get_field_ref(index)?;
            let (name_index, _descriptor_index) =
                constant_pool.try_get_name_and_type(*name_and_type_index)?;
            let field_name = constant_pool.try_get_utf8(*name_index)?;
            let field = object.field(field_name)?;
            field.set_value(value)?;
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
    use crate::call_stack::CallStack;
    use crate::frame::Frame;
    use crate::instruction::{dup, new};
    use crate::VM;
    use ristretto_classfile::MethodAccessFlags;
    use ristretto_classloader::{Method, Value};
    use std::sync::Arc;

    #[expect(clippy::type_complexity)]
    fn test_class_field(
        class_name: &str,
        field_name: &str,
        field_type: &str,
    ) -> Result<(Arc<VM>, Arc<CallStack>, Frame, u16, u16)> {
        let (vm, call_stack, mut class) = crate::test::class()?;
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
        let frame = Frame::new(
            &Arc::downgrade(&call_stack),
            &class,
            &Arc::new(method),
            arguments,
        )?;
        Ok((vm, call_stack, frame, class_index, field_index))
    }

    fn test_put_and_get_field() -> Result<()> {
        let (_vm, _call_stack, frame, class_index, field_index) =
            test_class_field("Child", "zero", "I")?;
        let constant_pool = frame.class().constant_pool().clone();
        let result = new(&frame, class_index)?;
        assert_eq!(Continue, result);

        let stack = frame.stack();
        let result = dup(stack)?;
        assert_eq!(Continue, result);

        let result = dup(stack)?;
        assert_eq!(Continue, result);

        stack.push_int(42)?;
        let result = putfield(stack, &constant_pool, field_index)?;
        assert_eq!(Continue, result);

        let result = getfield(stack, &constant_pool, field_index)?;
        assert_eq!(Continue, result);
        let value = stack.pop()?;
        assert_eq!(Value::Int(42), value);
        Ok(())
    }

    #[test]
    fn test_getfield() -> Result<()> {
        test_put_and_get_field()
    }

    #[test]
    fn test_getfield_field_not_found() -> Result<()> {
        let (_vm, _call_stack, frame, class_index, field_index) =
            test_class_field("Child", "foo", "I")?;
        let result = new(&frame, class_index)?;
        assert_eq!(Continue, result);
        let constant_pool = frame.class().constant_pool().clone();
        let stack = frame.stack();
        let result = getfield(stack, &constant_pool, field_index);
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_getfield_invalid_value() -> Result<()> {
        let (_vm, _call_stack, frame) = crate::test::frame()?;
        let stack = &mut OperandStack::with_max_size(2);
        let constant_pool = frame.class().constant_pool();
        stack.push_object(None)?;
        let result = getfield(stack, constant_pool, 0);
        assert!(matches!(result, Err(InvalidStackValue {
            expected,
            actual
        }) if expected == "object" && actual == "object(null)"));

        Ok(())
    }

    #[test]
    fn test_putfield() -> Result<()> {
        test_put_and_get_field()
    }

    #[test]
    fn test_putfield_field_not_found() -> Result<()> {
        let (_vm, _call_stack, frame, class_index, field_index) =
            test_class_field("Child", "foo", "I")?;
        let constant_pool = frame.class().constant_pool().clone();
        let result = new(&frame, class_index)?;
        let stack = frame.stack();
        assert_eq!(Continue, result);
        let result = dup(stack)?;
        assert_eq!(Continue, result);
        stack.push_int(42)?;
        let result = putfield(stack, &constant_pool, field_index);
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_putfield_invalid_value() -> Result<()> {
        let (_vm, _call_stack, frame) = crate::test::frame()?;
        let stack = &mut OperandStack::with_max_size(2);
        let constant_pool = frame.class().constant_pool();
        stack.push_object(None)?;
        stack.push_int(42)?;
        let result = putfield(stack, constant_pool, 0);
        assert!(matches!(result, Err(InvalidStackValue {
            expected,
            actual
        }) if expected == "object" && actual == "object(null)"));

        Ok(())
    }
}
