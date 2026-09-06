use crate::Result;
#[cfg(test)]
use crate::frame::ExecutionResult::Continue;
use crate::frame::{ExecutionResult, Frame};
use crate::operand_stack::OperandStack;

/// Get the value of a static field and push it onto the operand stack.
///
/// The getstatic instruction:
/// 1. Resolves the field reference [JVMS §5.4.3.2](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-5.html#jvms-5.4.3.2)
/// 2. Initializes the class, and its superclasses, that declares the field [JVMS §5.5](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-5.html#jvms-5.5)
/// 3. Gets the value of the static field
///
/// The slow path initializes the referenced class and its superclasses. Cached accesses check
/// initialization state before reading an inherited static field.
///
/// # References
///
/// - [JVMS §6.5.getstatic](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.getstatic)
#[inline]
pub(crate) async fn getstatic(
    frame: &Frame,
    stack: &mut OperandStack,
    index: u16,
) -> Result<ExecutionResult> {
    crate::instruction::execute_field(
        frame,
        stack,
        index,
        ristretto_classfile::attributes::Instruction::Getstatic(index),
    )
    .await
}

/// Set the value of a static field from the operand stack.
///
/// The putstatic instruction:
/// 1. Resolves the field reference [JVMS §5.4.3.2](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-5.html#jvms-5.4.3.2)
/// 2. Initializes the class, and its superclasses, that declares the field [JVMS §5.5](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-5.html#jvms-5.5)
/// 3. Sets the value of the static field
///
/// The slow path initializes the referenced class and its superclasses. Cached accesses check
/// initialization state before writing an inherited static field.
///
/// # References
///
/// - [JVMS §6.5.putstatic](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.putstatic)
#[inline]
pub(crate) async fn putstatic(
    frame: &Frame,
    stack: &mut OperandStack,
    index: u16,
) -> Result<ExecutionResult> {
    crate::instruction::execute_field(
        frame,
        stack,
        index,
        ristretto_classfile::attributes::Instruction::Putstatic(index),
    )
    .await
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
