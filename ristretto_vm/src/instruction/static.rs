use crate::Result;
use crate::frame::ExecutionResult::Continue;
use crate::frame::{ExecutionResult, Frame};
use crate::operand_stack::OperandStack;

/// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.getstatic>
///
/// The getstatic instruction:
/// 1. Resolves the field reference [JVMS §5.4.3.2](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-5.html#jvms-5.4.3.2)
/// 2. Initializes the class, and its superclasses, that declares the field [JVMS §5.5](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-5.html#jvms-5.5)
/// 3. Gets the value of the static field
///
/// Note: When `thread.class()` is called, it initializes the class and all its superclasses in
/// order (from Object down to the referenced class). This ensures that any inherited static fields
/// are properly initialized before we access them.
#[inline]
pub(crate) async fn getstatic(
    frame: &Frame,
    stack: &mut OperandStack,
    index: u16,
) -> Result<ExecutionResult> {
    let thread = frame.thread()?;
    let constant_pool = frame.class().constant_pool();
    let (class_index, name_and_type_index) = constant_pool.try_get_field_ref(index)?;
    let (name_index, _descriptor_index) =
        constant_pool.try_get_name_and_type(*name_and_type_index)?;
    let class_name = constant_pool.try_get_class(*class_index)?;

    // Initialize the class and its superclasses:
    // https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-5.html#jvms-5.5
    // This ensures parent classes are initialized first, which is required for inherited static
    // fields to be properly set up.
    let class = thread.class(class_name).await?;
    let field_name = constant_pool.try_get_utf8(*name_index)?;

    // Verify the field exists; searches class hierarchy per:
    // https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-5.html#jvms-5.4.3.2
    let _field = class.static_field(field_name)?;

    // Get the value (searches class hierarchy and returns from declaring class)
    let value = class.static_value(field_name)?;
    stack.push(value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.putstatic>
///
/// The putstatic instruction:
/// 1. Resolves the field reference [JVMS §5.4.3.2](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-5.html#jvms-5.4.3.2)
/// 2. Initializes the class, and its superclasses, that declares the field [JVMS §5.5](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-5.html#jvms-5.5)
/// 3. Sets the value of the static field
///
/// Note: When `thread.class()` is called, it initializes the class and all its superclasses in
/// order (from Object down to the referenced class). This ensures that any inherited static fields
/// are properly initialized before we modify them.
#[inline]
pub(crate) async fn putstatic(
    frame: &Frame,
    stack: &mut OperandStack,
    index: u16,
) -> Result<ExecutionResult> {
    let thread = frame.thread()?;
    let constant_pool = frame.class().constant_pool();
    let (class_index, name_and_type_index) = constant_pool.try_get_field_ref(index)?;
    let (name_index, _descriptor_index) =
        constant_pool.try_get_name_and_type(*name_and_type_index)?;
    let class_name = constant_pool.try_get_class(*class_index)?;

    // Initialize the class and its superclasses:
    // https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-5.html#jvms-5.5
    // This ensures parent classes are initialized first, which is required for inherited static
    // fields to be properly set up.
    let class = thread.class(class_name).await?;
    let field_name = constant_pool.try_get_utf8(*name_index)?;

    // Verify the field exists; searches class hierarchy per:
    // https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-5.html#jvms-5.4.3.2
    let _field = class.static_field(field_name)?;

    // Set the value (searches class hierarchy and sets in declaring class)
    let value = stack.pop()?;
    class.set_static_value(field_name, value)?;
    Ok(Continue)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::VM;
    use crate::frame::Frame;
    use crate::operand_stack::OperandStack;
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

    #[tokio::test]
    async fn test_getstatic() -> Result<()> {
        let (_vm, _thread, frame, _class_index, field_index) =
            test_class_field("java.lang.Integer", "MAX_VALUE", "I").await?;
        let stack = &mut OperandStack::with_max_size(1);
        let result = getstatic(&frame, stack, field_index).await?;
        assert_eq!(Continue, result);
        let value = stack.pop()?;
        assert_eq!(Value::Int(i32::MAX), value);
        Ok(())
    }

    #[tokio::test]
    async fn test_getstatic_field_not_found() -> Result<()> {
        let (_vm, _thread, frame, _class_index, field_index) =
            test_class_field("java.lang.Integer", "foo", "I").await?;
        let stack = &mut OperandStack::with_max_size(1);
        let result = getstatic(&frame, stack, field_index).await;
        assert!(result.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_putstatic() -> Result<()> {
        let (_vm, _thread, frame, _class_index, field_index) =
            test_class_field("Simple", "ANSWER", "I").await?;
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(3)?;
        let result = putstatic(&frame, stack, field_index).await?;
        assert_eq!(Continue, result);

        let result = getstatic(&frame, stack, field_index).await?;
        assert_eq!(Continue, result);
        let value = stack.pop()?;
        assert_eq!(Value::Int(3), value);
        Ok(())
    }

    #[tokio::test]
    async fn test_putstatic_field_not_found() -> Result<()> {
        let (_vm, _thread, frame, _class_index, field_index) =
            test_class_field("java.lang.Integer", "foo", "I").await?;
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(3)?;
        let result = putstatic(&frame, stack, field_index).await;
        assert!(result.is_err());
        Ok(())
    }
}
