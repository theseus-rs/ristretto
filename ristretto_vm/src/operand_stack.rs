use crate::Error::{InvalidOperand, OperandStackOverflow, OperandStackUnderflow};
use crate::Result;
use ristretto_classloader::{ConcurrentVec, Reference, Value};
use std::fmt::Display;

/// Operand stack for the Ristretto VM
///
/// See: <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-2.html#jvms-2.6.2>
#[derive(Debug)]
pub struct OperandStack {
    stack: ConcurrentVec<Value>,
}

impl OperandStack {
    /// Create a new operand stack with a maximum size.
    pub fn with_max_size(max_size: usize) -> Self {
        OperandStack {
            stack: ConcurrentVec::with_capacity(max_size),
        }
    }

    /// Push a value onto the operand stack.
    #[inline]
    pub fn push(&self, value: Value) -> Result<()> {
        if self.stack.len()? >= self.stack.capacity()? {
            return Err(OperandStackOverflow);
        }
        self.stack.push(value)?;
        Ok(())
    }

    /// Push an int value onto the operand stack.
    pub fn push_int(&self, value: i32) -> Result<()> {
        self.push(Value::Int(value))
    }

    /// Push a long value onto the operand stack.
    pub fn push_long(&self, value: i64) -> Result<()> {
        self.push(Value::Long(value))
    }

    /// Push a float value onto the operand stack.
    pub fn push_float(&self, value: f32) -> Result<()> {
        self.push(Value::Float(value))
    }

    /// Push a double value onto the operand stack.
    pub fn push_double(&self, value: f64) -> Result<()> {
        self.push(Value::Double(value))
    }

    /// Push a reference onto the operand stack.
    pub fn push_object(&self, value: Option<Reference>) -> Result<()> {
        self.push(Value::Object(value))
    }

    /// Pop a value from the operand stack.
    #[inline]
    pub fn pop(&self) -> Result<Value> {
        let Ok(Some(value)) = self.stack.pop() else {
            return Err(OperandStackUnderflow);
        };
        Ok(value)
    }

    /// Pop an int from the operand stack.
    pub fn pop_int(&self) -> Result<i32> {
        match self.pop()? {
            Value::Int(value) => Ok(value),
            value => Err(InvalidOperand {
                expected: "int".to_string(),
                actual: value.to_string(),
            }),
        }
    }

    /// Pop a long from the operand stack.
    pub fn pop_long(&self) -> Result<i64> {
        let value = match self.pop()? {
            Value::Long(value) => value,
            value => {
                return Err(InvalidOperand {
                    expected: "long".to_string(),
                    actual: value.to_string(),
                });
            }
        };
        Ok(value)
    }

    /// Pop a float from the operand stack.
    pub fn pop_float(&self) -> Result<f32> {
        match self.pop()? {
            Value::Float(value) => Ok(value),
            value => Err(InvalidOperand {
                expected: "float".to_string(),
                actual: value.to_string(),
            }),
        }
    }

    /// Pop a double from the operand stack.
    pub fn pop_double(&self) -> Result<f64> {
        let value = match self.pop()? {
            Value::Double(value) => value,
            value => {
                return Err(InvalidOperand {
                    expected: "double".to_string(),
                    actual: value.to_string(),
                });
            }
        };
        Ok(value)
    }

    /// Pop a null or object from the operand stack.
    pub fn pop_object(&self) -> Result<Option<Reference>> {
        let value = self.pop()?;
        match value {
            Value::Object(reference) => Ok(reference),
            value => Err(InvalidOperand {
                expected: "object".to_string(),
                actual: value.to_string(),
            }),
        }
    }

    /// Peek at the top value on the operand stack.
    pub fn peek(&self) -> Result<Value> {
        let Ok(Some(value)) = self.stack.pop() else {
            return Err(OperandStackUnderflow);
        };
        self.stack.push(value.clone())?;
        Ok(value)
    }

    /// Get the number of values on the operand stack.
    pub fn len(&self) -> Result<usize> {
        Ok(self.stack.len()?)
    }

    /// Check if the operand stack is empty.
    pub fn is_empty(&self) -> Result<bool> {
        Ok(self.stack.is_empty()?)
    }
}

