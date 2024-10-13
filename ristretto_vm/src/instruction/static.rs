use crate::frame::ExecutionResult::Continue;
use crate::frame::{ExecutionResult, Frame};
use crate::Result;
use ristretto_classfile::FieldType;

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.getstatic>
#[inline]
pub(crate) fn getstatic(frame: &mut Frame, index: u16) -> Result<ExecutionResult> {
    let call_stack = frame.call_stack()?;
    let constant_pool = frame.class().constant_pool();
    let vm = call_stack.vm()?;
    let (class_index, name_and_type_index) = constant_pool.try_get_field_ref(index)?;
    let (name_index, _descriptor_index) =
        constant_pool.try_get_name_and_type(*name_and_type_index)?;
    let class_name = constant_pool.try_get_class(*class_index)?;
    let class = vm.class(&call_stack, class_name)?;
    let field_name = constant_pool.try_get_utf8(*name_index)?;
    let field = class.static_field(field_name)?;
    let value = field.value()?;
    let stack = frame.stack_mut();
    stack.push(value)?;

    if let FieldType::Object(class_name) = field.field_type() {
        // Load the class of the field value if it is an object.
        // https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-5.html#jvms-5.4.3
        vm.class(&call_stack, class_name)?;
    }
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.putstatic>
#[inline]
pub(crate) fn putstatic(frame: &mut Frame, index: u16) -> Result<ExecutionResult> {
    let call_stack = frame.call_stack()?;
    let constant_pool = frame.class().constant_pool();
    let vm = call_stack.vm()?;
    let (class_index, name_and_type_index) = constant_pool.try_get_field_ref(index)?;
    let (name_index, _descriptor_index) =
        constant_pool.try_get_name_and_type(*name_and_type_index)?;
    let class_name = constant_pool.try_get_class(*class_index)?;
    let class = vm.class(&call_stack, class_name)?;
    let field_name = constant_pool.try_get_utf8(*name_index)?;
    let field = class.static_field(field_name)?;
    let stack = frame.stack_mut();
    let value = stack.pop()?;
    field.set_value(value)?;

    if let FieldType::Object(class_name) = field.field_type() {
        // Load the class of the field value if it is an object.
        // https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-5.html#jvms-5.4.3
        vm.class(&call_stack, class_name)?;
    }
    Ok(Continue)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::call_stack::CallStack;
    use crate::frame::Frame;
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
            &Arc::downgrade(&call_stack.clone()),
            &class,
            &Arc::new(method),
            arguments,
        )?;
        Ok((vm, call_stack, frame, class_index, field_index))
    }

    #[test]
    fn test_getstatic() -> Result<()> {
        let (_vm, _call_stack, mut frame, _class_index, field_index) =
            test_class_field("Constants", "INT_VALUE", "I")?;
        let result = getstatic(&mut frame, field_index)?;
        assert_eq!(Continue, result);
        let stack = frame.stack_mut();
        let value = stack.pop()?;
        assert_eq!(Value::Int(3), value);
        Ok(())
    }

    #[test]
    fn test_getstatic_field_not_found() -> Result<()> {
        let (_vm, _call_stack, mut frame, _class_index, field_index) =
            test_class_field("Child", "foo", "I")?;
        let result = getstatic(&mut frame, field_index);
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_putstatic() -> Result<()> {
        let (_vm, _call_stack, mut frame, _class_index, field_index) =
            test_class_field("Simple", "ANSWER", "I")?;
        {
            let stack = frame.stack_mut();
            stack.push_int(3)?;
        }
        let result = putstatic(&mut frame, field_index)?;
        assert_eq!(Continue, result);

        let result = getstatic(&mut frame, field_index)?;
        assert_eq!(Continue, result);
        let stack = frame.stack_mut();
        let value = stack.pop()?;
        assert_eq!(Value::Int(3), value);
        Ok(())
    }

    #[test]
    fn test_putstatic_field_not_found() -> Result<()> {
        let (_vm, _call_stack, mut frame, _class_index, field_index) =
            test_class_field("Child", "foo", "I")?;
        {
            let stack = frame.stack_mut();
            stack.push_int(3)?;
        }
        let result = putstatic(&mut frame, field_index);
        assert!(result.is_err());
        Ok(())
    }
}
