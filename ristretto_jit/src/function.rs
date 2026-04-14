use crate::{JitValue, Result, Value};

/// A structure representing a native function from the JIT compiler
#[derive(Clone, Debug)]
pub struct Function {
    function: fn(*const JitValue, usize, *mut JitValue, *const u8),
}

impl Function {
    /// Creates a new function instance
    pub(crate) fn new(function: fn(*const JitValue, usize, *mut JitValue, *const u8)) -> Self {
        Self { function }
    }

    /// Calls the function with the given arguments and a runtime context pointer.
    ///
    /// The `context` pointer is passed to the compiled function as the 4th argument.
    /// It typically points to a runtime context struct (e.g., the garbage collector)
    /// needed by runtime helper functions called from JIT-compiled code.
    ///
    /// # Errors
    ///
    /// if the function fails to execute
    pub fn execute(&self, arguments: &[Value], context: *const u8) -> Result<Option<Value>> {
        // Use a stack-allocated buffer for small argument lists (most JVM methods have 0-5 args)
        let mut stack_buf = [JitValue::new(); 8];
        let mut result = JitValue::new();
        if arguments.len() <= stack_buf.len() {
            for (i, arg) in arguments.iter().enumerate() {
                stack_buf[i] = JitValue::from(arg.clone());
            }
            (self.function)(
                stack_buf.as_ptr(),
                arguments.len(),
                &raw mut result,
                context,
            );
        } else {
            let heap_args: Vec<JitValue> = arguments.iter().cloned().map(JitValue::from).collect();
            (self.function)(
                heap_args.as_ptr(),
                heap_args.len(),
                &raw mut result,
                context,
            );
        }
        result.try_into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::JitValue;

    #[test]
    fn test_function() -> Result<()> {
        let function = Function::new(|_, _, jit_value, _| unsafe {
            *jit_value = JitValue::from(42i64);
        });
        let value = function.execute(&[], std::ptr::null())?.expect("value");
        assert_eq!(value, Value::I64(42));
        Ok(())
    }
}
