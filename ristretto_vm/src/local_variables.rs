use crate::Error::{InvalidLocalVariable, InvalidLocalVariableIndex};
use crate::Result;
use ristretto_classloader::{Reference, Value};
use std::fmt::Display;

/// Represents the local variables in a frame.
#[derive(Clone, Debug)]
pub struct LocalVariables {
    locals: Vec<Value>,
}

impl LocalVariables {
    /// Create a new local variables
    pub fn new(locals: Vec<Value>) -> Self {
        LocalVariables { locals }
    }

    /// Create a new local variables with a maximum size.
    pub fn with_max_size(max_size: usize) -> Self {
        Self::new(vec![Value::Unused; max_size])
    }

    /// Get a value from the local variables.
    ///
    /// # Errors
    ///
    /// if the local variable at the given index was not found.
    pub fn get(&self, index: usize) -> Result<Value> {
        match self.locals.get(index) {
            Some(value) => Ok(value.clone()),
            None => Err(InvalidLocalVariableIndex(index)),
        }
    }

    /// Get an int from the local variables.
    ///
    /// # Errors
    ///
    /// if the local variable at the given index was not found or if the value is not an int.
    pub fn get_int(&self, index: usize) -> Result<i32> {
        match self.locals.get(index) {
            Some(Value::Int(value)) => Ok(*value),
            Some(value) => Err(InvalidLocalVariable {
                expected: "int".to_string(),
                actual: value.to_string(),
            }),
            None => Err(InvalidLocalVariableIndex(index)),
        }
    }

    /// Get a long from the local variables.
    ///
    /// # Errors
    ///
    /// if the local variable at the given index was not found or if the value is not a long.
    pub fn get_long(&self, index: usize) -> Result<i64> {
        match self.locals.get(index) {
            Some(Value::Long(value)) => Ok(*value),
            Some(value) => Err(InvalidLocalVariable {
                expected: "long".to_string(),
                actual: value.to_string(),
            }),
            None => Err(InvalidLocalVariableIndex(index)),
        }
    }

    /// Get a float from the local variables.
    ///
    /// # Errors
    ///
    /// if the local variable at the given index was not found or if the value is not a float.
    pub fn get_float(&self, index: usize) -> Result<f32> {
        match self.locals.get(index) {
            Some(Value::Float(value)) => Ok(*value),
            Some(value) => Err(InvalidLocalVariable {
                expected: "float".to_string(),
                actual: value.to_string(),
            }),
            None => Err(InvalidLocalVariableIndex(index)),
        }
    }

    /// Get a double from the local variables.
    ///
    /// # Errors
    ///
    /// if the local variable at the given index was not found or if the value is not a double.
    pub fn get_double(&self, index: usize) -> Result<f64> {
        match self.locals.get(index) {
            Some(Value::Double(value)) => Ok(*value),
            Some(value) => Err(InvalidLocalVariable {
                expected: "double".to_string(),
                actual: value.to_string(),
            }),
            None => Err(InvalidLocalVariableIndex(index)),
        }
    }

    /// Get a null or object from the local variables.
    ///
    /// # Errors
    ///
    /// if the local variable at the given index was not found or if the value is not a null or
    /// object.
    pub fn get_object(&self, index: usize) -> Result<Option<Reference>> {
        match self.locals.get(index) {
            Some(Value::Object(reference)) => Ok(reference.clone()),
            Some(value) => Err(InvalidLocalVariable {
                expected: "object".to_string(),
                actual: value.to_string(),
            }),
            None => Err(InvalidLocalVariableIndex(index)),
        }
    }

    /// Set a value in the local variables.
    ///
    /// # Errors
    ///
    /// if the index is out of bounds.
    pub fn set(&mut self, index: usize, value: Value) -> Result<()> {
        if index >= self.locals.capacity() {
            return Err(InvalidLocalVariableIndex(index));
        }
        if index < self.locals.len() {
            let _ = std::mem::replace(&mut self.locals[index], value);
        }
        Ok(())
    }

    /// Set an int in the local variables.
    ///
    /// # Errors
    ///
    /// if the index is out of bounds or if the value is not an int.
    pub fn set_int(&mut self, index: usize, value: i32) -> Result<()> {
        self.set(index, Value::Int(value))
    }

    /// Set a long in the local variables.
    ///
    /// # Errors
    ///
    /// if the index is out of bounds or if the value is not a long.
    pub fn set_long(&mut self, index: usize, value: i64) -> Result<()> {
        self.set(index, Value::Long(value))
    }

    /// Set a float in the local variables.
    ///
    /// # Errors
    ///
    /// if the index is out of bounds or if the value is not a float.
    pub fn set_float(&mut self, index: usize, value: f32) -> Result<()> {
        self.set(index, Value::Float(value))
    }

    /// Set a double in the local variables.
    ///
    /// # Errors
    ///
    /// if the index is out of bounds or if the value is not a double.
    pub fn set_double(&mut self, index: usize, value: f64) -> Result<()> {
        self.set(index, Value::Double(value))
    }

