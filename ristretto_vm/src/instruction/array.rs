use crate::Error::{InternalError, InvalidStackValue};
use crate::JavaError::{NegativeArraySizeException, NullPointerException};
use crate::Result;
use crate::frame::ExecutionResult::Continue;
use crate::frame::{ExecutionResult, Frame};
use crate::operand_stack::OperandStack;
use crate::thread::Thread;
use ristretto_classfile::BaseType;
use ristretto_classfile::attributes::ArrayType;
use ristretto_classloader::{Class, Reference, Value};
use ristretto_types::array::{allocate_array, allocate_primitive_array};
use std::sync::Arc;

/// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.newarray>
#[inline]
pub(crate) fn newarray(
    frame: &Frame,
    stack: &mut OperandStack,
    array_type: &ArrayType,
) -> Result<()> {
    let thread = frame.thread()?;
    let vm = thread.vm()?;
    let count = stack.pop_int()?;
    if count < 0 {
        return Err(NegativeArraySizeException(count.to_string()).into());
    }
    let count = usize::try_from(count)?;
    let collector = vm.garbage_collector();
    let base_type = match array_type {
        ArrayType::Boolean => BaseType::Boolean,
        ArrayType::Byte => BaseType::Byte,
        ArrayType::Char => BaseType::Char,
        ArrayType::Float => BaseType::Float,
        ArrayType::Double => BaseType::Double,
        ArrayType::Short => BaseType::Short,
        ArrayType::Int => BaseType::Int,
        ArrayType::Long => BaseType::Long,
    };
    let array = Value::new_object(collector, allocate_primitive_array(&base_type, count)?);
    stack.push(array)?;
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.anewarray>
#[inline]
pub(crate) async fn anewarray(
    frame: &Frame,
    stack: &mut OperandStack,
    index: u16,
) -> Result<ExecutionResult> {
    let thread = frame.thread()?;
    let constant_pool = frame.class().constant_pool();
    let class_name = constant_pool.try_get_class(index)?;
    let component = thread
        .load_referenced_class(frame.class(), class_name)
        .await?;
    let class = thread.array_class(&component, 1).await?;
    let count = stack.pop_int()?;
    if count < 0 {
        return Err(NegativeArraySizeException(count.to_string()).into());
    }
    let count = usize::try_from(count)?;
    let vm = thread.vm()?;
    let reference = Reference::try_from((class, allocate_array(count, Value::Object(None))?))?;
    let array = Value::new_object(vm.garbage_collector(), reference);
    stack.push(array)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.arraylength>
#[inline]
pub(crate) fn arraylength(stack: &mut OperandStack) -> Result<()> {
    let Some(reference) = stack.pop_object()? else {
        return Err(NullPointerException(None).into());
    };
    let guard = reference.read();
    let length = match &*guard {
        Reference::BooleanArray(array) | Reference::ByteArray(array) => array.len(),
        Reference::CharArray(array) => array.len(),
        Reference::FloatArray(array) => array.len(),
        Reference::DoubleArray(array) => array.len(),
        Reference::ShortArray(array) => array.len(),
        Reference::IntArray(array) => array.len(),
        Reference::LongArray(array) => array.len(),
        Reference::Array(object_array) => object_array.elements.len(),
        object @ Reference::Object(_) => {
            return Err(InvalidStackValue {
                expected: "array".to_string(),
                actual: object.to_string(),
            });
        }
    };
    stack.push_int(i32::try_from(length)?)?;
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.multianewarray>
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
    let class = thread
        .load_referenced_class(frame.class(), class_name)
        .await?;

    // Pop dimension sizes from stack (in reverse order)
    let mut dimension_sizes = Vec::new();
    for _ in 0..dimensions {
        let count = stack.pop_int()?;
        if count < 0 {
            return Err(NegativeArraySizeException(count.to_string()).into());
        }
        let count = usize::try_from(count)?;
        dimension_sizes.push(count);
    }
    dimension_sizes.reverse();

    let array = create_multidimensional_array(&thread, class, &dimension_sizes, 0).await?;

    stack.push(array)?;
    Ok(Continue)
}

async fn create_multidimensional_array(
    thread: &Thread,
    class: Arc<Class>,
    dimension_sizes: &[usize],
    depth: usize,
) -> Result<Value> {
    let current_size = *dimension_sizes
        .get(depth)
        .ok_or_else(|| InternalError(format!("Invalid array dimension depth: {depth}")))?;

    if depth.checked_add(1) == Some(dimension_sizes.len()) {
        // Only a one-dimensional primitive array needs primitive backing storage.
        // Partially allocated arrays retain their full rank and null inner dimensions.
        let component_type = class.array_component_type();
        if class.name().len() == 2 {
            // Primitive array
            let base_type = BaseType::parse(component_type.chars().next().unwrap_or_default())?;
            let vm = thread.vm()?;
            let collector = vm.garbage_collector();
            let array = Value::new_object(
                collector,
                allocate_primitive_array(&base_type, current_size)?,
            );
            Ok(array)
        } else {
            // Object array
            let vm = thread.vm()?;
            let reference =
                Reference::try_from((class, allocate_array(current_size, Value::Object(None))?))?;
            let array = Value::new_object(vm.garbage_collector(), reference);
            Ok(array)
        }
    } else {
        let component = thread.array_component_class(&class).await?;
        let mut elements = allocate_array(current_size, Value::Object(None))?;
        for element in &mut elements {
            *element = Box::pin(create_multidimensional_array(
                thread,
                component.clone(),
                dimension_sizes,
                depth + 1,
            ))
            .await?;
        }

        let vm = thread.vm()?;
        let reference = Reference::try_from((class, elements))?;
        let array = Value::new_object(vm.garbage_collector(), reference);
        Ok(array)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error::JavaError;
    use crate::frame::ExecutionResult::Continue;
    use crate::java_object::JavaObject;
    use ristretto_classfile::attributes::ArrayType;
    use std::sync::Arc;

    /// Reproduce #940 without allocating gigabytes on a 64-bit test host.
    #[tokio::test]
    async fn test_multianewarray_capacity_overflow() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        for component in ["J", "Ljava/lang/Object;"] {
            for dimensions in [&[usize::MAX][..], &[usize::MAX, 1], &[1, usize::MAX]] {
                let class_name = format!("{}{component}", "[".repeat(dimensions.len()));
                let class = thread.class(&class_name).await?;
                let result = create_multidimensional_array(&thread, class, dimensions, 0).await;
                assert!(matches!(
                    result,
                    Err(JavaError(crate::JavaError::OutOfMemoryError(_)))
                ));
            }
        }
        Ok(())
    }

    /// The exact playground reproducer overflows Vec's capacity on wasm32.
    #[cfg(target_pointer_width = "32")]
    #[tokio::test]
    async fn test_newarray_capacity_overflow() -> Result<()> {
        let (_vm, _thread, frame) = crate::test::frame().await?;
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(i32::MAX - 8)?;
        let result = newarray(&frame, stack, &ArrayType::Long);
        assert!(matches!(
            result,
            Err(JavaError(crate::JavaError::OutOfMemoryError(_)))
        ));
        // A failed allocation must leave the VM usable.
        stack.push_int(1)?;
        newarray(&frame, stack, &ArrayType::Long)?;
        assert_eq!(&[0_i64], &*stack.pop()?.as_long_vec_ref()?);
        Ok(())
    }

    #[tokio::test]
    async fn test_newarray_boolean() -> Result<()> {
        let (_vm, _thread, frame) = crate::test::frame().await?;
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(0)?;
        newarray(&frame, stack, &ArrayType::Boolean)?;
        let object = stack.pop()?;
        let reference = object.as_reference()?;
        assert!(matches!(*reference, Reference::BooleanArray(_)));
        Ok(())
    }

    #[tokio::test]
    async fn test_newarray_byte() -> Result<()> {
        let (_vm, _thread, frame) = crate::test::frame().await?;
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(0)?;
        newarray(&frame, stack, &ArrayType::Byte)?;
        let object = stack.pop()?;
        let reference = object.as_reference()?;
        assert!(matches!(*reference, Reference::ByteArray(_)));
        Ok(())
    }

    #[tokio::test]
    async fn test_newarray_char() -> Result<()> {
        let (_vm, _thread, frame) = crate::test::frame().await?;
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(0)?;
        newarray(&frame, stack, &ArrayType::Char)?;
        let object = stack.pop()?;
        let reference = object.as_reference()?;
        assert!(matches!(*reference, Reference::CharArray(_)));
        Ok(())
    }

    #[tokio::test]
    async fn test_newarray_double() -> Result<()> {
        let (_vm, _thread, frame) = crate::test::frame().await?;
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(0)?;
        newarray(&frame, stack, &ArrayType::Double)?;
        let object = stack.pop()?;
        let reference = object.as_reference()?;
        assert!(matches!(*reference, Reference::DoubleArray(_)));
        Ok(())
    }

    #[tokio::test]
    async fn test_newarray_float() -> Result<()> {
        let (_vm, _thread, frame) = crate::test::frame().await?;
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(0)?;
        newarray(&frame, stack, &ArrayType::Float)?;
        let object = stack.pop()?;
        let reference = object.as_reference()?;
        assert!(matches!(*reference, Reference::FloatArray(_)));
        Ok(())
    }

    #[tokio::test]
    async fn test_newarray_int() -> Result<()> {
        let (_vm, _thread, frame) = crate::test::frame().await?;
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(0)?;
        newarray(&frame, stack, &ArrayType::Int)?;
        let object = stack.pop()?;
        let reference = object.as_reference()?;
        assert!(matches!(*reference, Reference::IntArray(_)));
        Ok(())
    }

    #[tokio::test]
    async fn test_newarray_long() -> Result<()> {
        let (_vm, _thread, frame) = crate::test::frame().await?;
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(0)?;
        newarray(&frame, stack, &ArrayType::Long)?;
        let object = stack.pop()?;
        let reference = object.as_reference()?;
        assert!(matches!(*reference, Reference::LongArray(_)));
        Ok(())
    }

    #[tokio::test]
    async fn test_newarray_short() -> Result<()> {
        let (_vm, _thread, frame) = crate::test::frame().await?;
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(0)?;
        newarray(&frame, stack, &ArrayType::Short)?;
        let object = stack.pop()?;
        let reference = object.as_reference()?;
        assert!(matches!(*reference, Reference::ShortArray(_)));
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
        let object = stack.pop()?;
        let reference = object.as_reference()?;
        assert_eq!("[Ljava/lang/Object;", reference.class_name()?);
        Ok(())
    }

    #[tokio::test]
    async fn test_anewarray_of_arrays() -> Result<()> {
        let (_vm, _thread, mut frame) = crate::test::frame().await?;
        let class = frame.class_mut();
        let stack = &mut OperandStack::with_max_size(1);
        let constant_pool = Arc::get_mut(class).expect("class").constant_pool_mut();
        let class_index = constant_pool.add_class("[Ljava/lang/Object;")?;
        stack.push_int(0)?;
        let result = anewarray(&frame, stack, class_index).await?;
        assert_eq!(Continue, result);
        let object = stack.pop()?;
        let reference = object.as_reference()?;
        assert_eq!("[[Ljava/lang/Object;", reference.class_name()?);
        Ok(())
    }

    #[tokio::test]
    async fn test_arraylength_boolean() -> Result<()> {
        let (_vm, _thread, frame) = crate::test::frame().await?;
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(3)?;
        newarray(&frame, stack, &ArrayType::Boolean)?;
        arraylength(stack)?;
        assert_eq!(3, stack.pop_int()?);
        Ok(())
    }

    #[tokio::test]
    async fn test_arraylength_byte() -> Result<()> {
        let (_vm, _thread, frame) = crate::test::frame().await?;
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(3)?;
        newarray(&frame, stack, &ArrayType::Byte)?;
        arraylength(stack)?;
        assert_eq!(3, stack.pop_int()?);
        Ok(())
    }

    #[tokio::test]
    async fn test_arraylength_char() -> Result<()> {
        let (_vm, _thread, frame) = crate::test::frame().await?;
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(3)?;
        newarray(&frame, stack, &ArrayType::Char)?;
        arraylength(stack)?;
        assert_eq!(3, stack.pop_int()?);
        Ok(())
    }

    #[tokio::test]
    async fn test_arraylength_double() -> Result<()> {
        let (_vm, _thread, frame) = crate::test::frame().await?;
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(3)?;
        newarray(&frame, stack, &ArrayType::Double)?;
        arraylength(stack)?;
        assert_eq!(3, stack.pop_int()?);
        Ok(())
    }

    #[tokio::test]
    async fn test_arraylength_float() -> Result<()> {
        let (_vm, _thread, frame) = crate::test::frame().await?;
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(3)?;
        newarray(&frame, stack, &ArrayType::Float)?;
        arraylength(stack)?;
        assert_eq!(3, stack.pop_int()?);
        Ok(())
    }

    #[tokio::test]
    async fn test_arraylength_int() -> Result<()> {
        let (_vm, _thread, frame) = crate::test::frame().await?;
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(3)?;
        newarray(&frame, stack, &ArrayType::Int)?;
        arraylength(stack)?;
        assert_eq!(3, stack.pop_int()?);
        Ok(())
    }

    #[tokio::test]
    async fn test_arraylength_long() -> Result<()> {
        let (_vm, _thread, frame) = crate::test::frame().await?;
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(3)?;
        newarray(&frame, stack, &ArrayType::Long)?;
        arraylength(stack)?;
        assert_eq!(3, stack.pop_int()?);
        Ok(())
    }

    #[tokio::test]
    async fn test_arraylength_short() -> Result<()> {
        let (_vm, _thread, frame) = crate::test::frame().await?;
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(3)?;
        newarray(&frame, stack, &ArrayType::Short)?;
        arraylength(stack)?;
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

        arraylength(stack)?;
        assert_eq!(3, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_arraylength_null() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_object(None)?;
        let result = arraylength(stack);
        assert!(matches!(result, Err(JavaError(NullPointerException(None)))));
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
        let reference = object.as_reference()?;
        assert_eq!(class_name, reference.class_name()?);
        Ok(())
    }

    #[tokio::test]
    async fn test_multianewarray_boolean() -> Result<()> {
        test_multianewarray_single_dimension("[Z").await
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
        let reference = object.as_reference()?;
        assert_eq!(class_name, reference.class_name()?);
        Ok(())
    }
}
