use crate::frame::ExecutionResult;
use crate::frame::ExecutionResult::Continue;
use crate::operand_stack::OperandStack;
use crate::Error::{ArrayIndexOutOfBounds, InvalidStackValue, NullPointer};
use crate::Result;
use ristretto_classloader::Reference;

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.saload>
#[inline]
pub(crate) fn saload(stack: &OperandStack) -> Result<ExecutionResult> {
    let index = stack.pop_int()?;
    match stack.pop_object()? {
        None => Err(NullPointer),
        Some(Reference::ShortArray(array)) => {
            let index = usize::try_from(index)?;
            let Some(value) = array.get(index)? else {
                return Err(ArrayIndexOutOfBounds(index));
            };
            stack.push_int(i32::from(value))?;
            Ok(Continue)
        }
        Some(object) => Err(InvalidStackValue {
            expected: "short array".to_string(),
            actual: object.to_string(),
        }),
    }
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.sastore>
#[inline]
pub(crate) fn sastore(stack: &OperandStack) -> Result<ExecutionResult> {
    let value = stack.pop_int()?;
    let index = stack.pop_int()?;
    match stack.pop_object()? {
        None => Err(NullPointer),
        Some(Reference::ShortArray(ref mut array)) => {
            let index = usize::try_from(index)?;
            if index >= array.capacity()? {
                return Err(ArrayIndexOutOfBounds(index));
            };
            array.set(index, i16::try_from(value)?)?;
            Ok(Continue)
        }
        Some(object) => Err(InvalidStackValue {
            expected: "short array".to_string(),
            actual: object.to_string(),
        }),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ristretto_classloader::ConcurrentVec;

    #[test]
    fn test_saload() -> crate::Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        let array = Reference::ShortArray(ConcurrentVec::from(vec![42]));
        stack.push_object(Some(array))?;
        stack.push_int(0)?;
        let result = saload(stack)?;
        assert_eq!(Continue, result);
        assert_eq!(42, stack.pop_int()?);
        Ok(())
    }

    #[test]
    fn test_saload_invalid_value() -> crate::Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        let object = Reference::IntArray(ConcurrentVec::from(vec![42]));
        stack.push_object(Some(object))?;
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

    #[test]
    fn test_saload_invalid_index() -> crate::Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        let array = Reference::ShortArray(ConcurrentVec::from(vec![42]));
        stack.push_object(Some(array))?;
        stack.push_int(2)?;
        let result = saload(stack);
        assert!(matches!(result, Err(ArrayIndexOutOfBounds(2))));
        Ok(())
    }

    #[test]
    fn test_saload_null_pointer() -> crate::Result<()> {
        let stack = &mut OperandStack::with_max_size(2);
        stack.push_object(None)?;
        stack.push_int(0)?;
        let result = saload(stack);
        assert!(matches!(result, Err(NullPointer)));
        Ok(())
    }

    #[test]
    fn test_sastore() -> crate::Result<()> {
        let stack = &mut OperandStack::with_max_size(3);
        let array = Reference::ShortArray(ConcurrentVec::from(vec![3]));
        stack.push_object(Some(array))?;
        stack.push_int(0)?;
        stack.push_int(42)?;
        let result = sastore(stack)?;
        assert_eq!(Continue, result);
        Ok(())
    }

    #[test]
    fn test_sastore_invalid_value() -> crate::Result<()> {
        let stack = &mut OperandStack::with_max_size(3);
        let object = Reference::IntArray(ConcurrentVec::from(vec![42]));
        stack.push_object(Some(object))?;
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

    #[test]
    fn test_sastore_invalid_index() -> crate::Result<()> {
        let stack = &mut OperandStack::with_max_size(3);
        let array = Reference::ShortArray(ConcurrentVec::from(vec![3]));
        stack.push_object(Some(array))?;
        stack.push_int(2)?;
        stack.push_int(42)?;
        let result = sastore(stack);
        assert!(matches!(result, Err(ArrayIndexOutOfBounds(2))));
        Ok(())
    }

    #[test]
    fn test_sastore_null_pointer() -> crate::Result<()> {
        let stack = &mut OperandStack::with_max_size(3);
        stack.push_object(None)?;
        stack.push_int(0)?;
        stack.push_int(42)?;
        let result = sastore(stack);
        assert!(matches!(result, Err(NullPointer)));
        Ok(())
    }
}
