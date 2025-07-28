use crate::Error::InvalidStackValue;
use crate::JavaError::{ArrayIndexOutOfBoundsException, NullPointerException};
use crate::Result;
use crate::frame::ExecutionResult;
use crate::frame::ExecutionResult::Continue;
use crate::operand_stack::OperandStack;
use ristretto_classloader::Reference;

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.baload>
#[inline]
pub(crate) fn baload(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let index = stack.pop_int()?;
    match stack.pop_object()? {
        None => Err(NullPointerException("array cannot be null".to_string()).into()),
        Some(Reference::ByteArray(array)) => {
            let original_index = index;
            let length = array.len()?;
            let index = usize::try_from(index).map_err(|_| ArrayIndexOutOfBoundsException {
                index: original_index,
                length,
            })?;
            let Some(value) = array.get(index)? else {
                return Err(ArrayIndexOutOfBoundsException {
                    index: original_index,
                    length,
                }
                .into());
            };
            stack.push_int(i32::from(value))?;
            Ok(Continue)
        }
        Some(object) => Err(InvalidStackValue {
            expected: "byte array".to_string(),
            actual: object.to_string(),
        }),
    }
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.bastore>
#[inline]
pub(crate) fn bastore(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let value = stack.pop_int()?;
    let index = stack.pop_int()?;
    match stack.pop_object()? {
        None => Err(NullPointerException("array cannot be null".to_string()).into()),
        Some(Reference::ByteArray(ref mut array)) => {
            let length = array.capacity()?;
            let original_index = index;
            let index = usize::try_from(index).map_err(|_| ArrayIndexOutOfBoundsException {
                index: original_index,
                length,
            })?;
            if index >= length {
                return Err(ArrayIndexOutOfBoundsException {
                    index: original_index,
                    length,
                }
                .into());
            }
            array.set(index, i8::try_from(value)?)?;
            Ok(Continue)
        }
        Some(object) => Err(InvalidStackValue {
            expected: "byte array".to_string(),
            actual: object.to_string(),
        }),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Error::JavaError;
    use ristretto_classloader::Value;

    #[test]
    fn test_baload() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        let array = Value::from(vec![42i8]);
        stack.push(array)?;
        stack.push_int(0)?;
        let result = baload(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(42, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_baload_invalid_value() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        let object = Value::from(vec![42i32]);
        stack.push(object)?;
        stack.push_int(2)?;
        let result = baload(stack);
        assert!(matches!(
            result,
            Err(InvalidStackValue {
                expected,
                actual
            }) if expected == "byte array" && actual == "int[42]"
        ));
        Ok(())
    }

    #[test]
    fn test_baload_invalid_index_negative() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        let array = Value::from(vec![42i8]);
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

    #[test]
    fn test_baload_invalid_index() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        let array = Value::from(vec![42i8]);
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
        assert!(matches!(result, Err(JavaError(NullPointerException(_)))));
        Ok(())
    }

    #[test]
    fn test_bastore() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(3);
        let array = Value::from(vec![3i8]);
        stack.push(array)?;
        stack.push_int(0)?;
        stack.push_int(42)?;
        let result = bastore(stack)?;
        assert_eq!(Continue, result);
        Ok(())
    }

    #[test]
    fn test_bastore_invalid_value() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(3);
        let object = Value::from(vec![42i32]);
        stack.push(object)?;
        stack.push_int(2)?;
        stack.push_int(42)?;
        let result = bastore(stack);
        assert!(matches!(
            result,
            Err(InvalidStackValue {
                expected,
                actual
            }) if expected == "byte array" && actual == "int[42]"
        ));
        Ok(())
    }

    #[test]
    fn test_bastore_invalid_index_negative() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(3);
        let array = Value::from(vec![3i8]);
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

    #[test]
    fn test_bastore_invalid_index() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(3);
        let array = Value::from(vec![3i8]);
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
        assert!(matches!(result, Err(JavaError(NullPointerException(_)))));
        Ok(())
    }
}
