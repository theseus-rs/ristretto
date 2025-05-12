use crate::Error::InternalError;
use crate::Result;
use cranelift::codegen::ir::Value;
use cranelift::frontend::FunctionBuilder;
use cranelift::prelude::types;

const POINTER_SIZE: i32 = 8;

/// Operand stack for the JIT compiler.
pub struct OperandStack {
    stack: Vec<Value>,
}

impl OperandStack {
    /// Creates a new operand stack with the specified capacity.
    pub fn with_capacity(capacity: u16) -> Self {
        OperandStack {
            stack: Vec::with_capacity(usize::from(capacity)),
        }
    }

    /// Pushes a value onto the stack.
    pub fn push(&mut self, value: Value) -> Result<()> {
        if self.stack.len() >= self.stack.capacity() {
            return Err(InternalError("OperandStack overflow".to_string()));
        }
        self.stack.push(value);
        Ok(())
    }

    /// Push an int value onto the operand stack.
    pub fn push_int(&mut self, function_builder: &mut FunctionBuilder, value: Value) -> Result<()> {
        let expected_type = types::I32;
        let value_type = function_builder.func.dfg.value_type(value);
        if value_type != expected_type {
            return Err(InternalError(format!(
                "Expected {expected_type:?}, found {value_type:?}",
            )));
        }
        self.push(value)
    }

    /// Push a long value onto the operand stack.
    pub fn push_long(
        &mut self,
        function_builder: &mut FunctionBuilder,
        value: Value,
    ) -> Result<()> {
        let expected_type = types::I64;
        let value_type = function_builder.func.dfg.value_type(value);
        if value_type != expected_type {
            return Err(InternalError(format!(
                "Expected {expected_type:?}, found {value_type:?}",
            )));
        }
        self.push(value)
    }

    /// Push a float value onto the operand stack.
    pub fn push_float(
        &mut self,
        function_builder: &mut FunctionBuilder,
        value: Value,
    ) -> Result<()> {
        let expected_type = types::F32;
        let value_type = function_builder.func.dfg.value_type(value);
        if value_type != expected_type {
            return Err(InternalError(format!(
                "Expected {expected_type:?}, found {value_type:?}",
            )));
        }
        self.push(value)
    }

    /// Push a double value onto the operand stack.
    pub fn push_double(
        &mut self,
        function_builder: &mut FunctionBuilder,
        value: Value,
    ) -> Result<()> {
        let expected_type = types::F64;
        let value_type = function_builder.func.dfg.value_type(value);
        if value_type != expected_type {
            return Err(InternalError(format!(
                "Expected {expected_type:?}, found {value_type:?}",
            )));
        }
        self.push(value)
    }

    /// Pops a value from the stack.
    pub fn pop(&mut self) -> Result<Value> {
        let Some(value) = self.stack.pop() else {
            return Err(InternalError("OperandStack underflow".to_string()));
        };
        Ok(value)
    }

    /// Pop an int from the operand stack.
    pub fn pop_int(&mut self, function_builder: &mut FunctionBuilder) -> Result<Value> {
        let expected_type = types::I32;
        let value = self.pop()?;
        let value_type = function_builder.func.dfg.value_type(value);
        if value_type != expected_type {
            return Err(InternalError(format!(
                "Expected {expected_type:?}, found {value_type:?}",
            )));
        }
        Ok(value)
    }

    /// Pop a long from the operand stack.
    pub fn pop_long(&mut self, function_builder: &mut FunctionBuilder) -> Result<Value> {
        let expected_type = types::I64;
        let value = self.pop()?;
        let value_type = function_builder.func.dfg.value_type(value);
        if value_type != expected_type {
            return Err(InternalError(format!(
                "Expected {expected_type:?}, found {value_type:?}",
            )));
        }
        Ok(value)
    }

    /// Pop a float from the operand stack.
    pub fn pop_float(&mut self, function_builder: &mut FunctionBuilder) -> Result<Value> {
        let expected_type = types::F32;
        let value = self.pop()?;
        let value_type = function_builder.func.dfg.value_type(value);
        if value_type != expected_type {
            return Err(InternalError(format!(
                "Expected {expected_type:?}, found {value_type:?}",
            )));
        }
        Ok(value)
    }

    /// Pop a double from the operand stack.
    pub fn pop_double(&mut self, function_builder: &mut FunctionBuilder) -> Result<Value> {
        let expected_type = types::F64;
        let value = self.pop()?;
        let value_type = function_builder.func.dfg.value_type(value);
        if value_type != expected_type {
            return Err(InternalError(format!(
                "Expected {expected_type:?}, found {value_type:?}",
            )));
        }
        Ok(value)
    }
}
