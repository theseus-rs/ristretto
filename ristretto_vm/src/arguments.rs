use crate::Error::{ArgumentsUnderflow, InvalidOperand};
use crate::Result;
use ristretto_classloader::{Reference, Value};
use std::fmt::Display;

/// Arguments for Ristretto VM rust arguments
#[derive(Clone, Debug, Default)]
pub(crate) struct Arguments {
    arguments: Vec<Value>,
}

impl Arguments {
    /// Create arguments from a vector of values.
    pub(crate) fn new(arguments: Vec<Value>) -> Self {
        Arguments { arguments }
    }

    /// Push a value onto the arguments.
    #[inline]
    pub fn push(&mut self, value: Value) {
        self.arguments.push(value);
    }

    /// Push an int value onto the arguments.
    pub fn push_int(&mut self, value: i32) {
        self.push(Value::Int(value));
    }

    /// Push a long value onto the arguments.
    pub fn push_long(&mut self, value: i64) {
        self.push(Value::Long(value));
    }

    /// Push a float value onto the arguments.
    pub fn push_float(&mut self, value: f32) {
        self.push(Value::Float(value));
    }

    /// Push a double value onto the arguments.
    pub fn push_double(&mut self, value: f64) {
        self.push(Value::Double(value));
    }

    /// Push a reference onto the arguments.
    pub fn push_object(&mut self, value: Option<Reference>) {
        self.push(Value::Object(value));
    }

    /// Pop a value from the arguments.
    #[inline]
    pub fn pop(&mut self) -> Result<Value> {
        let Some(value) = self.arguments.pop() else {
            return Err(ArgumentsUnderflow);
        };
        Ok(value)
    }

    /// Pop an int from the arguments.
    pub fn pop_int(&mut self) -> Result<i32> {
        match self.pop()? {
            Value::Int(value) => Ok(value),
            value => Err(InvalidOperand {
                expected: "int".to_string(),
                actual: value.to_string(),
            }),
        }
    }

    /// Pop a long from the arguments.
    pub fn pop_long(&mut self) -> Result<i64> {
        match self.pop()? {
            Value::Long(value) => Ok(value),
            value => Err(InvalidOperand {
                expected: "long".to_string(),
                actual: value.to_string(),
            }),
        }
    }

    /// Pop a float from the arguments.
    pub fn pop_float(&mut self) -> Result<f32> {
        match self.pop()? {
            Value::Float(value) => Ok(value),
            value => Err(InvalidOperand {
                expected: "float".to_string(),
                actual: value.to_string(),
            }),
        }
    }

    /// Pop a double from the arguments.
    pub fn pop_double(&mut self) -> Result<f64> {
        match self.pop()? {
            Value::Double(value) => Ok(value),
            value => Err(InvalidOperand {
                expected: "double".to_string(),
                actual: value.to_string(),
            }),
        }
    }

    /// Pop a null or object from the arguments.
    pub fn pop_object(&mut self) -> Result<Option<Reference>> {
        let value = self.pop()?;
        match value {
            Value::Object(reference) => Ok(reference),
            value => Err(InvalidOperand {
                expected: "object".to_string(),
                actual: value.to_string(),
            }),
        }
    }

    /// Peek at the top value on the arguments.
    pub fn peek(&self) -> Result<&Value> {
        let Some(value) = self.arguments.last() else {
            return Err(ArgumentsUnderflow);
        };
        Ok(value)
    }

    /// Get the number of values on the arguments.
    pub fn len(&self) -> usize {
        self.arguments.len()
    }

    /// Check if the arguments is empty.
    pub fn is_empty(&self) -> bool {
        self.arguments.is_empty()
    }
}

