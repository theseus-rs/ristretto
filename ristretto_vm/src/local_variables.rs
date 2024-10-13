use crate::Error::{InvalidLocalVariable, InvalidLocalVariableIndex, PoisonedLock};
use crate::Result;
use ristretto_classloader::{Reference, Value};
use std::cell::{Ref, RefCell};
use std::fmt::Display;

/// Represents the local variables in a frame.
#[derive(Clone, Debug)]
pub(crate) struct LocalVariables {
    locals: Vec<RefCell<Value>>,
}

impl LocalVariables {
    /// Create a new local variables with a maximum size.
    pub fn with_max_size(max_size: usize) -> Self {
        LocalVariables {
            locals: vec![RefCell::new(Value::Unused); max_size],
        }
    }

    /// Get a value from the local variables.
    ///
    /// # Errors
    /// if the local variable at the given index was not found.
    #[inline]
    pub fn get(&self, index: usize) -> Result<Ref<Value>> {
        match self.locals.get(index) {
            Some(value) => {
                let value = value
                    .try_borrow()
                    .map_err(|error| PoisonedLock(error.to_string()))?;
                Ok(value)
            }
            None => Err(InvalidLocalVariableIndex(index)),
        }
    }

    /// Get an int from the local variables.
    ///
    /// # Errors
    /// if the local variable at the given index was not found or if the value is not an int.
    pub fn get_int(&self, index: usize) -> Result<i32> {
        let value = self.get(index)?;
        value.as_int().map_err(|_error| InvalidLocalVariable {
            expected: "int".to_string(),
            actual: value.to_string(),
        })
    }

    /// Get a long from the local variables.
    ///
    /// # Errors
    /// if the local variable at the given index was not found or if the value is not a long.
    pub fn get_long(&self, index: usize) -> Result<i64> {
        let value = self.get(index)?;
        value.as_long().map_err(|_error| InvalidLocalVariable {
            expected: "long".to_string(),
            actual: value.to_string(),
        })
    }

    /// Get a float from the local variables.
    ///
    /// # Errors
    /// if the local variable at the given index was not found or if the value is not a float.
    pub fn get_float(&self, index: usize) -> Result<f32> {
        let value = self.get(index)?;
        value.as_float().map_err(|_error| InvalidLocalVariable {
            expected: "float".to_string(),
            actual: value.to_string(),
        })
    }

    /// Get a double from the local variables.
    ///
    /// # Errors
    /// if the local variable at the given index was not found or if the value is not a double.
    pub fn get_double(&self, index: usize) -> Result<f64> {
        let value = self.get(index)?;
        value.as_double().map_err(|_error| InvalidLocalVariable {
            expected: "double".to_string(),
            actual: value.to_string(),
        })
    }

    /// Get a null or object from the local variables.
    ///
    /// # Errors
    /// if the local variable at the given index was not found or if the value is not a null or
    /// object.
    pub fn get_object(&self, index: usize) -> Result<Option<Reference>> {
        let value = self.get(index)?;
        let reference = value.as_object().map_err(|_error| InvalidLocalVariable {
            expected: "object".to_string(),
            actual: value.to_string(),
        })?;
        Ok(reference.cloned())
    }

    /// Set a value in the local variables.
    ///
    /// # Errors
    /// if the index is out of bounds.
    #[inline]
    pub fn set(&self, index: usize, value: Value) -> Result<()> {
        if index < self.locals.capacity() {
            self.locals[index].replace(value);
            Ok(())
        } else {
            Err(InvalidLocalVariableIndex(index))
        }
    }

    /// Set an int in the local variables.
    ///
    /// # Errors
    /// if the index is out of bounds or if the value is not an int.
    pub fn set_int(&self, index: usize, value: i32) -> Result<()> {
        self.set(index, Value::Int(value))
    }

    /// Set a long in the local variables.
    ///
    /// # Errors
    /// if the index is out of bounds or if the value is not a long.
    pub fn set_long(&self, index: usize, value: i64) -> Result<()> {
        self.set(index, Value::Long(value))
    }

    /// Set a float in the local variables.
    ///
    /// # Errors
    /// if the index is out of bounds or if the value is not a float.
    pub fn set_float(&self, index: usize, value: f32) -> Result<()> {
        self.set(index, Value::Float(value))
    }

    /// Set a double in the local variables.
    ///
    /// # Errors
    /// if the index is out of bounds or if the value is not a double.
    pub fn set_double(&self, index: usize, value: f64) -> Result<()> {
        self.set(index, Value::Double(value))
    }

    /// Set a null or object in the local variables.
    ///
    /// # Errors
    /// if the index is out of bounds or if the value is not a null or object.
    pub fn set_object(&self, index: usize, value: Option<Reference>) -> Result<()> {
        self.set(index, Value::Object(value))
    }

    /// Get the length of the local variables.
    pub fn len(&self) -> Result<usize> {
        let mut length = 0;
        for (index, value) in self.locals.iter().enumerate() {
            let value = value
                .try_borrow()
                .map_err(|error| PoisonedLock(error.to_string()))?;
            if *value != Value::Unused {
                length = index + 1;
            }
        }
        Ok(length)
    }