    /// Set a null or object in the local variables.
    ///
    /// # Errors
    ///
    /// if the index is out of bounds or if the value is not a null or object.
    pub fn set_object(&mut self, index: usize, value: Option<Reference>) -> Result<()> {
        self.set(index, Value::Object(value))
    }

    /// Get the length of the local variables.
    pub fn len(&self) -> usize {
        self.locals
            .iter()
            .enumerate()
            .rev()
            .find(|&(_, value)| *value != Value::Unused)
            .map_or(0, |(index, _)| index + 1)
    }

    /// Check if the local variables are empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Display for LocalVariables {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut locals = Vec::new();
        for local in &self.locals {
            let value = local.to_string();
            let chars: Vec<char> = value.chars().collect();
            if chars.len() > 100 {
                let value = chars.iter().take(97).collect::<String>();
                locals.push(format!("{value}..."));
            } else {
                locals.push(value);
            }
        }
        write!(f, "[{}]", locals.join(", "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_classloader::Reference;

    #[test]
    fn test_get() -> Result<()> {
        let mut locals = LocalVariables::with_max_size(1);
        locals.set(0, Value::Int(42))?;
        assert_eq!(locals.get(0)?.to_int()?, 42);
        Ok(())
    }

    #[test]
    fn test_get_invalid_index() {
        let locals = LocalVariables::with_max_size(0);
        assert!(matches!(locals.get(0), Err(InvalidLocalVariableIndex(0))));
    }

    #[test]
    fn test_get_int() -> Result<()> {
        let mut locals = LocalVariables::with_max_size(1);
        locals.set(0, Value::Int(42))?;
        assert_eq!(locals.get_int(0)?, 42);
        Ok(())
    }

    #[test]
    fn test_get_int_invalid_index() {
        let locals = LocalVariables::with_max_size(0);
        assert!(matches!(
            locals.get_int(0),
            Err(InvalidLocalVariableIndex(0))
        ));
    }

    #[test]
    fn test_get_int_invalid_type() {
        let locals = LocalVariables::with_max_size(1);
        assert!(matches!(
            locals.get_int(0),
            Err(InvalidLocalVariable {
                expected,
                actual
            }) if expected == "int" && actual == "unused"
        ));
    }

    #[test]
    fn test_get_long() -> Result<()> {
        let mut locals = LocalVariables::with_max_size(1);
        locals.set(0, Value::Long(42))?;
        assert_eq!(locals.get_long(0)?, 42);
        Ok(())
    }

    #[test]
    fn test_get_long_invalid_index() {
        let locals = LocalVariables::with_max_size(0);
        assert!(matches!(
            locals.get_long(0),
            Err(InvalidLocalVariableIndex(0))
        ));
    }

    #[test]
    fn test_get_long_invalid_type() {
        let locals = LocalVariables::with_max_size(1);
        assert!(matches!(
            locals.get_long(0),
            Err(InvalidLocalVariable {
                expected,
                actual
            }) if expected == "long" && actual == "unused"
        ));
    }

    #[test]
    fn test_get_float() -> Result<()> {
        let mut locals = LocalVariables::with_max_size(1);
        locals.set(0, Value::Float(42.1))?;
        let value = locals.get_float(0)? - 42.1f32;
        assert!(value.abs() < 0.1f32);
        Ok(())
    }

    #[test]
    fn test_get_float_invalid_index() {
        let locals = LocalVariables::with_max_size(0);
        assert!(matches!(
            locals.get_float(0),
            Err(InvalidLocalVariableIndex(0))
        ));
    }

    #[test]
    fn test_get_float_invalid_type() {
        let locals = LocalVariables::with_max_size(1);
        assert!(matches!(
            locals.get_float(0),
            Err(InvalidLocalVariable {
                expected,
                actual
            }) if expected == "float" && actual == "unused"
        ));
    }

    #[test]
    fn test_get_double() -> Result<()> {
        let mut locals = LocalVariables::with_max_size(1);
        locals.set(0, Value::Double(42.1))?;
        let value = locals.get_double(0)? - 42.1f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[test]
    fn test_get_double_invalid_index() {
        let locals = LocalVariables::with_max_size(0);
        assert!(matches!(
            locals.get_double(0),
            Err(InvalidLocalVariableIndex(0))
        ));
    }

    #[test]
    fn test_get_double_invalid_type() {
        let locals = LocalVariables::with_max_size(1);
        assert!(matches!(
            locals.get_double(0),
            Err(InvalidLocalVariable {
                expected,
                actual
            }) if expected == "double" && actual == "unused"
        ));
    }

    #[test]
    fn test_get_object() -> Result<()> {
        let mut locals = LocalVariables::with_max_size(2);
        let object = Reference::from(vec![42i8]);
        locals.set_object(0, None)?;
        locals.set_object(1, Some(object.clone()))?;
        assert_eq!(locals.get_object(0)?, None);
        assert_eq!(locals.get_object(1)?, Some(object));
        Ok(())
    }

    #[test]
    fn test_get_object_invalid_index() {
        let locals = LocalVariables::with_max_size(0);
        assert!(matches!(
            locals.get_object(0),
            Err(InvalidLocalVariableIndex(0))
        ));
    }

    #[test]
    fn test_get_object_invalid_type() {
        let locals = LocalVariables::with_max_size(1);
        assert!(matches!(
            locals.get_object(0),
            Err(InvalidLocalVariable {
                expected,
                actual
            }) if expected == "object" && actual == "unused"
        ));
    }

    #[test]
    fn test_set() -> Result<()> {
        let mut locals = LocalVariables::with_max_size(1);
        locals.set(0, Value::Int(42))?;
        assert_eq!(locals.get(0)?.to_int()?, 42);
        Ok(())
    }

    #[test]
    fn test_set_invalid_index() {
        let mut locals = LocalVariables::with_max_size(0);
        assert!(matches!(
            locals.set(0, Value::Object(None)),
            Err(InvalidLocalVariableIndex(0))
        ));
    }

    #[test]
    fn test_set_int() -> Result<()> {
        let mut locals = LocalVariables::with_max_size(1);
        locals.set_int(0, 42)?;
        assert_eq!(locals.get_int(0)?, 42);
        Ok(())
    }

    #[test]
    fn test_set_int_invalid_index() {
        let mut locals = LocalVariables::with_max_size(0);
        assert!(matches!(
            locals.set_int(0, 42),
            Err(InvalidLocalVariableIndex(0))
        ));
    }

    #[test]
    fn test_set_long() -> Result<()> {
        let mut locals = LocalVariables::with_max_size(1);
        locals.set_long(0, 42)?;
        assert_eq!(locals.get_long(0)?, 42);
        Ok(())
    }

    #[test]
    fn test_set_long_invalid_index() {
        let mut locals = LocalVariables::with_max_size(0);
        assert!(matches!(
            locals.set_long(0, 42),
            Err(InvalidLocalVariableIndex(0))
        ));
    }

    #[test]
    fn test_set_float() -> Result<()> {
        let mut locals = LocalVariables::with_max_size(1);
        locals.set_float(0, 42.1)?;
        let value = locals.get_float(0)? - 42.1f32;
        assert!(value.abs() < 0.1f32);
        Ok(())
    }

    #[test]
    fn test_set_float_invalid_index() {
        let mut locals = LocalVariables::with_max_size(0);
        assert!(matches!(
            locals.set_float(0, 42.1),
            Err(InvalidLocalVariableIndex(0))
        ));
    }

    #[test]
    fn test_set_double() -> Result<()> {
        let mut locals = LocalVariables::with_max_size(1);
        locals.set_double(0, 42.1)?;
        let value = locals.get_double(0)? - 42.1f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[test]
    fn test_set_double_invalid_index() {
        let mut locals = LocalVariables::with_max_size(0);
        assert!(matches!(
            locals.set_double(0, 42.1),
            Err(InvalidLocalVariableIndex(0))
        ));
    }

    #[test]
    fn test_set_object() -> Result<()> {
        let mut locals = LocalVariables::with_max_size(2);
        let object = Reference::from(vec![42i8]);
        locals.set_object(0, None)?;
        locals.set_object(1, Some(object.clone()))?;
        assert_eq!(locals.get_object(0)?, None);
        assert_eq!(locals.get_object(1)?, Some(object));
        Ok(())
    }

    #[test]
    fn test_set_object_invalid_index() {
        let mut locals = LocalVariables::with_max_size(0);
        assert!(matches!(
            locals.set_object(0, None),
            Err(InvalidLocalVariableIndex(0))
        ));
    }

    #[test]
    fn test_len() -> Result<()> {
        let mut local_variables = LocalVariables::with_max_size(3);
        assert_eq!(local_variables.len(), 0);
        local_variables.set(1, Value::Int(42))?;
        assert_eq!(local_variables.len(), 2);
        Ok(())
    }

    #[test]
    fn test_is_empty() -> Result<()> {
        let mut local_variables = LocalVariables::with_max_size(3);
        assert!(local_variables.is_empty());
        local_variables.set(1, Value::Int(42))?;
        assert!(!local_variables.is_empty());
        Ok(())
    }

    #[test]
    fn test_display() -> Result<()> {
        let mut local_variables = LocalVariables::with_max_size(6);
        local_variables.set(0, Value::Int(1))?;
        local_variables.set(1, Value::Long(42))?;
        local_variables.set(2, Value::Float(2.3))?;
        local_variables.set(4, Value::Double(42.1))?;
        assert_eq!(
            "[int(1), long(42), float(2.3), unused, double(42.1), unused]",
            local_variables.to_string()
        );
        Ok(())
    }
}