impl Display for OperandStack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.stack.to_vec() {
            Ok(stack) => {
                let mut values = Vec::new();
                for stack_entry in &stack {
                    let value = stack_entry.to_string();
                    let chars: Vec<char> = value.chars().collect();
                    if chars.len() > 100 {
                        let value = chars.iter().take(97).collect::<String>();
                        values.push(format!("{value}..."));
                    } else {
                        values.push(value);
                    }
                }
                write!(f, "[{}]", values.join(", "))
            }
            Err(error) => {
                write!(f, "poisoned lock: {error}")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::operand_stack::OperandStack;
    use ristretto_classloader::{ConcurrentVec, Reference};

    #[test]
    fn test_can_push_and_pop_values() -> Result<()> {
        let stack = OperandStack::with_max_size(2);
        stack.push_int(1)?;
        stack.push_int(2)?;

        assert_eq!(stack.len()?, 2);

        assert_eq!(stack.pop()?, Value::Int(2));
        assert_eq!(stack.pop()?, Value::Int(1));
        Ok(())
    }

    #[test]
    fn test_pop_int() -> Result<()> {
        let stack = OperandStack::with_max_size(1);
        stack.push_int(42)?;
        assert_eq!(stack.pop_int()?, 42);
        Ok(())
    }

    #[test]
    fn test_pop_int_invalid_operand() -> Result<()> {
        let stack = OperandStack::with_max_size(1);
        stack.push_object(None)?;
        assert!(matches!(
            stack.pop_int(),
            Err(InvalidOperand {
                expected,
                actual
            }) if expected == "int" && actual == "Object(null)"
        ));
        Ok(())
    }

    #[test]
    fn test_pop_long() -> Result<()> {
        let stack = OperandStack::with_max_size(2);
        stack.push_long(42)?;
        assert_eq!(stack.pop_long()?, 42);
        Ok(())
    }

    #[test]
    fn test_pop_long_invalid_operand() -> Result<()> {
        let stack = OperandStack::with_max_size(2);
        stack.push_double(42.1)?;
        assert!(matches!(
            stack.pop_long(),
            Err(InvalidOperand {
                expected,
                actual
            }) if expected == "long" && actual == "double(42.1)"
        ));
        Ok(())
    }

    #[test]
    fn test_pop_float() -> Result<()> {
        let stack = OperandStack::with_max_size(1);
        stack.push_float(42.1)?;
        let value = stack.pop_float()? - 42.1f32;
        assert!(value.abs() < 0.1f32);
        Ok(())
    }

    #[test]
    fn test_pop_float_invalid_operand() -> Result<()> {
        let stack = OperandStack::with_max_size(1);
        stack.push_object(None)?;
        assert!(matches!(
            stack.pop_float(),
            Err(InvalidOperand {
                expected,
                actual
            }) if expected == "float" && actual == "Object(null)"
        ));
        Ok(())
    }

    #[test]
    fn test_pop_double() -> Result<()> {
        let stack = OperandStack::with_max_size(2);
        stack.push_double(42.1)?;
        let value = stack.pop_double()? - 42.1f64;
        assert!(value.abs() < 0.1f64);
        Ok(())
    }

    #[test]
    fn test_pop_double_invalid_operand() -> Result<()> {
        let stack = OperandStack::with_max_size(2);
        stack.push_long(42)?;
        assert!(matches!(
            stack.pop_double(),
            Err(InvalidOperand {
                expected,
                actual
            }) if expected == "double" && actual == "long(42)"
        ));
        Ok(())
    }

    #[test]
    fn test_pop_object() -> Result<()> {
        let stack = OperandStack::with_max_size(2);
        let object = Reference::ByteArray(ConcurrentVec::from(vec![42]));
        stack.push_object(None)?;
        stack.push_object(Some(object.clone()))?;
        assert_eq!(stack.pop_object()?, Some(object));
        assert_eq!(stack.pop_object()?, None);
        Ok(())
    }

    #[test]
    fn test_pop_object_invalid_operand() -> Result<()> {
        let stack = OperandStack::with_max_size(1);
        stack.push_int(42)?;
        assert!(matches!(
            stack.pop_object(),
            Err(InvalidOperand {
                expected,
                actual
            }) if expected == "object" && actual == "int(42)"
        ));
        Ok(())
    }

    #[test]
    fn test_pop_underflow() {
        let stack = OperandStack::with_max_size(1);
        let result = stack.pop();
        assert!(matches!(result, Err(OperandStackUnderflow)));
    }

    #[test]
    fn test_push_overflow() -> Result<()> {
        let stack = OperandStack::with_max_size(1);
        stack.push_int(42)?;
        let result = stack.push_int(43);
        assert!(matches!(result, Err(OperandStackOverflow)));
        Ok(())
    }

    #[test]
    fn test_peek_top_value() -> Result<()> {
        let stack = OperandStack::with_max_size(2);
        stack.push_int(1)?;
        stack.push_int(2)?;

        assert_eq!(stack.peek()?, Value::Int(2));
        assert_eq!(stack.len()?, 2);
        Ok(())
    }

    #[test]
    fn test_peek_underflow() {
        let stack = OperandStack::with_max_size(1);
        let result = stack.peek();
        assert!(matches!(result, Err(OperandStackUnderflow)));
    }

    #[test]
    fn test_is_empty() -> Result<()> {
        let stack = OperandStack::with_max_size(1);
        assert!(stack.is_empty()?);

        stack.push_int(42)?;
        assert!(!stack.is_empty()?);
        Ok(())
    }

    #[test]
    fn test_display() -> Result<()> {
        let stack = OperandStack::with_max_size(4);
        stack.push_int(1)?;
        stack.push_int(2)?;
        assert_eq!("[int(1), int(2)]", stack.to_string());
        Ok(())
    }
}
