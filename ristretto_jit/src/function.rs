use crate::{JitValue, Result, Value};

/// A structure representing a native function from the JIT compiler
#[derive(Clone, Debug)]
pub struct Function {
    function: fn(*const JitValue, usize, *mut JitValue),
}

impl Function {
    /// Creates a new function instance
    pub(crate) fn new(function: fn(*const JitValue, usize, *mut JitValue)) -> Self {
        Self { function }
    }

    /// Calls the function with the given arguments
    ///
    /// # Errors
    ///
    /// if the function fails to execute
    pub fn execute(&self, arguments: Vec<Value>) -> Result<Option<Value>> {
        let arguments = arguments
            .into_iter()
            .map(JitValue::from)
            .collect::<Vec<_>>();
        let mut result = JitValue::new();
        (self.function)(arguments.as_ptr(), arguments.len(), &raw mut result);
        let value: Result<Option<Value>> = result.try_into();
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::JitValue;

    #[test]
    fn test_function() -> Result<()> {
        let function = Function::new(|_, _, jit_value| unsafe {
            *jit_value = JitValue::from(42i64);
        });
        let value = function.execute(Vec::new())?.expect("value");
        assert_eq!(value, Value::I64(42));
        Ok(())
    }
}
