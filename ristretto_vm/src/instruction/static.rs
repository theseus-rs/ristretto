use crate::frame::ExecutionResult::Continue;
use crate::frame::{ExecutionResult, Frame};
use crate::Result;
use ristretto_classfile::FieldType;

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.getstatic>
#[inline]
pub(crate) async fn getstatic(frame: &Frame, index: u16) -> Result<ExecutionResult> {
    let thread = frame.thread()?;
    let constant_pool = frame.class().constant_pool();
    let (class_index, name_and_type_index) = constant_pool.try_get_field_ref(index)?;
    let (name_index, _descriptor_index) =
        constant_pool.try_get_name_and_type(*name_and_type_index)?;
    let class_name = constant_pool.try_get_class(*class_index)?;
    let class = thread.class(class_name).await?;
    let field_name = constant_pool.try_get_utf8(*name_index)?;
    let field = class.static_field(field_name)?;
    let value = field.value()?;
    let stack = frame.stack();
    stack.push(value)?;

    if let FieldType::Object(class_name) = field.field_type() {
        // Load the class of the field value if it is an object.
        // https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-5.html#jvms-5.4.3
        thread.class(class_name).await?;
    }
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.putstatic>
#[inline]
pub(crate) async fn putstatic(frame: &Frame, index: u16) -> Result<ExecutionResult> {
    let thread = frame.thread()?;
    let constant_pool = frame.class().constant_pool();
    let (class_index, name_and_type_index) = constant_pool.try_get_field_ref(index)?;
    let (name_index, _descriptor_index) =
        constant_pool.try_get_name_and_type(*name_and_type_index)?;
    let class_name = constant_pool.try_get_class(*class_index)?;
    let class = thread.class(class_name).await?;
    let field_name = constant_pool.try_get_utf8(*name_index)?;
    let field = class.static_field(field_name)?;
    let stack = frame.stack();
    let value = stack.pop()?;
    field.set_value(value)?;

    if let FieldType::Object(class_name) = field.field_type() {
        // Load the class of the field value if it is an object.
        // https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-5.html#jvms-5.4.3
        thread.class(class_name).await?;
    }
    Ok(Continue)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::frame::Frame;
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
            &Arc::downgrade(&thread.clone()),
            &class,
            &Arc::new(method),
            arguments,
        )?;
        Ok((vm, thread, frame, class_index, field_index))
    }

    #[tokio::test]
    async fn test_getstatic() -> Result<()> {
        let (_vm, _thread, frame, _class_index, field_index) =
            test_class_field("Constants", "INT_VALUE", "I").await?;
        let result = getstatic(&frame, field_index).await?;
        assert_eq!(Continue, result);
        let stack = frame.stack();
        let value = stack.pop()?;
        assert_eq!(Value::Int(3), value);
        Ok(())
    }

    #[tokio::test]
    async fn test_getstatic_field_not_found() -> Result<()> {
        let (_vm, _thread, frame, _class_index, field_index) =
            test_class_field("Child", "foo", "I").await?;
        let result = getstatic(&frame, field_index).await;
        assert!(result.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_putstatic() -> Result<()> {
        let (_vm, _thread, frame, _class_index, field_index) =
            test_class_field("Simple", "ANSWER", "I").await?;
        let stack = frame.stack();
        stack.push_int(3)?;
        let result = putstatic(&frame, field_index).await?;
        assert_eq!(Continue, result);

        let result = getstatic(&frame, field_index).await?;
        assert_eq!(Continue, result);
        let stack = frame.stack();
        let value = stack.pop()?;
        assert_eq!(Value::Int(3), value);
        Ok(())
    }

    #[tokio::test]
    async fn test_putstatic_field_not_found() -> Result<()> {
        let (_vm, _thread, frame, _class_index, field_index) =
            test_class_field("Child", "foo", "I").await?;
        let stack = frame.stack();
        stack.push_int(3)?;
        let result = putstatic(&frame, field_index).await;
        assert!(result.is_err());
        Ok(())
    }
}
