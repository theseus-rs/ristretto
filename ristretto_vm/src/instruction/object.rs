use crate::Error::{InternalError, InvalidStackValue, PoisonedLock};
use crate::JavaError::{ArrayIndexOutOfBoundsException, ClassCastException, NullPointerException};
use crate::assignable::Assignable;
use crate::frame::ExecutionResult::Return;
use crate::frame::{ExecutionResult, ExecutionResult::Continue, Frame};
use crate::local_variables::LocalVariables;
use crate::operand_stack::OperandStack;
use crate::thread::Thread;
use crate::{Result, Value};
use ristretto_classloader::{Class, Object, Reference};
use std::sync::Arc;

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.aconst_null>
#[inline]
pub(crate) fn aconst_null(stack: &mut OperandStack) -> Result<ExecutionResult> {
    stack.push_object(None)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.aload>
#[inline]
pub(crate) fn aload(
    locals: &LocalVariables,
    stack: &mut OperandStack,
    index: u8,
) -> Result<ExecutionResult> {
    let object = locals.get_object(usize::from(index))?;
    stack.push_object(object)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.aload>
/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.wide>
#[inline]
pub(crate) fn aload_w(
    locals: &LocalVariables,
    stack: &mut OperandStack,
    index: u16,
) -> Result<ExecutionResult> {
    let object = locals.get_object(usize::from(index))?;
    stack.push_object(object)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.aload_n>
#[inline]
pub(crate) fn aload_0(
    locals: &LocalVariables,
    stack: &mut OperandStack,
) -> Result<ExecutionResult> {
    let object = locals.get_object(0)?;
    stack.push_object(object)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.aload_n>
#[inline]
pub(crate) fn aload_1(
    locals: &LocalVariables,
    stack: &mut OperandStack,
) -> Result<ExecutionResult> {
    let object = locals.get_object(1)?;
    stack.push_object(object)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.aload_n>
#[inline]
pub(crate) fn aload_2(
    locals: &LocalVariables,
    stack: &mut OperandStack,
) -> Result<ExecutionResult> {
    let object = locals.get_object(2)?;
    stack.push_object(object)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.aload_n>
#[inline]
pub(crate) fn aload_3(
    locals: &LocalVariables,
    stack: &mut OperandStack,
) -> Result<ExecutionResult> {
    let object = locals.get_object(3)?;
    stack.push_object(object)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.astore>
#[inline]
pub(crate) fn astore(
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
    index: u8,
) -> Result<ExecutionResult> {
    let value = stack.pop_object()?;
    locals.set_object(usize::from(index), value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.astore>
/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.wide>
#[inline]
pub(crate) fn astore_w(
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
    index: u16,
) -> Result<ExecutionResult> {
    let value = stack.pop_object()?;
    locals.set_object(usize::from(index), value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.astore_n>
#[inline]
pub(crate) fn astore_0(
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<ExecutionResult> {
    let value = stack.pop_object()?;
    locals.set_object(0, value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.astore_n>
#[inline]
pub(crate) fn astore_1(
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<ExecutionResult> {
    let value = stack.pop_object()?;
    locals.set_object(1, value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.astore_n>
#[inline]
pub(crate) fn astore_2(
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<ExecutionResult> {
    let value = stack.pop_object()?;
    locals.set_object(2, value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.astore_n>
#[inline]
pub(crate) fn astore_3(
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<ExecutionResult> {
    let value = stack.pop_object()?;
    locals.set_object(3, value)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.aaload>
#[inline]
pub(crate) fn aaload(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let index = stack.pop_int()?;
    match stack.pop_object()? {
        None => Err(NullPointerException("array cannot be null".to_string()).into()),
        Some(Reference::Array(object_array)) => {
            let array = object_array
                .elements
                .read()
                .map_err(|error| PoisonedLock(error.to_string()))?;
            let original_index = index;
            let length = array.len();
            let index = usize::try_from(index).map_err(|_| ArrayIndexOutOfBoundsException {
                index: original_index,
                length,
            })?;
            let Some(value) = array.get(index) else {
                return Err(ArrayIndexOutOfBoundsException {
                    index: original_index,
                    length,
                }
                .into());
            };
            stack.push_object(value.clone())?;
            Ok(Continue)
        }
        Some(object) => Err(InvalidStackValue {
            expected: "reference array".to_string(),
            actual: object.to_string(),
        }),
    }
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.aastore>
#[inline]
pub(crate) fn aastore(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let value = stack.pop_object()?;
    let index = stack.pop_int()?;
    match stack.pop_object()? {
        None => Err(NullPointerException("array cannot be null".to_string()).into()),
        Some(Reference::Array(object_array)) => {
            let mut array = object_array
                .elements
                .write()
                .map_err(|error| PoisonedLock(error.to_string()))?;
            let length = array.capacity();
            let original_index = index;
            let index = usize::try_from(index).map_err(|_| ArrayIndexOutOfBoundsException {
                index: original_index,
                length,
            })?;
            // TODO: validate object type is compatible with array type
            // See: https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.aastore
            if let Some(element) = array.get_mut(index) {
                *element = value;
            } else {
                return Err(ArrayIndexOutOfBoundsException {
                    index: original_index,
                    length,
                }
                .into());
            }
            Ok(Continue)
        }
        Some(object) => Err(InvalidStackValue {
            expected: "reference array".to_string(),
            actual: object.to_string(),
        }),
    }
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.areturn>
#[inline]
pub(crate) fn areturn(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let value = stack.pop_object()?;
    Ok(Return(Some(Value::Object(value))))
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.new>
#[inline]
pub(crate) async fn new(
    frame: &Frame,
    stack: &mut OperandStack,
    index: u16,
) -> Result<ExecutionResult> {
    let thread = frame.thread()?;
    let constant_pool = frame.class().constant_pool();
    let class_name = constant_pool.try_get_class(index)?;
    let class = thread.class(class_name).await?;
    let object = Object::new(class)?;
    let reference = Value::from(object);
    stack.push(reference)?;
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.checkcast>
#[inline]
pub(crate) async fn checkcast(
    frame: &Frame,
    stack: &mut OperandStack,
    class_index: u16,
) -> Result<ExecutionResult> {
    let Value::Object(object) = stack.peek()? else {
        return Err(InternalError("Expected object".to_string()));
    };
    let Some(object) = object else {
        return Ok(Continue);
    };

    let class = frame.class();
    let constant_pool = class.constant_pool();
    let class_name = constant_pool.try_get_class(class_index)?;
    let thread = frame.thread()?;
    let class = thread.class(class_name).await?;
    if !is_instance_of(&thread, &object, &class).await? {
        let source_class_name = object.class_name()?.replace('/', ".");
        let target_class_name = class_name.replace('/', ".");
        return Err(ClassCastException {
            source_class_name,
            target_class_name,
        }
        .into());
    }
    Ok(Continue)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.instanceof>
#[inline]
pub(crate) async fn instanceof(
    frame: &Frame,
    stack: &mut OperandStack,
    class_index: u16,
) -> Result<ExecutionResult> {
    let object = stack.pop_object()?;
    let Some(object) = object else {
        stack.push_int(0)?;
        return Ok(Continue);
    };

    let class = frame.class();
    let constant_pool = class.constant_pool();
    let class_name = constant_pool.try_get_class(class_index)?;
    let thread = frame.thread()?;
    let class = thread.class(class_name).await?;
    if is_instance_of(&thread, &object, &class).await? {
        stack.push_int(1)?;
    } else {
        stack.push_int(0)?;
    }
    Ok(Continue)
}

#[inline]
async fn is_instance_of(thread: &Thread, object: &Reference, class: &Arc<Class>) -> Result<bool> {
    match object {
        Reference::ByteArray(_)
        | Reference::CharArray(_)
        | Reference::ShortArray(_)
        | Reference::IntArray(_)
        | Reference::LongArray(_)
        | Reference::FloatArray(_)
        | Reference::DoubleArray(_) => {
            let reference_class_name = object.class_name()?;
            let object_class = thread.class(reference_class_name).await?;
            Ok(class.is_assignable_from(thread, &object_class).await?)
        }
        Reference::Array(object_array) => Ok(class
            .is_assignable_from(thread, &object_array.class)
            .await?),
        Reference::Object(object) => {
            let object_class = {
                let object = object
                    .read()
                    .map_err(|error| PoisonedLock(error.to_string()))?;
                object.class().clone()
            };
            Ok(class.is_assignable_from(thread, &object_class).await?)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error::{InvalidOperand, JavaError};
    use crate::JavaError::NullPointerException;
    use crate::java_object::JavaObject;
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
        let mut locals = LocalVariables::with_max_size(1);
        locals.set_object(0, None)?;
        let stack = &mut OperandStack::with_max_size(1);
        let result = aload(&locals, stack, 0)?;
        assert_eq!(Continue, result);
        assert_eq!(None, stack.pop_object()?);
        Ok(())
    }

    #[test]
    fn test_aload_w() -> Result<()> {
        let mut locals = LocalVariables::with_max_size(1);
        locals.set_object(0, None)?;
        let stack = &mut OperandStack::with_max_size(1);
        let result = aload_w(&locals, stack, 0)?;
        assert_eq!(Continue, result);
        assert_eq!(None, stack.pop_object()?);
        Ok(())
    }

    #[test]
    fn test_aload_0() -> Result<()> {
        let mut locals = LocalVariables::with_max_size(1);
        locals.set_object(0, None)?;
        let stack = &mut OperandStack::with_max_size(1);
        let result = aload_0(&locals, stack)?;
        assert_eq!(Continue, result);
        assert_eq!(None, stack.pop_object()?);
        Ok(())
    }

    #[test]
    fn test_aload_1() -> Result<()> {
        let mut locals = LocalVariables::with_max_size(2);
        locals.set_object(1, None)?;
        let stack = &mut OperandStack::with_max_size(1);
        let result = aload_1(&locals, stack)?;
        assert_eq!(Continue, result);
        assert_eq!(None, stack.pop_object()?);
        Ok(())
    }

    #[test]
    fn test_aload_2() -> Result<()> {
        let mut locals = LocalVariables::with_max_size(3);
        locals.set_object(2, None)?;
        let stack = &mut OperandStack::with_max_size(1);
        let result = aload_2(&locals, stack)?;
        assert_eq!(Continue, result);
        assert_eq!(None, stack.pop_object()?);
        Ok(())
    }

    #[test]
    fn test_aload_3() -> Result<()> {
        let mut locals = LocalVariables::with_max_size(4);
        locals.set_object(3, None)?;
        let stack = &mut OperandStack::with_max_size(1);
        let result = aload_3(&locals, stack)?;
        assert_eq!(Continue, result);
        assert_eq!(None, stack.pop_object()?);
        Ok(())
    }

    #[test]
    fn test_astore() -> Result<()> {
        let locals = &mut LocalVariables::with_max_size(1);
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_object(None)?;
        let result = astore(locals, stack, 0)?;
        assert_eq!(Continue, result);
        assert_eq!(None, locals.get_object(0)?);
        Ok(())
    }

    #[test]
    fn test_astore_w() -> Result<()> {
        let locals = &mut LocalVariables::with_max_size(1);
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_object(None)?;
        let result = astore_w(locals, stack, 0)?;
        assert_eq!(Continue, result);
        assert_eq!(None, locals.get_object(0)?);
        Ok(())
    }

    #[test]
    fn test_astore_0() -> Result<()> {
        let locals = &mut LocalVariables::with_max_size(1);
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_object(None)?;
        let result = astore_0(locals, stack)?;
        assert_eq!(Continue, result);
        assert_eq!(None, locals.get_object(0)?);
        Ok(())
    }

    #[test]
    fn test_astore_1() -> Result<()> {
        let locals = &mut LocalVariables::with_max_size(2);
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_object(None)?;
        let result = astore_1(locals, stack)?;
        assert_eq!(Continue, result);
        assert_eq!(None, locals.get_object(1)?);
        Ok(())
    }

    #[test]
    fn test_astore_2() -> Result<()> {
        let locals = &mut LocalVariables::with_max_size(3);
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_object(None)?;
        let result = astore_2(locals, stack)?;
        assert_eq!(Continue, result);
        assert_eq!(None, locals.get_object(2)?);
        Ok(())
    }

    #[test]
    fn test_astore_3() -> Result<()> {
        let locals = &mut LocalVariables::with_max_size(4);
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_object(None)?;
        let result = astore_3(locals, stack)?;
        assert_eq!(Continue, result);
        assert_eq!(None, locals.get_object(3)?);
        Ok(())
    }

    #[tokio::test]
    async fn test_aaload() -> Result<()> {
        let (_vm, thread, _frame) = crate::test::frame().await?;
        let stack = &mut OperandStack::with_max_size(2);
        let class = thread.class("java/lang/Object").await?;
        let object = Value::from(vec![42i32]);
        let array = Value::try_from((class, vec![object.clone()]))?;
        stack.push(array)?;
        stack.push_int(0)?;
        let result = aaload(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(object, stack.pop()?);
        Ok(())
    }

    #[test]
    fn test_aaload_invalid_value() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        let object = Value::from(vec![42i32]);
        stack.push(object)?;
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
    async fn test_aaload_invalid_index_negative() -> Result<()> {
        let (_vm, thread, _frame) = crate::test::frame().await?;
        let stack = &mut OperandStack::with_max_size(2);
        let class = thread.class("java/lang/Object").await?;
        let object = Value::from(vec![42i32]);
        let array = Value::try_from((class, vec![object.clone()]))?;
        stack.push(array)?;
        stack.push_int(-1)?;
        let result = aaload(stack);
        assert!(matches!(
            result,
            Err(JavaError(ArrayIndexOutOfBoundsException { index, length }))
            if index == -1 && length == 1
        ));
        Ok(())
    }

    #[tokio::test]
    async fn test_aaload_invalid_index() -> Result<()> {
        let (_vm, thread, _frame) = crate::test::frame().await?;
        let stack = &mut OperandStack::with_max_size(2);
        let class = thread.class("java/lang/Object").await?;
        let object = Value::from(vec![42i32]);
        let array = Value::try_from((class, vec![object.clone()]))?;
        stack.push(array)?;
        stack.push_int(2)?;
        let result = aaload(stack);
        assert!(matches!(
            result,
            Err(JavaError(ArrayIndexOutOfBoundsException { index, length }))
            if index == 2 && length == 1
        ));
        Ok(())
    }

    #[test]
    fn test_aaload_null_pointer() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_object(None)?;
        stack.push_int(0)?;
        let result = aaload(stack);
        assert!(matches!(result, Err(JavaError(NullPointerException(_)))));
        Ok(())
    }

    #[tokio::test]
    async fn test_aastore() -> Result<()> {
        let (_vm, thread, _frame) = crate::test::frame().await?;
        let stack = &mut OperandStack::with_max_size(3);
        let class = thread.class("java/lang/Object").await?;
        let object = Value::from(vec![3i32]);
        let array = Value::try_from((class, vec![object]))?;
        stack.push(array)?;
        stack.push_int(0)?;
        stack.push(Value::from(vec![3i32]))?;
        let result = aastore(stack)?;
        assert_eq!(Continue, result);
        Ok(())
    }

    #[test]
    fn test_aastore_invalid_value() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(3);
        let object = Value::from(vec![42i32]);
        stack.push(object.clone())?;
        stack.push_int(0)?;
        stack.push(object)?;
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
    async fn test_aastore_invalid_index_negative() -> Result<()> {
        let (_vm, thread, _frame) = crate::test::frame().await?;
        let stack = &mut OperandStack::with_max_size(3);
        let class = thread.class("java/lang/Object").await?;
        let object = Value::from(vec![3i32]);
        let array = Value::try_from((class, vec![object.clone()]))?;
        stack.push(array)?;
        stack.push_int(-1)?;
        stack.push(object)?;
        let result = aastore(stack);
        assert!(matches!(
            result,
            Err(JavaError(ArrayIndexOutOfBoundsException { index, length }))
            if index == -1 && length == 1
        ));
        Ok(())
    }

    #[tokio::test]
    async fn test_aastore_invalid_index() -> Result<()> {
        let (_vm, thread, _frame) = crate::test::frame().await?;
        let stack = &mut OperandStack::with_max_size(3);
        let class = thread.class("java/lang/Object").await?;
        let object = Value::from(vec![3i32]);
        let array = Value::try_from((class, vec![object.clone()]))?;
        stack.push(array)?;
        stack.push_int(2)?;
        stack.push(object)?;
        let result = aastore(stack);
        assert!(matches!(
            result,
            Err(JavaError(ArrayIndexOutOfBoundsException { index, length }))
            if index == 2 && length == 1
        ));
        Ok(())
    }

    #[test]
    fn test_aastore_null_pointer() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(3);
        let object = Value::from(vec![3i32]);
        stack.push_object(None)?;
        stack.push_int(0)?;
        stack.push(object)?;
        let result = aastore(stack);
        assert!(matches!(result, Err(JavaError(NullPointerException(_)))));
        Ok(())
    }

    #[test]
    fn test_areturn_object() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(1);
        let object = Value::from(vec![42i8]);
        stack.push(object)?;
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
        let (_vm, _thread, mut frame) = crate::test::frame().await?;
        let class = frame.class_mut();
        let constant_pool = Arc::get_mut(class).expect("class").constant_pool_mut();
        let class_index = constant_pool.add_class("Minimum")?;
        let stack = &mut OperandStack::with_max_size(1);
        let process_result = new(&frame, stack, class_index).await?;
        assert_eq!(process_result, Continue);
        let object = stack.pop()?;
        assert!(matches!(object, Value::Object(Some(Reference::Object(_)))));
        Ok(())
    }

    fn get_class_index(frame: &mut Frame, class_name: &str) -> Result<u16> {
        let class = frame.class_mut();
        let constant_pool = Arc::get_mut(class).expect("class").constant_pool_mut();
        let class_index = constant_pool.add_class(class_name)?;
        Ok(class_index)
    }

    #[tokio::test]
    async fn test_checkcast_null() -> Result<()> {
        let (_vm, _thread, mut frame) = crate::test::frame().await?;
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_object(None)?;
        let class_index = get_class_index(&mut frame, "java/lang/Object")?;
        let result = checkcast(&frame, stack, class_index).await?;
        assert_eq!(Continue, result);
        Ok(())
    }

    #[tokio::test]
    async fn test_checkcast_string_to_object() -> Result<()> {
        let (_vm, thread, mut frame) = crate::test::frame().await?;
        let stack = &mut OperandStack::with_max_size(1);
        let string = "foo".to_object(&thread).await?;
        stack.push(string)?;
        let class_index = get_class_index(&mut frame, "java/lang/Object")?;
        let result = checkcast(&frame, stack, class_index).await?;
        assert_eq!(Continue, result);
        Ok(())
    }

    #[tokio::test]
    async fn test_checkcast_object_to_string() -> Result<()> {
        let (_vm, thread, mut frame) = crate::test::frame().await?;
        let stack = &mut OperandStack::with_max_size(1);
        let object_class = thread.class("java/lang/Object").await?;
        let object = Object::new(object_class)?;
        stack.push(Value::from(object))?;
        let class_index = get_class_index(&mut frame, "java/lang/String")?;
        let result = checkcast(&frame, stack, class_index).await;
        assert!(matches!(
            result,
            Err(JavaError(ClassCastException { source_class_name, target_class_name}))
            if source_class_name == "java.lang.Object" && target_class_name == "java.lang.String"
        ));
        Ok(())
    }

    #[tokio::test]
    async fn test_checkcast_string_array_to_object_array() -> Result<()> {
        let (_vm, thread, mut frame) = crate::test::frame().await?;
        let stack = &mut OperandStack::with_max_size(1);
        let string_class = thread.class("[Ljava/lang/String;").await?;
        let string_array = Value::from((string_class, Vec::new()));
        stack.push(string_array)?;
        let class_index = get_class_index(&mut frame, "[Ljava/lang/Object;")?;
        let result = checkcast(&frame, stack, class_index).await?;
        assert_eq!(Continue, result);
        Ok(())
    }

    #[tokio::test]
    async fn test_instanceof_null() -> Result<()> {
        let (_vm, _thread, mut frame) = crate::test::frame().await?;
        let stack = &mut OperandStack::with_max_size(1);
        stack.push_object(None)?;
        let class_index = get_class_index(&mut frame, "java/lang/Object")?;
        let result = instanceof(&frame, stack, class_index).await?;
        assert_eq!(Continue, result);
        assert_eq!(0, stack.pop_int()?);
        Ok(())
    }

    #[tokio::test]
    async fn test_instanceof_string_to_object() -> Result<()> {
        let (_vm, thread, mut frame) = crate::test::frame().await?;
        let stack = &mut OperandStack::with_max_size(1);
        let string = "foo".to_object(&thread).await?;
        stack.push(string)?;
        let class_index = get_class_index(&mut frame, "java/lang/Object")?;
        let result = instanceof(&frame, stack, class_index).await?;
        assert_eq!(Continue, result);
        assert_eq!(1, stack.pop_int()?);
        Ok(())
    }

    #[tokio::test]
    async fn test_instanceof_object_to_string() -> Result<()> {
        let (_vm, thread, mut frame) = crate::test::frame().await?;
        let stack = &mut OperandStack::with_max_size(1);
        let object_class = thread.class("java/lang/Object").await?;
        let object = Object::new(object_class)?;
        stack.push(Value::from(object))?;
        let class_index = get_class_index(&mut frame, "java/lang/String")?;
        let result = instanceof(&frame, stack, class_index).await?;
        assert_eq!(Continue, result);
        assert_eq!(0, stack.pop_int()?);
        Ok(())
    }

    #[tokio::test]
    async fn test_instanceof_string_array_to_object() -> Result<()> {
        let (_vm, thread, mut frame) = crate::test::frame().await?;
        let stack = &mut OperandStack::with_max_size(1);
        let string_class = thread.class("[Ljava/lang/String;").await?;
        let string_array = Value::try_from((string_class, Vec::<Value>::new()))?;
        stack.push(string_array)?;
        let class_index = get_class_index(&mut frame, "java/lang/Object")?;
        let result = instanceof(&frame, stack, class_index).await?;
        assert_eq!(Continue, result);
        assert_eq!(1, stack.pop_int()?);
        Ok(())
    }

    #[tokio::test]
    async fn test_instanceof_object_array_to_string() -> Result<()> {
        let (_vm, thread, mut frame) = crate::test::frame().await?;
        let stack = &mut OperandStack::with_max_size(1);
        let object_class = thread.class("[Ljava/lang/Object;").await?;
        let object_array = Value::try_from((object_class, Vec::<Value>::new()))?;
        stack.push(object_array)?;
        let class_index = get_class_index(&mut frame, "java/lang/String")?;
        let result = instanceof(&frame, stack, class_index).await?;
        assert_eq!(Continue, result);
        assert_eq!(0, stack.pop_int()?);
        Ok(())
    }

    #[tokio::test]
    async fn test_instanceof_int_array_to_int_array() -> Result<()> {
        let (_vm, _thread, mut frame) = crate::test::frame().await?;
        let stack = &mut OperandStack::with_max_size(1);
        let int_array = Value::from(vec![0i32; 0]);
        stack.push(int_array)?;
        let class_index = get_class_index(&mut frame, "[I")?;
        let result = instanceof(&frame, stack, class_index).await?;
        assert_eq!(Continue, result);
        assert_eq!(1, stack.pop_int()?);
        Ok(())
    }

    #[tokio::test]
    async fn test_instanceof_long_array_to_int_array() -> Result<()> {
        let (_vm, _thread, mut frame) = crate::test::frame().await?;
        let stack = &mut OperandStack::with_max_size(1);
        let long_array = Value::from(Vec::<i64>::new());
        stack.push(long_array)?;
        let class_index = get_class_index(&mut frame, "[I")?;
        let result = instanceof(&frame, stack, class_index).await?;
        assert_eq!(Continue, result);
        assert_eq!(0, stack.pop_int()?);
        Ok(())
    }
}
