use crate::Error::{InvalidOperand, ParametersUnderflow};
use crate::Result;
use parking_lot::RwLock;
use ristretto_classloader::{Reference, Value};
use ristretto_gc::Gc;
use std::fmt::Display;

/// Parameters for Ristretto VM methods
#[derive(Clone, Debug, Default)]
pub(crate) struct Parameters {
    parameters: Vec<Value>,
}

impl Parameters {
    /// Create parameters from a vector of values.
    pub(crate) fn new(parameters: Vec<Value>) -> Self {
        Parameters { parameters }
    }

    /// Push a value onto the parameters.
    #[inline]
    pub fn push(&mut self, value: Value) {
        self.parameters.push(value);
    }

    /// Push a bool value onto the parameters.
    pub fn push_bool(&mut self, value: bool) {
        self.push(Value::from(value));
    }

    /// Push an int value onto the parameters.
    pub fn push_int(&mut self, value: i32) {
        self.push(Value::Int(value));
    }

    /// Push a long value onto the parameters.
    pub fn push_long(&mut self, value: i64) {
        self.push(Value::Long(value));
    }

    /// Push a float value onto the parameters.
    pub fn push_float(&mut self, value: f32) {
        self.push(Value::Float(value));
    }

    /// Push a double value onto the parameters.
    pub fn push_double(&mut self, value: f64) {
        self.push(Value::Double(value));
    }

    /// Push a reference onto the parameters.
    pub fn push_reference(&mut self, value: Option<Gc<RwLock<Reference>>>) {
        self.push(Value::Object(value));
    }

    /// Pop a value from the parameters.
    #[inline]
    pub fn pop(&mut self) -> Result<Value> {
        let Some(value) = self.parameters.pop() else {
            return Err(ParametersUnderflow);
        };
        Ok(value)
    }

    /// Pop a bool from the parameters.
    pub fn pop_bool(&mut self) -> Result<bool> {
        let bool_value = self.pop()?.as_bool()?;
        Ok(bool_value)
    }

    /// Pop an int from the parameters.
    pub fn pop_int(&mut self) -> Result<i32> {
        match self.pop()? {
            Value::Int(value) => Ok(value),
            value => Err(InvalidOperand {
                expected: "int".to_string(),
                actual: value.to_string(),
            }),
        }
    }

    /// Pop a long from the parameters.
    pub fn pop_long(&mut self) -> Result<i64> {
        match self.pop()? {
            Value::Long(value) => Ok(value),
            value => Err(InvalidOperand {
                expected: "long".to_string(),
                actual: value.to_string(),
            }),
        }
    }

    /// Pop a float from the parameters.
    pub fn pop_float(&mut self) -> Result<f32> {
        match self.pop()? {
            Value::Float(value) => Ok(value),
            value => Err(InvalidOperand {
                expected: "float".to_string(),
                actual: value.to_string(),
            }),
        }
    }

    /// Pop a double from the parameters.
    pub fn pop_double(&mut self) -> Result<f64> {
        match self.pop()? {
            Value::Double(value) => Ok(value),
            value => Err(InvalidOperand {
                expected: "double".to_string(),
                actual: value.to_string(),
            }),
        }
    }

    /// Pop a null or reference from the parameters.
    pub fn pop_reference(&mut self) -> Result<Option<Gc<RwLock<Reference>>>> {
        let value = self.pop()?;
        match value {
            Value::Object(reference) => Ok(reference),
            value => Err(InvalidOperand {
                expected: "reference".to_string(),
                actual: value.to_string(),
            }),
        }
    }

    /// Peek at the top value on the parameters.
    pub fn peek(&self) -> Result<&Value> {
        let Some(value) = self.parameters.last() else {
            return Err(ParametersUnderflow);
        };
        Ok(value)
    }

    /// Get the number of values on the parameters.
    pub fn len(&self) -> usize {
        self.parameters.len()
    }

    /// Check if the parameters is empty.
    pub fn is_empty(&self) -> bool {
        self.parameters.is_empty()
    }

    /// Returns a reference to the inner Vec
    pub fn as_vec(&self) -> &Vec<Value> {
        &self.parameters
    }

    /// Returns a slice reference
    pub fn as_slice(&self) -> &[Value] {
        &self.parameters
    }

    /// Returns a cloned Vec
    pub fn to_vec(&self) -> Vec<Value> {
        self.parameters.clone()
    }

    /// Consumes self and returns the Vec
    pub fn into_vec(self) -> Vec<Value> {
        self.parameters
    }
}

