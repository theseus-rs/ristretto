use crate::Error::InternalError;
use crate::Result;
use cranelift::prelude::{Type, types};

const POINTER_SIZE: i32 = 8;

/// Type stack for determining block parameters when simulating stack operations.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TypeStack {
    stack: Vec<Type>,
}

impl TypeStack {
    /// Creates a new type stack with the specified capacity.
    pub fn new() -> Self {
        TypeStack { stack: Vec::new() }
    }

    /// Pushes a value onto the stack.
    #[expect(clippy::unnecessary_wraps)]
    pub fn push(&mut self, value_type: Type) -> Result<()> {
        self.stack.push(value_type);
        Ok(())
    }

    /// Push an int type onto the stack.
    pub fn push_int(&mut self) -> Result<()> {
        self.push(types::I32)
    }

    /// Push a long type onto the stack.
    pub fn push_long(&mut self) -> Result<()> {
        self.push(types::I64)
    }

    /// Push a float type onto the stack.
    pub fn push_float(&mut self) -> Result<()> {
        self.push(types::F32)
    }

    /// Push a double type onto the stack.
    pub fn push_double(&mut self) -> Result<()> {
        self.push(types::F64)
    }

    /// Push a double type onto the stack.
    pub fn push_object(&mut self) -> Result<()> {
        self.push(types::I64)
    }

    /// Pops a type from the stack.
    pub fn pop(&mut self) -> Result<Type> {
        let Some(value) = self.stack.pop() else {
            return Err(InternalError("TypeStack underflow".to_string()));
        };
        Ok(value)
    }

    /// Pop an int from the stack.
    pub fn pop_int(&mut self) -> Result<Type> {
        let expected_type = types::I32;
        let value_type = self.pop()?;
        if value_type != expected_type {
            return Err(InternalError(format!(
                "Expected {expected_type:?}, found {value_type:?}",
            )));
        }
        Ok(value_type)
    }

    /// Pop a long from the stack.
    pub fn pop_long(&mut self) -> Result<Type> {
        let expected_type = types::I64;
        let value_type = self.pop()?;
        if value_type != expected_type {
            return Err(InternalError(format!(
                "Expected {expected_type:?}, found {value_type:?}",
            )));
        }
        Ok(value_type)
    }

    /// Pop a float from the stack.
    pub fn pop_float(&mut self) -> Result<Type> {
        let expected_type = types::F32;
        let value_type = self.pop()?;
        if value_type != expected_type {
            return Err(InternalError(format!(
                "Expected {expected_type:?}, found {value_type:?}",
            )));
        }
        Ok(value_type)
    }

    /// Pop a double from the stack.
    pub fn pop_double(&mut self) -> Result<Type> {
        let expected_type = types::F64;
        let value_type = self.pop()?;
        if value_type != expected_type {
            return Err(InternalError(format!(
                "Expected {expected_type:?}, found {value_type:?}",
            )));
        }
        Ok(value_type)
    }

    /// Pop a object from the stack.
    pub fn pop_object(&mut self) -> Result<Type> {
        let expected_type = types::I64;
        let value_type = self.pop()?;
        if value_type != expected_type {
            return Err(InternalError(format!(
                "Expected {expected_type:?}, found {value_type:?}",
            )));
        }
        Ok(value_type)
    }

    /// Returns the length of the stack.
    pub fn len(&self) -> usize {
        self.stack.len()
    }

    /// Clears the stack.
    pub fn clear(&mut self) {
        self.stack.clear();
    }

    /// Returns stack as a vector.
    pub fn to_vec(&self) -> Vec<Type> {
        self.stack.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_stack_all_types() -> Result<()> {
        let mut type_stack = TypeStack::new();
        type_stack.push_int()?;
        type_stack.push_long()?;
        type_stack.push_float()?;
        type_stack.push_double()?;
        type_stack.push_object()?;

        assert_eq!(type_stack.stack.len(), 5);
        assert_eq!(type_stack.pop_object()?, types::I64);
        assert_eq!(type_stack.pop_double()?, types::F64);
        assert_eq!(type_stack.pop_float()?, types::F32);
        assert_eq!(type_stack.pop_long()?, types::I64);
        assert_eq!(type_stack.pop_int()?, types::I32);
        Ok(())
    }

    #[test]
    fn test_push_and_pop() -> Result<()> {
        let mut type_stack = TypeStack::new();
        type_stack.push(types::I32)?;

        let value_type = type_stack.pop()?;
        assert_eq!(value_type, types::I32);
        assert!(type_stack.pop().is_err());
        Ok(())
    }

    #[test]
    fn test_push_int_and_pop_int() -> Result<()> {
        let mut type_stack = TypeStack::new();
        type_stack.push_int()?;

        let value_type = type_stack.pop_int()?;
        assert_eq!(value_type, types::I32);
        assert!(type_stack.pop_int().is_err());
        Ok(())
    }

    #[test]
    fn test_pop_int_error() -> Result<()> {
        let mut type_stack = TypeStack::new();
        type_stack.push_long()?;
        assert!(type_stack.pop_int().is_err());
        Ok(())
    }

    #[test]
    fn test_push_long_and_pop_long() -> Result<()> {
        let mut type_stack = TypeStack::new();
        type_stack.push_long()?;

        let value_type = type_stack.pop_long()?;
        assert_eq!(value_type, types::I64);
        assert!(type_stack.pop_long().is_err());
        Ok(())
    }

    #[test]
    fn test_pop_long_error() -> Result<()> {
        let mut type_stack = TypeStack::new();
        type_stack.push_int()?;
        assert!(type_stack.pop_long().is_err());
        Ok(())
    }

    #[test]
    fn test_push_float_and_pop_float() -> Result<()> {
        let mut type_stack = TypeStack::new();
        type_stack.push_float()?;

        let value_type = type_stack.pop_float()?;
        assert_eq!(value_type, types::F32);
        assert!(type_stack.pop_float().is_err());
        Ok(())
    }

    #[test]
    fn test_pop_float_error() -> Result<()> {
        let mut type_stack = TypeStack::new();
        type_stack.push_long()?;
        assert!(type_stack.pop_float().is_err());
        Ok(())
    }

    #[test]
    fn test_push_double_and_pop_double() -> Result<()> {
        let mut type_stack = TypeStack::new();
        type_stack.push_double()?;

        let value_type = type_stack.pop_double()?;
        assert_eq!(value_type, types::F64);
        assert!(type_stack.pop_double().is_err());
        Ok(())
    }

    #[test]
    fn test_pop_double_error() -> Result<()> {
        let mut type_stack = TypeStack::new();
        type_stack.push_long()?;
        assert!(type_stack.pop_double().is_err());
        Ok(())
    }

    #[test]
    fn test_push_object_and_pop_object() -> Result<()> {
        let mut type_stack = TypeStack::new();
        type_stack.push_object()?;

        let value_type = type_stack.pop_object()?;
        assert_eq!(value_type, types::I64);
        assert!(type_stack.pop_object().is_err());
        Ok(())
    }

    #[test]
    fn test_pop_object_error() -> Result<()> {
        let mut type_stack = TypeStack::new();
        type_stack.push_int()?;
        assert!(type_stack.pop_object().is_err());
        Ok(())
    }

    #[test]
    fn test_to_vec() -> Result<()> {
        let mut type_stack = TypeStack::new();
        type_stack.push_int()?;
        type_stack.push_long()?;

        let types = type_stack.to_vec();
        assert_eq!(types, vec![types::I32, types::I64]);
        Ok(())
    }
}
