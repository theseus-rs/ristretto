use crate::Error::InternalError;
use crate::Result;
use cranelift::prelude::{Type, types};

const POINTER_SIZE: i32 = 8;

/// Type stack for determining block parameters when simulating stack operations.
///
/// This struct maintains a stack of Cranelift types to track the types of values during
/// compilation. It helps with type checking and ensures proper stack manipulation in the control
/// flow graph.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TypeStack {
    stack: Vec<Type>,
}

impl TypeStack {
    /// Creates a new empty type stack.
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

    /// Pushes an object reference type (represented as I64) onto the stack.
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

    /// Pop a type from the stack and verify it matches the expected type.
    fn pop_type(&mut self, expected_type: Type) -> Result<Type> {
        let value_type = self.pop()?;
        if value_type != expected_type {
            return Err(InternalError(format!(
                "Expected {expected_type:?}, found {value_type:?}",
            )));
        }
        Ok(value_type)
    }

    /// Pop an int from the stack, verifying the type.
    pub fn pop_int(&mut self) -> Result<Type> {
        self.pop_type(types::I32)
    }

    /// Pop a long from the stack, verifying the type.
    pub fn pop_long(&mut self) -> Result<Type> {
        self.pop_type(types::I64)
    }

    /// Pop a float from the stack, verifying the type.
    pub fn pop_float(&mut self) -> Result<Type> {
        self.pop_type(types::F32)
    }

    /// Pop a double from the stack, verifying the type.
    pub fn pop_double(&mut self) -> Result<Type> {
        self.pop_type(types::F64)
    }

    /// Pops an object reference from the stack, verifying the type.
    pub fn pop_object(&mut self) -> Result<Type> {
        self.pop_type(types::I64)
    }

    /// Returns the number of elements in the stack.
    pub fn len(&self) -> usize {
        self.stack.len()
    }

    /// Checks if the stack is empty.
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    /// Returns a reference to the inner Vec
    pub fn as_vec(&self) -> &Vec<Type> {
        &self.stack
    }

    /// Returns a slice reference
    pub fn as_slice(&self) -> &[Type] {
        &self.stack
    }

    /// Returns a cloned Vec
    pub fn to_vec(&self) -> Vec<Type> {
        self.stack.clone()
    }

    /// Consumes self and returns the Vec
    pub fn into_vec(self) -> Vec<Type> {
        self.stack
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
    fn test_len() -> Result<()> {
        let mut type_stack = TypeStack::new();
        assert_eq!(type_stack.len(), 0);
        type_stack.push_int()?;
        assert_eq!(type_stack.len(), 1);
        Ok(())
    }

    #[test]
    fn test_is_empty() -> Result<()> {
        let mut type_stack = TypeStack::new();
        assert!(type_stack.is_empty());
        type_stack.push_int()?;
        assert!(!type_stack.is_empty());
        Ok(())
    }

    #[test]
    fn test_as_vec() -> Result<()> {
        let mut type_stack = TypeStack::new();
        type_stack.push_int()?;
        type_stack.push_long()?;
        let vec = type_stack.as_vec();
        assert_eq!(vec.len(), 2);
        assert_eq!(vec[0], types::I32);
        assert_eq!(vec[1], types::I64);
        Ok(())
    }

    #[test]
    fn test_as_slice() -> Result<()> {
        let mut type_stack = TypeStack::new();
        type_stack.push_int()?;
        type_stack.push_long()?;
        let slice = type_stack.as_slice();
        assert_eq!(slice.len(), 2);
        assert_eq!(slice[0], types::I32);
        assert_eq!(slice[1], types::I64);
        Ok(())
    }

    #[test]
    fn test_to_vec() -> Result<()> {
        let mut type_stack = TypeStack::new();
        type_stack.push_int()?;
        type_stack.push_long()?;
        let vec = type_stack.to_vec();
        assert_eq!(vec.len(), 2);
        assert_eq!(vec[0], types::I32);
        assert_eq!(vec[1], types::I64);
        Ok(())
    }

    #[test]
    fn test_into_vec() -> Result<()> {
        let mut type_stack = TypeStack::new();
        type_stack.push_int()?;
        type_stack.push_long()?;
        let vec = type_stack.into_vec();
        assert_eq!(vec.len(), 2);
        assert_eq!(vec[0], types::I32);
        assert_eq!(vec[1], types::I64);
        Ok(())
    }
}
