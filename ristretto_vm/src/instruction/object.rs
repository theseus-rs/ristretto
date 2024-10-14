use crate::frame::ExecutionResult::Return;
use crate::frame::{ExecutionResult, ExecutionResult::Continue, Frame};
use crate::local_variables::LocalVariables;
use crate::operand_stack::OperandStack;
use crate::Error::{
    ArrayIndexOutOfBounds, ClassCastError, InvalidStackValue, NullPointer, RuntimeError,
};
use crate::{Result, Value};
use ristretto_classloader::{Object, Reference};

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.aconst_null>
#[inline]
pub(crate) fn aconst_null(stack: &OperandStack) -> Result<ExecutionResult> {
    stack.push_object(None)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.aload>
#[inline]
pub(crate) fn aload(
    locals: &LocalVariables,
    stack: &OperandStack,
    index: u8,
) -> Result<ExecutionResult> {
    let object = locals.get_object(usize::from(index))?;
    stack.push_object(object)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.aload>
/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.wide>
#[inline]
pub(crate) fn aload_w(
    locals: &LocalVariables,
    stack: &OperandStack,
    index: u16,
) -> Result<ExecutionResult> {
    let object = locals.get_object(usize::from(index))?;
    stack.push_object(object)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.aload_n>
#[inline]
pub(crate) fn aload_0(locals: &LocalVariables, stack: &OperandStack) -> Result<ExecutionResult> {
    let object = locals.get_object(0)?;
    stack.push_object(object)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.aload_n>
#[inline]
pub(crate) fn aload_1(locals: &LocalVariables, stack: &OperandStack) -> Result<ExecutionResult> {
    let object = locals.get_object(1)?;
    stack.push_object(object)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.aload_n>
#[inline]
pub(crate) fn aload_2(locals: &LocalVariables, stack: &OperandStack) -> Result<ExecutionResult> {
    let object = locals.get_object(2)?;
    stack.push_object(object)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.aload_n>
#[inline]
pub(crate) fn aload_3(locals: &LocalVariables, stack: &OperandStack) -> Result<ExecutionResult> {
    let object = locals.get_object(3)?;
    stack.push_object(object)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.astore>
#[inline]
pub(crate) fn astore(
    locals: &LocalVariables,
    stack: &OperandStack,
    index: u8,
) -> Result<ExecutionResult> {
    let value = stack.pop_object()?;
    locals.set_object(usize::from(index), value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.astore>
/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.wide>
#[inline]
pub(crate) fn astore_w(
    locals: &LocalVariables,
    stack: &OperandStack,
    index: u16,
) -> Result<ExecutionResult> {
    let value = stack.pop_object()?;
    locals.set_object(usize::from(index), value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.astore_n>
#[inline]
pub(crate) fn astore_0(locals: &LocalVariables, stack: &OperandStack) -> Result<ExecutionResult> {
    let value = stack.pop_object()?;
    locals.set_object(0, value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.astore_n>
#[inline]
pub(crate) fn astore_1(locals: &LocalVariables, stack: &OperandStack) -> Result<ExecutionResult> {
    let value = stack.pop_object()?;
    locals.set_object(1, value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.astore_n>
#[inline]
pub(crate) fn astore_2(locals: &LocalVariables, stack: &OperandStack) -> Result<ExecutionResult> {
    let value = stack.pop_object()?;
    locals.set_object(2, value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.astore_n>
#[inline]
pub(crate) fn astore_3(locals: &LocalVariables, stack: &OperandStack) -> Result<ExecutionResult> {
    let value = stack.pop_object()?;
    locals.set_object(3, value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.aaload>
#[inline]
pub(crate) fn aaload(stack: &OperandStack) -> Result<ExecutionResult> {
    let index = stack.pop_int()?;
    match stack.pop_object()? {
        None => Err(NullPointer),
        Some(Reference::Array(_class, array)) => {
            let index = usize::try_from(index)?;
            let Some(value) = array.get(index)? else {
                return Err(ArrayIndexOutOfBounds(index));
            };
            stack.push_object(value)?;
            Ok(Continue)
        }
        Some(object) => Err(InvalidStackValue {
            expected: "reference array".to_string(),
            actual: object.to_string(),
        }),
    }
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.aastore>
#[inline]
pub(crate) fn aastore(stack: &OperandStack) -> Result<ExecutionResult> {
    let value = stack.pop_object()?;
    let index = stack.pop_int()?;
    match stack.pop_object()? {
        None => Err(NullPointer),
        Some(Reference::Array(_class, ref mut array)) => {
            let index = usize::try_from(index)?;
            if index >= array.capacity()? {
                return Err(ArrayIndexOutOfBounds(index));
            };
            // TODO: validate object type is compatible with array type
            // See: https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.aastore
            array.set(index, value)?;
            Ok(Continue)
        }
        Some(object) => Err(InvalidStackValue {
            expected: "reference array".to_string(),
            actual: object.to_string(),
        }),
    }
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.areturn>
#[inline]
pub(crate) fn areturn(stack: &OperandStack) -> Result<ExecutionResult> {
    let value = stack.pop_object()?;
    Ok(Return(Some(Value::Object(value))))
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.new>
#[inline]
pub(crate) async fn new(frame: &Frame, index: u16) -> Result<ExecutionResult> {
    let call_stack = frame.call_stack()?;
    let vm = call_stack.vm()?;
    let constant_pool = frame.class().constant_pool();
    let class_name = constant_pool.try_get_class(index)?;
    let class = vm.class(&call_stack, class_name).await?;
    let object = Object::new(class)?;
    let reference = Reference::Object(object);
    let stack = frame.stack();
    stack.push_object(Some(reference))?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.checkcast>
#[inline]
pub(crate) fn checkcast(stack: &OperandStack, class_name: &str) -> Result<ExecutionResult> {
    let Value::Object(object) = stack.peek()? else {
        return Err(RuntimeError("Expected object".to_string()));
    };
    let Some(object) = object else {
        return Ok(Continue);
    };
    if !is_instanceof(&object, class_name)? {
        return Err(ClassCastError(class_name.to_string()));
    }
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.instanceof>
#[inline]
pub(crate) fn instanceof(stack: &OperandStack, class_name: &str) -> Result<ExecutionResult> {
    let object = stack.pop_object()?;
    let Some(object) = object else {
        stack.push_int(0)?;
        return Ok(Continue);
    };
    if is_instanceof(&object, class_name)? {
        stack.push_int(1)?;
    } else {
        stack.push_int(0)?;
    }
    Ok(Continue)
}

#[inline]
fn is_instanceof(object: &Reference, class_name: &str) -> Result<bool> {
    match object {
        Reference::ByteArray(_)
        | Reference::CharArray(_)
        | Reference::ShortArray(_)
        | Reference::IntArray(_)
        | Reference::LongArray(_)
        | Reference::FloatArray(_)
        | Reference::DoubleArray(_) => {
            let reference_class_name = object.class_name();
            Ok(reference_class_name == class_name)
        }
        Reference::Array(array_class, _) => {
            let reference_class_name = object.class_name();
            let reference_array_dimensions =
                reference_class_name.chars().filter(|&c| c == '[').count();
            let class_array_depth = class_name.chars().filter(|&c| c == '[').count();
            if class_array_depth > 0 && class_array_depth != reference_array_dimensions {
                return Ok(false);
            }
            // Convert an array class name (e.g. [Ljava/lang/String;) into the base class name by
            // remove the leading '[' and 'L' and trailing ';'
            let class_name = class_name
                .trim_start_matches('[')
                .strip_prefix("L")
                .unwrap_or(class_name)
                .strip_suffix(";")
                .unwrap_or(class_name);

            Ok(array_class.is_assignable_from(class_name)?)
        }
        Reference::Object(object) => Ok(object.instanceof(class_name)?),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error::InvalidOperand;
    use ristretto_classloader::ConcurrentVec;
    use std::sync::Arc;

    #[test]
    fn test_aconst_null() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        let result = aconst_null(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(Value::Object(None), stack.pop()?);
        Ok(())
    }

    #[test]
    fn test_aload() -> Result<()> {
        let locals = LocalVariables::with_max_size(1);
        locals.set_object(0, None)?;
        let stack = &mut OperandStack::with_max_size(1);
        let result = aload(&locals, stack, 0)?;
        assert_eq!(Continue, result);
        assert_eq!(None, stack.pop_object()?);
        Ok(())
    }

    #[test]
    fn test_aload_w() -> Result<()> {
        let locals = LocalVariables::with_max_size(1);
        locals.set_object(0, None)?;
        let stack = &mut OperandStack::with_max_size(1);
        let result = aload_w(&locals, stack, 0)?;
        assert_eq!(Continue, result);
        assert_eq!(None, stack.pop_object()?);
        Ok(())
    }

    #[test]
    fn test_aload_0() -> Result<()> {
        let locals = LocalVariables::with_max_size(1);
        locals.set_object(0, None)?;
        let stack = &mut OperandStack::with_max_size(1);
        let result = aload_0(&locals, stack)?;
        assert_eq!(Continue, result);
        assert_eq!(None, stack.pop_object()?);
        Ok(())
    }

    #[test]
    fn test_aload_1() -> Result<()> {
        let locals = LocalVariables::with_max_size(2);
        locals.set_object(1, None)?;
        let stack = &mut OperandStack::with_max_size(1);
        let result = aload_1(&locals, stack)?;
        assert_eq!(Continue, result);
        assert_eq!(None, stack.pop_object()?);
        Ok(())
    }

    #[test]
    fn test_aload_2() -> Result<()> {
        let locals = LocalVariables::with_max_size(3);
        locals.set_object(2, None)?;
        let stack = &mut OperandStack::with_max_size(1);
        let result = aload_2(&locals, stack)?;
        assert_eq!(Continue, result);
        assert_eq!(None, stack.pop_object()?);
        Ok(())
    }

    #[test]
    fn test_aload_3() -> Result<()> {
        let locals = LocalVariables::with_max_size(4);
        locals.set_object(3, None)?;
        let stack = &mut OperandStack::with_max_size(1);
        let result = aload_3(&locals, stack)?;
        assert_eq!(Continue, result);
        assert_eq!(None, stack.pop_object()?);
        Ok(())
    }

    #[test]
    fn test_astore() -> Result<()> {
        let locals = &LocalVariables::with_max_size(1);
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_object(None)?;
        let result = astore(locals, stack, 0)?;
        assert_eq!(Continue, result);
        assert_eq!(None, locals.get_object(0)?);
        Ok(())
    }

    #[test]
    fn test_astore_w() -> Result<()> {
        let locals = &LocalVariables::with_max_size(1);
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_object(None)?;
        let result = astore_w(locals, stack, 0)?;
        assert_eq!(Continue, result);
        assert_eq!(None, locals.get_object(0)?);
        Ok(())
    }

    #[test]
    fn test_astore_0() -> Result<()> {
        let locals = &LocalVariables::with_max_size(1);
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_object(None)?;
        let result = astore_0(locals, stack)?;
        assert_eq!(Continue, result);
        assert_eq!(None, locals.get_object(0)?);
        Ok(())
    }

    #[test]
    fn test_astore_1() -> Result<()> {
        let locals = &LocalVariables::with_max_size(2);
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_object(None)?;
        let result = astore_1(locals, stack)?;
        assert_eq!(Continue, result);
        assert_eq!(None, locals.get_object(1)?);
        Ok(())
    }

    #[test]
    fn test_astore_2() -> Result<()> {
        let locals = &LocalVariables::with_max_size(3);
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_object(None)?;
        let result = astore_2(locals, stack)?;
        assert_eq!(Continue, result);
        assert_eq!(None, locals.get_object(2)?);
        Ok(())
    }

    #[test]
    fn test_astore_3() -> Result<()> {
        let locals = &LocalVariables::with_max_size(4);
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_object(None)?;
        let result = astore_3(locals, stack)?;
        assert_eq!(Continue, result);
        assert_eq!(None, locals.get_object(3)?);
        Ok(())
    }

    #[tokio::test]
    async fn test_aaload() -> Result<()> {
        let (vm, call_stack, frame) = crate::test::frame().await?;
        let stack = frame.stack();
        let class = vm.class(&call_stack, "java/lang/Object").await?;
        let object = Reference::IntArray(ConcurrentVec::from(vec![42]));
        let array = Reference::Array(class, ConcurrentVec::from(vec![Some(object.clone())]));
        stack.push_object(Some(array))?;
        stack.push_int(0)?;
        let result = aaload(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(Some(object), stack.pop_object()?);
        Ok(())
    }

    #[test]
    fn test_aaload_invalid_value() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        let object = Reference::IntArray(ConcurrentVec::from(vec![42]));
        stack.push_object(Some(object))?;
        stack.push_int(2)?;
        let result = aaload(stack);
        assert!(matches!(
            result,
            Err(InvalidStackValue {
                expected,
                actual
            }) if expected == "reference array" && actual == "int[42]"
        ));
        Ok(())
    }

    #[tokio::test]
    async fn test_aaload_invalid_index() -> Result<()> {
        let (vm, call_stack, frame) = crate::test::frame().await?;
        let stack = frame.stack();
        let class = vm.class(&call_stack, "java/lang/Object").await?;
        let object = Reference::IntArray(ConcurrentVec::from(vec![42]));
        let array = Reference::Array(class, ConcurrentVec::from(vec![Some(object.clone())]));
        stack.push_object(Some(array))?;
        stack.push_int(2)?;
        let result = aaload(stack);
        assert!(matches!(result, Err(ArrayIndexOutOfBounds(2))));
        Ok(())
    }

    #[test]
    fn test_aaload_null_pointer() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_object(None)?;
        stack.push_int(0)?;
        let result = aaload(stack);
        assert!(matches!(result, Err(NullPointer)));
        Ok(())
    }

    #[tokio::test]
    async fn test_aastore() -> Result<()> {
        let (vm, call_stack, frame) = crate::test::frame().await?;
        let stack = frame.stack();
        let class = vm.class(&call_stack, "java/lang/Object").await?;
        let object = Reference::IntArray(ConcurrentVec::from(vec![3]));
        let array = Reference::Array(class, ConcurrentVec::from(vec![Some(object)]));
        stack.push_object(Some(array))?;
        stack.push_int(0)?;
        stack.push_object(Some(Reference::IntArray(ConcurrentVec::from(vec![3]))))?;
        let result = aastore(stack)?;
        assert_eq!(Continue, result);
        Ok(())
    }

    #[test]
    fn test_aastore_invalid_value() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(3);
        let object = Reference::IntArray(ConcurrentVec::from(vec![42]));
        stack.push_object(Some(object.clone()))?;
        stack.push_int(0)?;
        stack.push_object(Some(object))?;
        let result = aastore(stack);
        assert!(matches!(
            result,
            Err(InvalidStackValue {
                expected,
                actual
            }) if expected == "reference array" && actual == "int[42]"
        ));
        Ok(())
    }

    #[tokio::test]
    async fn test_aastore_invalid_index() -> Result<()> {
        let (vm, call_stack, frame) = crate::test::frame().await?;
        let stack = frame.stack();
        let class = vm.class(&call_stack, "java/lang/Object").await?;
        let object = Reference::IntArray(ConcurrentVec::from(vec![3]));
        let array = Reference::Array(class, ConcurrentVec::from(vec![Some(object.clone())]));
        stack.push_object(Some(array))?;
        stack.push_int(2)?;
        stack.push_object(Some(object))?;
        let result = aastore(stack);
        assert!(matches!(result, Err(ArrayIndexOutOfBounds(2))));
        Ok(())
    }

    #[test]
    fn test_aastore_null_pointer() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(3);
        let object = Reference::IntArray(ConcurrentVec::from(vec![3]));
        stack.push_object(None)?;
        stack.push_int(0)?;
        stack.push_object(Some(object))?;
        let result = aastore(stack);
        assert!(matches!(result, Err(NullPointer)));
        Ok(())
    }

    #[test]
    fn test_areturn_object() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        let object = Reference::ByteArray(ConcurrentVec::from(vec![42]));
        stack.push_object(Some(object))?;
        let result = areturn(stack)?;
        assert!(matches!(result, Return(Some(Value::Object(_)))));
        Ok(())
    }

    #[test]
    fn test_areturn_null() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_object(None)?;
        let result = areturn(stack)?;
        assert!(matches!(result, Return(Some(Value::Object(None)))));
        Ok(())
    }

    #[test]
    fn test_areturn_invalid_type() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_int(42)?;
        let result = areturn(stack);
        assert!(matches!(
            result,
            Err(InvalidOperand {
                expected,
                actual
            }) if expected == "object" && actual == "int(42)"
        ));
        Ok(())
    }

    #[tokio::test]
    async fn test_new() -> Result<()> {
        let (_vm, _call_stack, mut frame) = crate::test::frame().await?;
        let class = frame.class_mut();
        let constant_pool = Arc::get_mut(class).expect("class").constant_pool_mut();
        let class_index = constant_pool.add_class("Child")?;
        let process_result = new(&frame, class_index).await?;
        assert_eq!(process_result, Continue);
        let stack = frame.stack();
        let object = stack.pop()?;
        assert!(matches!(object, Value::Object(Some(Reference::Object(_)))));
        Ok(())
    }

    #[tokio::test]
    async fn test_checkcast_null() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_object(None)?;
        let result = checkcast(stack, "java/lang/Object")?;
        assert_eq!(Continue, result);
        Ok(())
    }

    #[tokio::test]
    async fn test_checkcast_string_to_object() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        let (vm, _call_stack, object_class) = crate::test::load_class("java/lang/Object").await?;
        let string = vm.string("foo").await?;
        stack.push(string)?;
        let result = checkcast(stack, object_class.name())?;
        assert_eq!(Continue, result);
        Ok(())
    }

    #[tokio::test]
    async fn test_checkcast_object_to_string() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        let (_vm, _call_stack, object_class) = crate::test::load_class("java/lang/Object").await?;
        let (_vm, _call_stack, string_class) = crate::test::load_class("java/lang/String").await?;
        let object = Object::new(object_class)?;
        stack.push_object(Some(Reference::Object(object)))?;
        let result = checkcast(stack, string_class.name());
        assert!(matches!(
            result,
            Err(ClassCastError(class)) if class == "java/lang/String"
        ));
        Ok(())
    }

    #[tokio::test]
    async fn test_instanceof_null() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        let (_vm, _call_stack, object_class) = crate::test::load_class("java/lang/Object").await?;
        stack.push_object(None)?;
        let result = instanceof(stack, object_class.name())?;
        assert_eq!(Continue, result);
        assert_eq!(0, stack.pop_int()?);
        Ok(())
    }

    #[tokio::test]
    async fn test_instanceof_string_to_object() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        let (vm, _call_stack, object_class) = crate::test::load_class("java/lang/Object").await?;
        let string = vm.string("foo").await?;
        stack.push(string)?;
        let result = instanceof(stack, object_class.name())?;
        assert_eq!(Continue, result);
        assert_eq!(1, stack.pop_int()?);
        Ok(())
    }

    #[tokio::test]
    async fn test_instanceof_object_to_string() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        let (_vm, _call_stack, object_class) = crate::test::load_class("java/lang/Object").await?;
        let (_vm, _call_stack, string_class) = crate::test::load_class("java/lang/String").await?;
        let object = Object::new(object_class)?;
        stack.push_object(Some(Reference::Object(object)))?;
        let result = instanceof(stack, string_class.name())?;
        assert_eq!(Continue, result);
        assert_eq!(0, stack.pop_int()?);
        Ok(())
    }

    #[tokio::test]
    async fn test_instanceof_string_array_to_object() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        let (_vm, _call_stack, object_class) = crate::test::load_class("java/lang/Object").await?;
        let (_vm, _call_stack, string_class) = crate::test::load_class("java/lang/String").await?;
        let string_array = Reference::Array(string_class, ConcurrentVec::default());
        stack.push_object(Some(string_array))?;
        let result = instanceof(stack, object_class.name())?;
        assert_eq!(Continue, result);
        assert_eq!(1, stack.pop_int()?);
        Ok(())
    }

    #[tokio::test]
    async fn test_instanceof_object_array_to_string() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        let (_vm, _call_stack, object_class) = crate::test::load_class("java/lang/Object").await?;
        let (_vm, _call_stack, string_class) = crate::test::load_class("java/lang/String").await?;
        let object_array = Reference::Array(object_class, ConcurrentVec::default());
        stack.push_object(Some(object_array))?;
        let result = instanceof(stack, string_class.name())?;
        assert_eq!(Continue, result);
        assert_eq!(0, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_instanceof_int_array_to_int_array() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        let int_array = Reference::IntArray(ConcurrentVec::default());
        let int_array_class = int_array.class()?;
        stack.push_object(Some(int_array))?;
        let result = instanceof(stack, int_array_class.name())?;
        assert_eq!(Continue, result);
        assert_eq!(1, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_instanceof_long_array_to_int_array() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        let long_array = Reference::LongArray(ConcurrentVec::default());
        let int_array = Reference::IntArray(ConcurrentVec::default());
        let int_array_class = int_array.class()?;
        stack.push_object(Some(long_array))?;
        let result = instanceof(stack, int_array_class.name())?;
        assert_eq!(Continue, result);
        assert_eq!(0, stack.pop_int()?);
        Ok(())
    }
}