impl Display for Arguments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}]",
            self.arguments
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_classloader::{ConcurrentVec, Reference};

    #[test]
    fn test_can_push_and_pop_values() -> Result<()> {
        let mut arguments = Arguments::default();
        arguments.push(Value::Int(1));
        arguments.push(Value::Int(2));

        assert_eq!(arguments.len(), 2);

        assert_eq!(arguments.pop()?, Value::Int(2));
        assert_eq!(arguments.pop()?, Value::Int(1));
        Ok(())
    }

    #[test]
    fn test_pop_int() -> Result<()> {
        let mut arguments = Arguments::default();
        arguments.push_int(42);
        assert_eq!(arguments.pop_int()?, 42);
        Ok(())
    }

    #[test]
    fn test_pop_int_invalid_operand() {
        let mut arguments = Arguments::default();
        arguments.push_object(None);
        assert!(matches!(
            arguments.pop_int(),
            Err(InvalidOperand {
                expected,
                actual
            }) if expected == "int" && actual == "object(null)"
        ));
    }

    #[test]
    fn test_pop_long() -> Result<()> {
        let mut arguments = Arguments::default();
        arguments.push_long(42);
        assert_eq!(arguments.pop_long()?, 42);
        Ok(())
    }

    #[test]
    fn test_pop_long_invalid_operand() {
        let mut arguments = Arguments::default();
        arguments.push_object(None);
        assert!(matches!(
            arguments.pop_long(),
            Err(InvalidOperand {
                expected,
                actual
            }) if expected == "long" && actual == "object(null)"
        ));
    }

    #[test]
    fn test_pop_float() -> Result<()> {
        let mut arguments = Arguments::default();
        arguments.push_float(42.1);
        let value = arguments.pop_float()? - 42.1f32;
        assert!(value.abs() < 0.1f32);
        Ok(())
    }

    #[test]
    fn test_pop_float_invalid_operand() {
        let mut arguments = Arguments::default();
        arguments.push_object(None);
        assert!(matches!(
            arguments.pop_float(),
            Err(InvalidOperand {
                expected,
                actual
            }) if expected == "float" && actual == "object(null)"
        ));
    }

    #[test]
    fn test_pop_double() -> Result<()> {
        let mut arguments = Arguments::default();
        arguments.push_double(42.1);
        let value = arguments.pop_double()? - 42.1f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[test]
    fn test_pop_double_invalid_operand() {
        let mut arguments = Arguments::default();
        arguments.push_object(None);
        assert!(matches!(
            arguments.pop_double(),
            Err(InvalidOperand {
                expected,
                actual
            }) if expected == "double" && actual == "object(null)"
        ));
    }

    #[test]
    fn test_pop_object() -> Result<()> {
        let mut arguments = Arguments::default();
        let object = Reference::ByteArray(ConcurrentVec::from(vec![42]));
        arguments.push_object(None);
        arguments.push_object(Some(object.clone()));
        assert_eq!(arguments.pop_object()?, Some(object));
        assert_eq!(arguments.pop_object()?, None);
        Ok(())
    }

    #[test]
    fn test_pop_object_invalid_operand() {
        let mut arguments = Arguments::default();
        arguments.push_int(42);
        assert!(matches!(
            arguments.pop_object(),
            Err(InvalidOperand {
                expected,
                actual
            }) if expected == "object" && actual == "int(42)"
        ));
    }

    #[test]
    fn test_pop_underflow() {
        let mut arguments = Arguments::default();
        let result = arguments.pop();
        assert!(matches!(result, Err(ArgumentsUnderflow)));
    }

    #[test]
    fn test_peek_top_value() -> Result<()> {
        let mut arguments = Arguments::default();
        arguments.push_int(1);
        arguments.push_int(2);

        assert_eq!(arguments.peek()?, &Value::Int(2));
        assert_eq!(arguments.len(), 2);
        Ok(())
    }

    #[test]
    fn test_peek_underflow() {
        let arguments = Arguments::default();
        let result = arguments.peek();
        assert!(matches!(result, Err(ArgumentsUnderflow)));
    }

    #[test]
    fn test_is_empty() {
        let mut arguments = Arguments::default();
        assert!(arguments.is_empty());

        arguments.push_int(42);
        assert!(!arguments.is_empty());
    }

    #[test]
    fn test_display() {
        let mut arguments = Arguments::default();
        arguments.push_int(1);
        arguments.push_int(2);
        assert_eq!("[int(1), int(2)]", arguments.to_string());
    }
}