impl Display for Parameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}]",
            self.parameters
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
    use ristretto_classloader::Reference;

    #[test]
    fn test_can_push_and_pop_values() -> Result<()> {
        let mut parameters = Parameters::default();
        parameters.push(Value::Int(1));
        parameters.push(Value::Int(2));

        assert_eq!(parameters.len(), 2);

        assert_eq!(parameters.pop()?, Value::Int(2));
        assert_eq!(parameters.pop()?, Value::Int(1));
        Ok(())
    }

    #[test]
    fn test_pop_bool() -> Result<()> {
        let mut parameters = Parameters::default();
        parameters.push_bool(true);
        assert!(parameters.pop_bool()?);
        Ok(())
    }

    #[test]
    fn test_pop_int() -> Result<()> {
        let mut parameters = Parameters::default();
        parameters.push_int(42);
        assert_eq!(parameters.pop_int()?, 42);
        Ok(())
    }

    #[test]
    fn test_pop_int_invalid_operand() {
        let mut parameters = Parameters::default();
        parameters.push_reference(None);
        assert!(matches!(
            parameters.pop_int(),
            Err(InvalidOperand {
                expected,
                actual
            }) if expected == "int" && actual == "Object(null)"
        ));
    }

    #[test]
    fn test_pop_long() -> Result<()> {
        let mut parameters = Parameters::default();
        parameters.push_long(42);
        assert_eq!(parameters.pop_long()?, 42);
        Ok(())
    }

    #[test]
    fn test_pop_long_invalid_operand() {
        let mut parameters = Parameters::default();
        parameters.push_reference(None);
        assert!(matches!(
            parameters.pop_long(),
            Err(InvalidOperand {
                expected,
                actual
            }) if expected == "long" && actual == "Object(null)"
        ));
    }

    #[test]
    fn test_pop_float() -> Result<()> {
        let mut parameters = Parameters::default();
        parameters.push_float(42.1);
        let value = parameters.pop_float()? - 42.1f32;
        assert!(value.abs() < 0.1f32);
        Ok(())
    }

    #[test]
    fn test_pop_float_invalid_operand() {
        let mut parameters = Parameters::default();
        parameters.push_reference(None);
        assert!(matches!(
            parameters.pop_float(),
            Err(InvalidOperand {
                expected,
                actual
            }) if expected == "float" && actual == "Object(null)"
        ));
    }

    #[test]
    fn test_pop_double() -> Result<()> {
        let mut parameters = Parameters::default();
        parameters.push_double(42.1);
        let value = parameters.pop_double()? - 42.1f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[test]
    fn test_pop_double_invalid_operand() {
        let mut parameters = Parameters::default();
        parameters.push_reference(None);
        assert!(matches!(
            parameters.pop_double(),
            Err(InvalidOperand {
                expected,
                actual
            }) if expected == "double" && actual == "Object(null)"
        ));
    }

    #[tokio::test]
    async fn test_pop_reference() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let mut parameters = Parameters::default();
        let reference = Reference::from(vec![42i8]);
        parameters.push_reference(None);
        let value = Value::new_object(thread.vm()?.garbage_collector(), reference);
        let Value::Object(wrapped_object) = value else {
            panic!("expected object")
        };
        parameters.push_reference(wrapped_object);

        let popped = parameters.pop_reference()?;
        assert!(popped.is_some());
        let popped_ref = popped.expect("popped object");
        let guard = popped_ref.read();
        assert!(matches!(*guard, Reference::ByteArray(_)));
        assert!(parameters.pop_reference()?.is_none());
        Ok(())
    }

    #[test]
    fn test_pop_reference_invalid_operand() {
        let mut parameters = Parameters::default();
        parameters.push_int(42);
        assert!(matches!(
            parameters.pop_reference(),
            Err(InvalidOperand {
                expected,
                actual
            }) if expected == "reference" && actual == "int(42)"
        ));
    }

    #[test]
    fn test_pop_underflow() {
        let mut parameters = Parameters::default();
        let result = parameters.pop();
        assert!(matches!(result, Err(ParametersUnderflow)));
    }

    #[test]
    fn test_peek_top_value() -> Result<()> {
        let mut parameters = Parameters::default();
        parameters.push_int(1);
        parameters.push_int(2);

        assert_eq!(parameters.peek()?, &Value::Int(2));
        assert_eq!(parameters.len(), 2);
        Ok(())
    }

    #[test]
    fn test_peek_underflow() {
        let parameters = Parameters::default();
        let result = parameters.peek();
        assert!(matches!(result, Err(ParametersUnderflow)));
    }

    #[test]
    fn test_is_empty() {
        let mut parameters = Parameters::default();
        assert!(parameters.is_empty());

        parameters.push_int(42);
        assert!(!parameters.is_empty());
    }

    #[test]
    fn test_display() {
        let mut parameters = Parameters::default();
        parameters.push_int(1);
        parameters.push_int(2);
        assert_eq!("[int(1), int(2)]", parameters.to_string());
    }

    #[test]
    fn test_as_vec() {
        let mut parameters = Parameters::default();
        parameters.push_int(1);
        parameters.push_int(2);
        let vec = parameters.as_vec();
        assert_eq!(vec, &vec![Value::Int(1), Value::Int(2)]);
    }

    #[test]
    fn test_as_slice() {
        let mut parameters = Parameters::default();
        parameters.push_int(1);
        parameters.push_int(2);
        let slice = parameters.as_slice();
        assert_eq!(slice, &[Value::Int(1), Value::Int(2)]);
    }

    #[test]
    fn test_to_vec() {
        let mut parameters = Parameters::default();
        parameters.push_int(1);
        parameters.push_int(2);
        let vec = parameters.to_vec();
        assert_eq!(vec, vec![Value::Int(1), Value::Int(2)]);
    }

    #[test]
    fn test_into_vec() {
        let mut parameters = Parameters::default();
        parameters.push_int(1);
        parameters.push_int(2);
        let vec = parameters.into_vec();
        assert_eq!(vec, vec![Value::Int(1), Value::Int(2)]);
    }
}
