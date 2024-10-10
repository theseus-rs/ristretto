use crate::call_stack::CallStack;
use crate::frame::ExecutionResult;
use crate::frame::ExecutionResult::Continue;
use crate::operand_stack::OperandStack;
use crate::Error::{InvalidStackValue, NullPointer};
use crate::{Result, VM};
use ristretto_classfile::attributes::ArrayType;
use ristretto_classloader::{Class, ConcurrentVec, Reference};
use std::sync::Arc;

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.newarray>
#[inline]
pub(crate) fn newarray(
    stack: &mut OperandStack,
    array_type: &ArrayType,
) -> Result<ExecutionResult> {
    let count = stack.pop_int()?;
    let count = usize::try_from(count)?;
    let array = match array_type {
        ArrayType::Char => Reference::CharArray(ConcurrentVec::from(vec![0; count])),
        ArrayType::Float => Reference::FloatArray(ConcurrentVec::from(vec![0.0; count])),
        ArrayType::Double => Reference::DoubleArray(ConcurrentVec::from(vec![0.0; count])),
        ArrayType::Boolean | ArrayType::Byte => {
            Reference::ByteArray(ConcurrentVec::from(vec![0; count]))
        }
        ArrayType::Short => Reference::ShortArray(ConcurrentVec::from(vec![0; count])),
        ArrayType::Int => Reference::IntArray(ConcurrentVec::from(vec![0; count])),
        ArrayType::Long => Reference::LongArray(ConcurrentVec::from(vec![0; count])),
    };
    stack.push_object(Some(array))?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.anewarray>
#[inline]
pub(crate) fn anewarray(
    vm: &VM,
    call_stack: &mut CallStack,
    stack: &mut OperandStack,
    class: &Arc<Class>,
    index: u16,
) -> Result<ExecutionResult> {
    let count = stack.pop_int()?;
    let count = usize::try_from(count)?;
    let constant_pool = class.constant_pool();
    let class_name = constant_pool.try_get_class(index)?;
    let class = vm.class(call_stack, class_name)?;
    let array = Reference::Array(class, ConcurrentVec::from(vec![None; count]));
    stack.push_object(Some(array))?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.arraylength>
#[inline]
pub(crate) fn arraylength(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let length = match stack.pop_object()? {
        None => return Err(NullPointer),
        Some(Reference::ByteArray(ref array)) => array.len()?,
        Some(Reference::CharArray(ref array)) => array.len()?,
        Some(Reference::FloatArray(ref array)) => array.len()?,
        Some(Reference::DoubleArray(ref array)) => array.len()?,
        Some(Reference::ShortArray(ref array)) => array.len()?,
        Some(Reference::IntArray(ref array)) => array.len()?,
        Some(Reference::LongArray(ref array)) => array.len()?,
        Some(Reference::Array(_class, ref array)) => array.len()?,
        Some(object) => {
            return Err(InvalidStackValue {
                expected: "array".to_string(),
                actual: object.to_string(),
            })
        }
    };
    stack.push_int(i32::try_from(length)?)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.multianewarray>
#[inline]
pub(crate) fn multianewarray(
    vm: &VM,
    call_stack: &mut CallStack,
    stack: &mut OperandStack,
    class: &Arc<Class>,
    index: u16,
    dimensions: u8,
) -> Result<ExecutionResult> {
    let count = stack.pop_int()?;
    let count = usize::try_from(count)?;
    let constant_pool = class.constant_pool();
    let class_name = constant_pool.try_get_class(index)?;
    let class = vm.class(call_stack, class_name)?;
    let array = Reference::Array(class, ConcurrentVec::from(vec![None; count]));

    for _ in 1..dimensions {
        let count = stack.pop_int()?;
        let _count = usize::try_from(count)?;
        todo!()
    }

    stack.push_object(Some(array))?;
    Ok(Continue)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame::ExecutionResult::Continue;
    use crate::frame::Frame;
    use ristretto_classfile::attributes::ArrayType;
    use ristretto_classfile::MethodAccessFlags;
    use ristretto_classloader::{Method, Value};
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

    #[test]
    fn test_anewarray() -> Result<()> {
        let (vm, mut call_stack, mut class) = crate::test::class()?;
        let constant_pool = Arc::get_mut(&mut class).expect("class").constant_pool_mut();
        let class_index = constant_pool.add_class("java/lang/Object")?;
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
        let mut frame = Frame::new(&class, &Arc::new(method), arguments)?;
        let stack = &mut frame.stack;
        let class = frame.class;

        stack.push_int(0)?;
        let result = anewarray(&vm, &mut call_stack, stack, &class, class_index)?;
        assert_eq!(Continue, result);
        let object = stack.pop()?;
        assert!(matches!(
            object,
            Value::Object(Some(Reference::Array(_, _)))
        ));
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

    #[test]
    fn test_arraylength_object() -> Result<()> {
        let (vm, mut call_stack, mut class) = crate::test::class()?;
        let constant_pool = Arc::get_mut(&mut class).expect("class").constant_pool_mut();
        let class_index = constant_pool.add_class("java/lang/Object")?;
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
        let mut frame = Frame::new(&class, &Arc::new(method), arguments)?;
        let stack = &mut frame.stack;
        let class = frame.class;

        stack.push_int(3)?;
        let result = anewarray(&vm, &mut call_stack, stack, &class, class_index)?;
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
        assert!(matches!(result, Err(NullPointer)));
        Ok(())
    }

    #[test]
    fn test_arraylength_invalid_type() -> Result<()> {
        let (vm, mut call_stack, mut frame) = crate::test::frame()?;
        let invalid_value = vm.to_string_value(&mut call_stack, "foo")?;
        let stack = &mut frame.stack;
        stack.push(invalid_value)?;
        let result = arraylength(stack);
        assert!(matches!(
            result,
            Err(InvalidStackValue {
                expected,
                actual
            }) if expected == "array" && actual == "string(foo)"
        ));
        Ok(())
    }

    // #[test]
    // fn test_multianewarray() -> Result<()> {
    //     todo!()
    // }
}
