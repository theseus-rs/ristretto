use crate::Error::InvalidStackValue;
use crate::JavaError::NullPointerException;
use crate::Result;
use crate::frame::ExecutionResult::Continue;
use crate::frame::{ExecutionResult, Frame};
use crate::operand_stack::OperandStack;
use crate::thread::Thread;
use ristretto_classfile::BaseType;
use ristretto_classfile::attributes::ArrayType;
use ristretto_classloader::{Reference, Value};

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.newarray>
#[inline]
pub(crate) fn newarray(
    stack: &mut OperandStack,
    array_type: &ArrayType,
) -> Result<ExecutionResult> {
    let count = stack.pop_int()?;
    let count = usize::try_from(count)?;
    let array = match array_type {
        ArrayType::Char => Value::from(vec![0 as char; count]),
        ArrayType::Float => Value::from(vec![0.0f32; count]),
        ArrayType::Double => Value::from(vec![0.0f64; count]),
        ArrayType::Boolean | ArrayType::Byte => Value::from(vec![0i8; count]),
        ArrayType::Short => Value::from(vec![0i16; count]),
        ArrayType::Int => Value::from(vec![0i32; count]),
        ArrayType::Long => Value::from(vec![0i64; count]),
    };
    stack.push(array)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.anewarray>
#[inline]
pub(crate) async fn anewarray(
    frame: &Frame,
    stack: &mut OperandStack,
    index: u16,
) -> Result<ExecutionResult> {
    let thread = frame.thread()?;
    let constant_pool = frame.class().constant_pool();
    let class_name = constant_pool.try_get_class(index)?;
    let array_class_name = format!("[L{class_name};");
    let class = thread.class(array_class_name.as_str()).await?;
    let count = stack.pop_int()?;
    let count = usize::try_from(count)?;
    let array = Value::from((class, vec![None; count]));
    stack.push(array)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.arraylength>
#[inline]
pub(crate) fn arraylength(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let length = match stack.pop_object()? {
        None => return Err(NullPointerException("array cannot be null".to_string()).into()),
        Some(Reference::ByteArray(ref array)) => array.len()?,
        Some(Reference::CharArray(ref array)) => array.len()?,
        Some(Reference::FloatArray(ref array)) => array.len()?,
        Some(Reference::DoubleArray(ref array)) => array.len()?,
        Some(Reference::ShortArray(ref array)) => array.len()?,
        Some(Reference::IntArray(ref array)) => array.len()?,
        Some(Reference::LongArray(ref array)) => array.len()?,
        Some(Reference::Array(ref object_array)) => object_array.elements.len()?,
        Some(object) => {
            return Err(InvalidStackValue {
                expected: "array".to_string(),
                actual: object.to_string(),
            });
        }
    };
    stack.push_int(i32::try_from(length)?)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.multianewarray>
#[inline]
pub(crate) async fn multianewarray(
    frame: &Frame,
    stack: &mut OperandStack,
    index: u16,
    dimensions: u8,
) -> Result<ExecutionResult> {
    let thread = frame.thread()?;
    let constant_pool = frame.class().constant_pool();
    let class_name = constant_pool.try_get_class(index)?;
    let class = thread.class(class_name).await?;

    // Pop dimension sizes from stack (in reverse order)
    let mut dimension_sizes = Vec::new();
    for _ in 0..dimensions {
        let count = stack.pop_int()?;
        let count = usize::try_from(count)?;
        dimension_sizes.push(count);
    }
    dimension_sizes.reverse();

    // Create the nested array structure
    let array =
        create_multidimensional_array(&thread, class.array_component_type(), &dimension_sizes, 0)
            .await?;

    stack.push_object(Some(array))?;
    Ok(Continue)
}

async fn create_multidimensional_array(
    thread: &Thread,
    component_type: &str,
    dimension_sizes: &[usize],
    depth: usize,
) -> Result<Reference> {
    let current_size = dimension_sizes[depth];

    if depth == dimension_sizes.len() - 1 {
        // This is the innermost dimension; create the actual array
        if component_type.len() == 1 {
            // Primitive array
            let base_type = BaseType::parse(component_type.chars().next().unwrap_or_default())?;
            let array = match base_type {
                BaseType::Char => Reference::from(vec![0 as char; current_size]),
                BaseType::Float => Reference::from(vec![0.0f32; current_size]),
                BaseType::Double => Reference::from(vec![0.0f64; current_size]),
                BaseType::Boolean | BaseType::Byte => Reference::from(vec![0i8; current_size]),
                BaseType::Short => Reference::from(vec![0i16; current_size]),
                BaseType::Int => Reference::from(vec![0i32; current_size]),
                BaseType::Long => Reference::from(vec![0i64; current_size]),
            };
            Ok(array)
        } else {
            // Object array
            let array_class_name = format!("[L{component_type};");
            let array_class = thread.class(&array_class_name).await?;
            Ok(Reference::from((array_class, vec![None; current_size])))
        }
    } else {
        // This is not the innermost dimension; create array of arrays
        let mut elements = Vec::new();
        for _ in 0..current_size {
            let sub_array = Box::pin(create_multidimensional_array(
                thread,
                component_type,
                dimension_sizes,
                depth + 1,
            ))
            .await?;
            elements.push(Some(sub_array));
        }

        // Build the array class name for this dimension
        let mut array_class_name = "[".repeat(dimension_sizes.len() - depth);
        if component_type.len() == 1 {
            array_class_name.push_str(component_type);
        } else {
            array_class_name.push('L');
            array_class_name.push_str(component_type);
            array_class_name.push(';');
        }

        let array_class = thread.class(&array_class_name).await?;
        Ok(Reference::from((array_class, elements)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error::JavaError;
    use crate::frame::ExecutionResult::Continue;
    use crate::java_object::JavaObject;
    use ristretto_classfile::attributes::ArrayType;
    use ristretto_classloader::Value;
    use std::sync::Arc;

    #[test]
    fn test_newarray_boolean() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(0)?;
        let result = newarray(stack, &ArrayType::Boolean)?;
        assert_eq!(Continue, result);
        let object = stack.pop()?;
        assert!(matches!(
            object,
            Value::Object(Some(Reference::ByteArray(_)))
        ));
        Ok(())
    }

    #[test]
    fn test_newarray_byte() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(0)?;
        let result = newarray(stack, &ArrayType::Byte)?;
        assert_eq!(Continue, result);
        let object = stack.pop()?;
        assert!(matches!(
            object,
            Value::Object(Some(Reference::ByteArray(_)))
        ));
        Ok(())
    }

    #[test]
    fn test_newarray_char() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(0)?;
        let result = newarray(stack, &ArrayType::Char)?;
        assert_eq!(Continue, result);
        let object = stack.pop()?;
        assert!(matches!(
            object,
            Value::Object(Some(Reference::CharArray(_)))
        ));
        Ok(())
    }

    #[test]
    fn test_newarray_double() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(0)?;
        let result = newarray(stack, &ArrayType::Double)?;
        assert_eq!(Continue, result);
        let object = stack.pop()?;
        assert!(matches!(
            object,
            Value::Object(Some(Reference::DoubleArray(_)))
        ));
        Ok(())
    }

    #[test]
    fn test_newarray_float() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(0)?;
        let result = newarray(stack, &ArrayType::Float)?;
        assert_eq!(Continue, result);
        let object = stack.pop()?;
        assert!(matches!(
            object,
            Value::Object(Some(Reference::FloatArray(_)))
        ));
        Ok(())
    }

    #[test]
    fn test_newarray_int() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(0)?;
        let result = newarray(stack, &ArrayType::Int)?;
        assert_eq!(Continue, result);
        let object = stack.pop()?;
        assert!(matches!(
            object,
            Value::Object(Some(Reference::IntArray(_)))
        ));
        Ok(())
    }

    #[test]
    fn test_newarray_long() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(0)?;
        let result = newarray(stack, &ArrayType::Long)?;
        assert_eq!(Continue, result);
        let object = stack.pop()?;
        assert!(matches!(
            object,
            Value::Object(Some(Reference::LongArray(_)))
        ));
        Ok(())
    }

    #[test]
    fn test_newarray_short() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(0)?;
        let result = newarray(stack, &ArrayType::Short)?;
        assert_eq!(Continue, result);
        let object = stack.pop()?;
        assert!(matches!(
            object,
            Value::Object(Some(Reference::ShortArray(_)))
        ));
        Ok(())
    }

    #[tokio::test]
    async fn test_anewarray() -> Result<()> {
        let (_vm, _thread, mut frame) = crate::test::frame().await?;
        let class = frame.class_mut();
        let stack = &mut OperandStack::with_max_size(1);
        let constant_pool = Arc::get_mut(class).expect("class").constant_pool_mut();
        let class_index = constant_pool.add_class("java/lang/Object")?;
        stack.push_int(0)?;
        let result = anewarray(&frame, stack, class_index).await?;
        assert_eq!(Continue, result);
        let Value::Object(Some(reference)) = stack.pop()? else {
            panic!("expected reference");
        };
        assert_eq!("[Ljava/lang/Object;", reference.class_name());
        Ok(())
    }

    #[test]
    fn test_arraylength_boolean() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(3)?;
        let result = newarray(stack, &ArrayType::Boolean)?;
        assert_eq!(Continue, result);
        let result = arraylength(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(3, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_arraylength_byte() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(3)?;
        let result = newarray(stack, &ArrayType::Byte)?;
        assert_eq!(Continue, result);
        let result = arraylength(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(3, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_arraylength_char() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(3)?;
        let result = newarray(stack, &ArrayType::Char)?;
        assert_eq!(Continue, result);
        let result = arraylength(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(3, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_arraylength_double() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(3)?;
        let result = newarray(stack, &ArrayType::Double)?;
        assert_eq!(Continue, result);
        let result = arraylength(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(3, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_arraylength_float() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(3)?;
        let result = newarray(stack, &ArrayType::Float)?;
        assert_eq!(Continue, result);
        let result = arraylength(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(3, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_arraylength_int() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(3)?;
        let result = newarray(stack, &ArrayType::Int)?;
        assert_eq!(Continue, result);
        let result = arraylength(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(3, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_arraylength_long() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(3)?;
        let result = newarray(stack, &ArrayType::Long)?;
        assert_eq!(Continue, result);
        let result = arraylength(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(3, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_arraylength_short() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(3)?;
        let result = newarray(stack, &ArrayType::Short)?;
        assert_eq!(Continue, result);
        let result = arraylength(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(3, stack.pop_int()?);
        Ok(())
    }

    #[tokio::test]
    async fn test_arraylength_object() -> Result<()> {
        let (_vm, _thread, mut frame) = crate::test::frame().await?;
        let class = frame.class_mut();
        let stack = &mut OperandStack::with_max_size(1);
        let constant_pool = Arc::get_mut(class).expect("class").constant_pool_mut();
        let class_index = constant_pool.add_class("java/lang/Object")?;
        stack.push_int(3)?;
        let result = anewarray(&frame, stack, class_index).await?;
        assert_eq!(Continue, result);

        let result = arraylength(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(3, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_arraylength_null() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_object(None)?;
        let result = arraylength(stack);
        assert!(matches!(result, Err(JavaError(NullPointerException(_)))));
        Ok(())
    }

    #[tokio::test]
    async fn test_arraylength_invalid_type() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let stack = &mut OperandStack::with_max_size(1);
        let invalid_value = "foo".to_object(&thread).await?;
        stack.push(invalid_value)?;
        let result = arraylength(stack);
        assert!(matches!(
            result,
            Err(InvalidStackValue {
                expected,
                actual
            }) if expected == "array" && actual == "String(\"foo\")"
        ));
        Ok(())
    }

    async fn test_multianewarray_single_dimension(class_name: &str) -> Result<()> {
        let (_vm, _thread, mut frame) = crate::test::frame().await?;
        let class = frame.class_mut();
        let stack = &mut OperandStack::with_max_size(1);
        let constant_pool = Arc::get_mut(class).expect("class").constant_pool_mut();
        let class_index = constant_pool.add_class(class_name)?;
        stack.push_int(0)?;
        let result = multianewarray(&frame, stack, class_index, 1).await?;
        assert_eq!(Continue, result);
        let object = stack.pop()?;
        assert!(matches!(
            object,
            Value::Object(Some(ref reference)) if reference.class_name() == class_name
        ));
        Ok(())
    }

    #[tokio::test]
    async fn test_multianewarray_byte() -> Result<()> {
        test_multianewarray_single_dimension("[B").await
    }

    #[tokio::test]
    async fn test_multianewarray_char() -> Result<()> {
        test_multianewarray_single_dimension("[C").await
    }

    #[tokio::test]
    async fn test_multianewarray_short() -> Result<()> {
        test_multianewarray_single_dimension("[S").await
    }

    #[tokio::test]
    async fn test_multianewarray_int() -> Result<()> {
        test_multianewarray_single_dimension("[I").await
    }

    #[tokio::test]
    async fn test_multianewarray_long() -> Result<()> {
        test_multianewarray_single_dimension("[J").await
    }

    #[tokio::test]
    async fn test_multianewarray_float() -> Result<()> {
        test_multianewarray_single_dimension("[F").await
    }

    #[tokio::test]
    async fn test_multianewarray_double() -> Result<()> {
        test_multianewarray_single_dimension("[D").await
    }

    #[tokio::test]
    async fn test_multianewarray_object() -> Result<()> {
        test_multianewarray_single_dimension("[Ljava/lang/Object;").await
    }

    #[tokio::test]
    async fn test_multianewarray_multiple_dimensions() -> Result<()> {
        let (_vm, _thread, mut frame) = crate::test::frame().await?;
        let class = frame.class_mut();
        let stack = &mut OperandStack::with_max_size(5);
        let constant_pool = Arc::get_mut(class).expect("class").constant_pool_mut();
        let class_name = "[[[[[I";
        let class_index = constant_pool.add_class(class_name)?;
        stack.push_int(1)?;
        stack.push_int(2)?;
        stack.push_int(3)?;
        stack.push_int(4)?;
        stack.push_int(5)?;
        let result = multianewarray(&frame, stack, class_index, 5).await?;
        assert_eq!(Continue, result);
        let object = stack.pop()?;
        assert!(matches!(
            object,
            Value::Object(Some(ref reference)) if reference.class_name() == class_name
        ));
        Ok(())
    }
}
