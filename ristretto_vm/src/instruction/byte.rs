use crate::Error::InvalidStackValue;
use crate::JavaError::{ArrayIndexOutOfBoundsException, NullPointerException};
use crate::Result;
use crate::frame::ExecutionResult;
use crate::frame::ExecutionResult::Continue;
use crate::operand_stack::OperandStack;
use ristretto_classloader::Reference;

/// # References
///
/// - [JVMS §6.5.baload](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.baload)
#[inline]
pub(crate) fn baload(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let index = stack.pop_int()?;
    let Some(reference) = stack.pop_object()? else {
        return Err(NullPointerException(None).into());
    };
    let guard = reference.read();
    let (Reference::BooleanArray(array) | Reference::ByteArray(array)) = &*guard else {
        return Err(InvalidStackValue {
            expected: "byte or boolean array".to_string(),
            actual: guard.to_string(),
        });
    };

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
    stack.push_int(i32::from(*value))?;
    Ok(Continue)
}

/// # References
///
/// - [JVMS §6.5.bastore](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.bastore)
#[inline]
pub(crate) fn bastore(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let value = stack.pop_int()?;
    let index = stack.pop_int()?;
    let Some(reference) = stack.pop_object()? else {
        return Err(NullPointerException(None).into());
    };
    let mut guard = reference.write();
    let (Reference::BooleanArray(array) | Reference::ByteArray(array)) = &mut *guard else {
        return Err(InvalidStackValue {
            expected: "byte or boolean array".to_string(),
            actual: guard.to_string(),
        });
    };

    let length = array.len();
    let original_index = index;
    let index = usize::try_from(index).map_err(|_| ArrayIndexOutOfBoundsException {
        index: original_index,
        length,
    })?;
    if let Some(element) = array.get_mut(index) {
        *element = i8::try_from(value)?;
    } else {
        return Err(ArrayIndexOutOfBoundsException {
            index: original_index,
            length,
        }
        .into());
    }
    Ok(Continue)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Error::JavaError;
    use ristretto_classloader::Value;

    #[tokio::test]
    async fn test_baload() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let stack = &mut OperandStack::with_max_size(2);
        let reference = Reference::from(vec![42i8]);
        let array = Value::new_object(thread.vm()?.garbage_collector(), reference);
        stack.push(array)?;
        stack.push_int(0)?;
        let result = baload(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(42, stack.pop_int()?);
        Ok(())
    }

    #[tokio::test]
    async fn test_baload_invalid_value() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let stack = &mut OperandStack::with_max_size(2);
        let reference = Reference::from(vec![42i32]);
        let object = Value::new_object(thread.vm()?.garbage_collector(), reference);
        stack.push(object)?;
        stack.push_int(2)?;
        let result = baload(stack);
        assert!(matches!(
            result,
            Err(InvalidStackValue {
                expected,
                actual
            }) if expected == "byte or boolean array" && actual == "int[42]"
        ));
        Ok(())
    }

    #[tokio::test]
    async fn test_baload_invalid_index_negative() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let stack = &mut OperandStack::with_max_size(2);
        let reference = Reference::from(vec![42i8]);
        let array = Value::new_object(thread.vm()?.garbage_collector(), reference);
        stack.push(array)?;
        stack.push_int(-1)?;
        let result = baload(stack);
        assert!(matches!(
            result,
            Err(JavaError(ArrayIndexOutOfBoundsException{ index, length }))
            if index == -1 && length == 1
        ));
        Ok(())
    }

    #[tokio::test]
    async fn test_baload_invalid_index() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let stack = &mut OperandStack::with_max_size(2);
        let reference = Reference::from(vec![42i8]);
        let array = Value::new_object(thread.vm()?.garbage_collector(), reference);
        stack.push(array)?;
        stack.push_int(2)?;
        let result = baload(stack);
        assert!(matches!(
            result,
            Err(JavaError(ArrayIndexOutOfBoundsException{ index, length }))
            if index == 2 && length == 1
        ));
        Ok(())
    }

    #[test]
    fn test_baload_null_pointer() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_object(None)?;
        stack.push_int(0)?;
        let result = baload(stack);
        assert!(matches!(result, Err(JavaError(NullPointerException(None)))));
        Ok(())
    }

    #[tokio::test]
    async fn test_bastore() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let stack = &mut OperandStack::with_max_size(3);
        let reference = Reference::from(vec![3i8]);
        let array = Value::new_object(thread.vm()?.garbage_collector(), reference);
        stack.push(array)?;
        stack.push_int(0)?;
        stack.push_int(42)?;
        let result = bastore(stack)?;
        assert_eq!(Continue, result);
        Ok(())
    }

    #[tokio::test]
    async fn test_bastore_invalid_value() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let stack = &mut OperandStack::with_max_size(3);
        let reference = Reference::from(vec![42i32]);
        let object = Value::new_object(thread.vm()?.garbage_collector(), reference);
        stack.push(object)?;
        stack.push_int(2)?;
        stack.push_int(42)?;
        let result = bastore(stack);
        assert!(matches!(
            result,
            Err(InvalidStackValue {
                expected,
                actual
            }) if expected == "byte or boolean array" && actual == "int[42]"
        ));
        Ok(())
    }

    #[tokio::test]
    async fn test_bastore_invalid_index_negative() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let stack = &mut OperandStack::with_max_size(3);
        let reference = Reference::from(vec![3i8]);
        let array = Value::new_object(thread.vm()?.garbage_collector(), reference);
        stack.push(array)?;
        stack.push_int(-1)?;
        stack.push_int(42)?;
        let result = bastore(stack);
        assert!(matches!(
            result,
            Err(JavaError(ArrayIndexOutOfBoundsException{ index, length }))
            if index == -1 && length == 1
        ));
        Ok(())
    }

    #[tokio::test]
    async fn test_bastore_invalid_index() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let stack = &mut OperandStack::with_max_size(3);
        let reference = Reference::from(vec![3i8]);
        let array = Value::new_object(thread.vm()?.garbage_collector(), reference);
        stack.push(array)?;
        stack.push_int(2)?;
        stack.push_int(42)?;
        let result = bastore(stack);
        assert!(matches!(
            result,
            Err(JavaError(ArrayIndexOutOfBoundsException{ index, length }))
            if index == 2 && length == 1
        ));
        Ok(())
    }

    #[test]
    fn test_bastore_null_pointer() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(3);
        stack.push_object(None)?;
        stack.push_int(0)?;
        stack.push_int(42)?;
        let result = bastore(stack);
        assert!(matches!(result, Err(JavaError(NullPointerException(None)))));
        Ok(())
    }
}
