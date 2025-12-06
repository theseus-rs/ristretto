use crate::Error::InvalidStackValue;
use crate::JavaError::{ArrayIndexOutOfBoundsException, NullPointerException};
use crate::Result;
use crate::frame::ExecutionResult;
use crate::frame::ExecutionResult::Continue;
use crate::operand_stack::OperandStack;
use ristretto_classloader::Reference;

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.caload>
#[inline]
pub(crate) fn caload(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let index = stack.pop_int()?;
    let Some(reference) = stack.pop_object()? else {
        return Err(NullPointerException("array cannot be null".to_string()).into());
    };
    let guard = reference.read();
    let Reference::CharArray(array) = &*guard else {
        return Err(InvalidStackValue {
            expected: "char array".to_string(),
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

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.castore>
#[inline]
pub(crate) fn castore(stack: &mut OperandStack) -> Result<ExecutionResult> {
    let value = stack.pop_int()?;
    let index = stack.pop_int()?;
    let Some(reference) = stack.pop_object()? else {
        return Err(NullPointerException("array cannot be null".to_string()).into());
    };
    let mut guard = reference.write();
    let Reference::CharArray(array) = &mut *guard else {
        return Err(InvalidStackValue {
            expected: "char array".to_string(),
            actual: guard.to_string(),
        });
    };

    let length = array.capacity();
    let original_index = index;
    let index = usize::try_from(index).map_err(|_| ArrayIndexOutOfBoundsException {
        index: original_index,
        length,
    })?;
    if let Some(element) = array.get_mut(index) {
        *element = u16::try_from(value)?;
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

    #[test]
    fn test_caload() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        let array = Value::from(vec![42 as char]);
        stack.push(array)?;
        stack.push_int(0)?;
        let result = caload(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(42, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_caload_invalid_value() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        let object = Value::from(vec![42i32]);
        stack.push(object)?;
        stack.push_int(2)?;
        let result = caload(stack);
        assert!(matches!(
            result,
            Err(InvalidStackValue {
                expected,
                actual
            }) if expected == "char array" && actual == "int[42]"
        ));
        Ok(())
    }

    #[test]
    fn test_caload_invalid_index_negative() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        let array = Value::from(vec![42 as char]);
        stack.push(array)?;
        stack.push_int(-1)?;
        let result = caload(stack);
        assert!(matches!(
            result,
            Err(JavaError(ArrayIndexOutOfBoundsException { index, length }))
            if index == -1 && length == 1
        ));
        Ok(())
    }

    #[test]
    fn test_caload_invalid_index() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        let array = Value::from(vec![42 as char]);
        stack.push(array)?;
        stack.push_int(2)?;
        let result = caload(stack);
        assert!(matches!(
            result,
            Err(JavaError(ArrayIndexOutOfBoundsException { index, length }))
            if index == 2 && length == 1
        ));
        Ok(())
    }

    #[test]
    fn test_caload_null_pointer() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_object(None)?;
        stack.push_int(0)?;
        let result = caload(stack);
        assert!(matches!(result, Err(JavaError(NullPointerException(_)))));
        Ok(())
    }

    #[test]
    fn test_castore() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(3);
        let array = Value::from(vec![3 as char]);
        stack.push(array)?;
        stack.push_int(0)?;
        stack.push_int(42)?;
        let result = castore(stack)?;
        assert_eq!(Continue, result);
        Ok(())
    }

    #[test]
    fn test_castore_invalid_value() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(3);
        let object = Value::from(vec![42i32]);
        stack.push(object)?;
        stack.push_int(2)?;
        stack.push_int(42)?;
        let result = castore(stack);
        assert!(matches!(
            result,
            Err(InvalidStackValue {
                expected,
                actual
            }) if expected == "char array" && actual == "int[42]"
        ));
        Ok(())
    }

    #[test]
    fn test_castore_invalid_index_negative() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(3);
        let array = Value::from(vec![3 as char]);
        stack.push(array)?;
        stack.push_int(-1)?;
        stack.push_int(42)?;
        let result = castore(stack);
        assert!(matches!(
            result,
            Err(JavaError(ArrayIndexOutOfBoundsException{ index, length }))
            if index == -1 && length == 1
        ));
        Ok(())
    }

    #[test]
    fn test_castore_invalid_index() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(3);
        let array = Value::from(vec![3 as char]);
        stack.push(array)?;
        stack.push_int(2)?;
        stack.push_int(42)?;
        let result = castore(stack);
        assert!(matches!(
            result,
            Err(JavaError(ArrayIndexOutOfBoundsException{ index, length }))
            if index == 2 && length == 1
        ));
        Ok(())
    }

    #[test]
    fn test_castore_null_pointer() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(3);
        stack.push_object(None)?;
        stack.push_int(0)?;
        stack.push_int(42)?;
        let result = castore(stack);
        assert!(matches!(result, Err(JavaError(NullPointerException(_)))));
        Ok(())
    }
}
