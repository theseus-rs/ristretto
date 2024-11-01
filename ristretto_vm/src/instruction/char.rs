use crate::frame::ExecutionResult;
use crate::frame::ExecutionResult::Continue;
use crate::operand_stack::OperandStack;
use crate::Error::{ArrayIndexOutOfBounds, InvalidStackValue, NullPointer};
use crate::Result;
use ristretto_classloader::Reference;

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.caload>
#[inline]
pub(crate) fn caload(stack: &OperandStack) -> Result<ExecutionResult> {
    let index = stack.pop_int()?;
    match stack.pop_object()? {
        None => Err(NullPointer("array cannot be null".to_string())),
        Some(Reference::CharArray(array)) => {
            let index = usize::try_from(index)?;
            let Some(value) = array.get(index)? else {
                return Err(ArrayIndexOutOfBounds(index));
            };
            stack.push_int(i32::from(value))?;
            Ok(Continue)
        }
        Some(object) => Err(InvalidStackValue {
            expected: "char array".to_string(),
            actual: object.to_string(),
        }),
    }
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.castore>
#[inline]
pub(crate) fn castore(stack: &OperandStack) -> Result<ExecutionResult> {
    let value = stack.pop_int()?;
    let index = stack.pop_int()?;
    match stack.pop_object()? {
        None => Err(NullPointer("array cannot be null".to_string())),
        Some(Reference::CharArray(ref mut array)) => {
            let index = usize::try_from(index)?;
            if index >= array.capacity()? {
                return Err(ArrayIndexOutOfBounds(index));
            };
            array.set(index, u16::try_from(value)?)?;
            Ok(Continue)
        }
        Some(object) => Err(InvalidStackValue {
            expected: "char array".to_string(),
            actual: object.to_string(),
        }),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_caload() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        let array = Reference::from(vec![42 as char]);
        stack.push_object(Some(array))?;
        stack.push_int(0)?;
        let result = caload(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(42, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_caload_invalid_value() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        let object = Reference::from(vec![42i32]);
        stack.push_object(Some(object))?;
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
    fn test_caload_invalid_index() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        let array = Reference::from(vec![42 as char]);
        stack.push_object(Some(array))?;
        stack.push_int(2)?;
        let result = caload(stack);
        assert!(matches!(result, Err(ArrayIndexOutOfBounds(2))));
        Ok(())
    }

    #[test]
    fn test_caload_null_pointer() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_object(None)?;
        stack.push_int(0)?;
        let result = caload(stack);
        assert!(matches!(result, Err(NullPointer(_))));
        Ok(())
    }

    #[test]
    fn test_castore() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(3);
        let array = Reference::from(vec![3 as char]);
        stack.push_object(Some(array))?;
        stack.push_int(0)?;
        stack.push_int(42)?;
        let result = castore(stack)?;
        assert_eq!(Continue, result);
        Ok(())
    }

    #[test]
    fn test_castore_invalid_value() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(3);
        let object = Reference::from(vec![42i32]);
        stack.push_object(Some(object))?;
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
    fn test_castore_invalid_index() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(3);
        let array = Reference::from(vec![3 as char]);
        stack.push_object(Some(array))?;
        stack.push_int(2)?;
        stack.push_int(42)?;
        let result = castore(stack);
        assert!(matches!(result, Err(ArrayIndexOutOfBounds(2))));
        Ok(())
    }

    #[test]
    fn test_castore_null_pointer() -> Result<()> {
        let stack = &mut OperandStack::with_max_size(3);
        stack.push_object(None)?;
        stack.push_int(0)?;
        stack.push_int(42)?;
        let result = castore(stack);
        assert!(matches!(result, Err(NullPointer(_))));
        Ok(())
    }
}