    /// Check if the local variables are empty.
    pub fn is_empty(&self) -> Result<bool> {
        Ok(self.len()? == 0)
    }
}

impl Display for LocalVariables {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut locals = Vec::new();
        for local in &self.locals {
            let value = match local.try_borrow() {
                Ok(value) => value.to_string(),
                Err(error) => format!("poisoned lock: {error}"),
            };
            if value.len() > 100 {
                locals.push(format!("{}...", &value[..97]));
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
    use ristretto_classloader::{ConcurrentVec, Reference};

    #[test]
    fn test_get() -> Result<()> {
        let locals = LocalVariables::with_max_size(1);
        locals.set(0, Value::Int(42))?;
        let result = locals.get(0)?.as_int()?;
        assert_eq!(result, 42);
        Ok(())
    }

    #[test]
    fn test_get_invalid_index() {
        let locals = LocalVariables::with_max_size(0);
        assert!(matches!(locals.get(0), Err(InvalidLocalVariableIndex(0))));
    }

    #[test]
    fn test_get_int() -> Result<()> {
        let locals = LocalVariables::with_max_size(1);
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
        let locals = LocalVariables::with_max_size(1);
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
        let locals = LocalVariables::with_max_size(1);
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
        let locals = LocalVariables::with_max_size(1);
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
        let locals = LocalVariables::with_max_size(2);
        let object = Reference::ByteArray(ConcurrentVec::from(vec![42]));
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
        let locals = LocalVariables::with_max_size(1);
        locals.set(0, Value::Int(42))?;
        let result = locals.get(0)?.as_int()?;
        assert_eq!(result, 42);
        Ok(())
    }

    #[test]
    fn test_set_invalid_index() {
        let locals = LocalVariables::with_max_size(0);
        assert!(matches!(
            locals.set(0, Value::Object(None)),
            Err(InvalidLocalVariableIndex(0))
        ));
    }

    #[test]
    fn test_set_int() -> Result<()> {
        let locals = LocalVariables::with_max_size(1);
        locals.set_int(0, 42)?;
        assert_eq!(locals.get_int(0)?, 42);
        Ok(())
    }

    #[test]
    fn test_set_int_invalid_index() {
        let locals = LocalVariables::with_max_size(0);
        assert!(matches!(
            locals.set_int(0, 42),
            Err(InvalidLocalVariableIndex(0))
        ));
    }

    #[test]
    fn test_set_long() -> Result<()> {
        let locals = LocalVariables::with_max_size(1);
        locals.set_long(0, 42)?;
        assert_eq!(locals.get_long(0)?, 42);
        Ok(())
    }

    #[test]
    fn test_set_long_invalid_index() {
        let locals = LocalVariables::with_max_size(0);
        assert!(matches!(
            locals.set_long(0, 42),
            Err(InvalidLocalVariableIndex(0))
        ));
    }

    #[test]
    fn test_set_float() -> Result<()> {
        let locals = LocalVariables::with_max_size(1);
        locals.set_float(0, 42.1)?;
        let value = locals.get_float(0)? - 42.1f32;
        assert!(value.abs() < 0.1f32);
        Ok(())
    }

    #[test]
    fn test_set_float_invalid_index() {
        let locals = LocalVariables::with_max_size(0);
        assert!(matches!(
            locals.set_float(0, 42.1),
            Err(InvalidLocalVariableIndex(0))
        ));
    }

    #[test]
    fn test_set_double() -> Result<()> {
        let locals = LocalVariables::with_max_size(1);
        locals.set_double(0, 42.1)?;
        let value = locals.get_double(0)? - 42.1f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[test]
    fn test_set_double_invalid_index() {
        let locals = LocalVariables::with_max_size(0);
        assert!(matches!(
            locals.set_double(0, 42.1),
            Err(InvalidLocalVariableIndex(0))
        ));
    }

    #[test]
    fn test_set_object() -> Result<()> {
        let locals = LocalVariables::with_max_size(2);
        let object = Reference::ByteArray(ConcurrentVec::from(vec![42]));
        locals.set_object(0, None)?;
        locals.set_object(1, Some(object.clone()))?;
        assert_eq!(locals.get_object(0)?, None);
        assert_eq!(locals.get_object(1)?, Some(object));
        Ok(())
    }

    #[test]
    fn test_set_object_invalid_index() {
        let locals = LocalVariables::with_max_size(0);
        assert!(matches!(
            locals.set_object(0, None),
            Err(InvalidLocalVariableIndex(0))
        ));
    }

    #[test]
    fn test_len() -> Result<()> {
        let local_variables = LocalVariables::with_max_size(3);
        assert_eq!(local_variables.len()?, 0);
        local_variables.set(1, Value::Int(42))?;
        assert_eq!(local_variables.len()?, 2);
        Ok(())
    }

    #[test]
    fn test_is_empty() -> Result<()> {
        let local_variables = LocalVariables::with_max_size(3);
        assert!(local_variables.is_empty()?);
        local_variables.set(1, Value::Int(42))?;
        assert!(!local_variables.is_empty()?);
        Ok(())
    }

    #[test]
    fn test_display() -> Result<()> {
        let local_variables = LocalVariables::with_max_size(6);
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
