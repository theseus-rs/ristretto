use crate::Error::InvalidStackValue;
use crate::JavaError::{ArrayIndexOutOfBoundsException, NullPointerException};
use crate::Result;
use crate::frame::ExecutionResult;
use crate::frame::ExecutionResult::Continue;
use crate::operand_stack::OperandStack;
use ristretto_classloader::Reference;

/// # References
///
/// - [JVMS §6.5.saload](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.saload)
#[inline]
pub(crate) fn saload(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let index = stack.pop_int()?;
    let Some(reference) = stack.pop_object()? else {
        return Err(NullPointerException(None).into());
    };
    let guard = reference.read();
    let Reference::ShortArray(array) = &*guard else {
        return Err(InvalidStackValue {
            expected: "short array".to_string(),
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
/// - [JVMS §6.5.sastore](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.sastore)
#[inline]
pub(crate) fn sastore(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let value = stack.pop_int()?;
    let index = stack.pop_int()?;
    let Some(reference) = stack.pop_object()? else {
        return Err(NullPointerException(None).into());
    };
    let mut guard = reference.write();
    let Reference::ShortArray(array) = &mut *guard else {
        return Err(InvalidStackValue {
            expected: "short array".to_string(),
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
        *element = i16::try_from(value)?;
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
    async fn test_saload() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let stack = &mut OperandStack::with_max_size(2);
        let reference = Reference::from(vec![42i16]);
        let array = Value::new_object(thread.vm()?.garbage_collector(), reference);
        stack.push(array)?;
        stack.push_int(0)?;
        let result = saload(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(42, stack.pop_int()?);
        Ok(())
    }

    #[tokio::test]
    async fn test_saload_invalid_value() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let stack = &mut OperandStack::with_max_size(2);
        let reference = Reference::from(vec![42i32]);
        let object = Value::new_object(thread.vm()?.garbage_collector(), reference);
        stack.push(object)?;
        stack.push_int(2)?;
        let result = saload(stack);
        assert!(matches!(
            result,
            Err(InvalidStackValue {
                expected,
                actual
            }) if expected == "short array" && actual == "int[42]"
        ));
        Ok(())
    }

    #[tokio::test]
    async fn test_saload_invalid_index_negative() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let stack = &mut OperandStack::with_max_size(2);
        let reference = Reference::from(vec![42i16]);
        let array = Value::new_object(thread.vm()?.garbage_collector(), reference);
        stack.push(array)?;
        stack.push_int(-1)?;
        let result = saload(stack);
        assert!(matches!(
            result,
            Err(JavaError(ArrayIndexOutOfBoundsException { index, length }))
            if index == -1 && length == 1
        ));
        Ok(())
    }

    #[tokio::test]
    async fn test_saload_invalid_index() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let stack = &mut OperandStack::with_max_size(2);
        let reference = Reference::from(vec![42i16]);
        let array = Value::new_object(thread.vm()?.garbage_collector(), reference);
        stack.push(array)?;
        stack.push_int(2)?;
        let result = saload(stack);
        assert!(matches!(
            result,
            Err(JavaError(ArrayIndexOutOfBoundsException { index, length }))
            if index == 2 && length == 1
        ));
        Ok(())
    }

    #[test]
    fn test_saload_null_pointer() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_object(None)?;
        stack.push_int(0)?;
        let result = saload(stack);
        assert!(matches!(result, Err(JavaError(NullPointerException(None)))));
        Ok(())
    }

    #[tokio::test]
    async fn test_sastore() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let stack = &mut OperandStack::with_max_size(3);
        let reference = Reference::from(vec![3i16]);
        let array = Value::new_object(thread.vm()?.garbage_collector(), reference);
        stack.push(array)?;
        stack.push_int(0)?;
        stack.push_int(42)?;
        let result = sastore(stack)?;
        assert_eq!(Continue, result);
        Ok(())
    }

    #[tokio::test]
    async fn test_sastore_invalid_value() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let stack = &mut OperandStack::with_max_size(3);
        let reference = Reference::from(vec![42i32]);
        let object = Value::new_object(thread.vm()?.garbage_collector(), reference);
        stack.push(object)?;
        stack.push_int(2)?;
        stack.push_int(42)?;
        let result = sastore(stack);
        assert!(matches!(
            result,
            Err(InvalidStackValue {
                expected,
                actual
            }) if expected == "short array" && actual == "int[42]"
        ));
        Ok(())
    }

    #[tokio::test]
    async fn test_sastore_invalid_index_negative() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let stack = &mut OperandStack::with_max_size(3);
        let reference = Reference::from(vec![3i16]);
        let array = Value::new_object(thread.vm()?.garbage_collector(), reference);
        stack.push(array)?;
        stack.push_int(-1)?;
        stack.push_int(42)?;
        let result = sastore(stack);
        assert!(matches!(
            result,
            Err(JavaError(ArrayIndexOutOfBoundsException { index, length }))
            if index == -1 && length == 1
        ));
        Ok(())
    }

    #[tokio::test]
    async fn test_sastore_invalid_index() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let stack = &mut OperandStack::with_max_size(3);
        let reference = Reference::from(vec![3i16]);
        let array = Value::new_object(thread.vm()?.garbage_collector(), reference);
        stack.push(array)?;
        stack.push_int(2)?;
        stack.push_int(42)?;
        let result = sastore(stack);
        assert!(matches!(
            result,
            Err(JavaError(ArrayIndexOutOfBoundsException { index, length }))
            if index == 2 && length == 1
        ));
        Ok(())
    }

    #[test]
    fn test_sastore_null_pointer() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(3);
        stack.push_object(None)?;
        stack.push_int(0)?;
        stack.push_int(42)?;
        let result = sastore(stack);
        assert!(matches!(result, Err(JavaError(NullPointerException(None)))));
        Ok(())
    }
}
