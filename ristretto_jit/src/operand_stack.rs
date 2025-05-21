use crate::Error::InternalError;
use crate::Result;
use cranelift::codegen::ir::{BlockArg, Value};
use cranelift::frontend::FunctionBuilder;
use cranelift::prelude::{Type, types};

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

    /// Pushes a value of the specified type onto the stack.
    fn push_type(
        &mut self,
        function_builder: &mut FunctionBuilder,
        expected_type: Type,
        value: Value,
    ) -> Result<()> {
        let value_type = function_builder.func.dfg.value_type(value);
        if value_type != expected_type {
            return Err(InternalError(format!(
                "Expected {expected_type:?}, found {value_type:?}",
            )));
        }
        self.push(value)
    }

    /// Push an int value onto the operand stack.
    pub fn push_int(&mut self, function_builder: &mut FunctionBuilder, value: Value) -> Result<()> {
        self.push_type(function_builder, types::I32, value)
    }

    /// Push a long value onto the operand stack.
    pub fn push_long(
        &mut self,
        function_builder: &mut FunctionBuilder,
        value: Value,
    ) -> Result<()> {
        self.push_type(function_builder, types::I64, value)
    }

    /// Push a float value onto the operand stack.
    pub fn push_float(
        &mut self,
        function_builder: &mut FunctionBuilder,
        value: Value,
    ) -> Result<()> {
        self.push_type(function_builder, types::F32, value)
    }

    /// Push a double value onto the operand stack.
    pub fn push_double(
        &mut self,
        function_builder: &mut FunctionBuilder,
        value: Value,
    ) -> Result<()> {
        self.push_type(function_builder, types::F64, value)
    }

    /// Pops a value from the stack.
    pub fn pop(&mut self) -> Result<Value> {
        let Some(value) = self.stack.pop() else {
            return Err(InternalError("OperandStack underflow".to_string()));
        };
        Ok(value)
    }

    /// Pop a value of the specified type from the stack.
    fn pop_type(
        &mut self,
        function_builder: &mut FunctionBuilder,
        expected_type: Type,
    ) -> Result<Value> {
        let value = self.pop()?;
        let value_type = function_builder.func.dfg.value_type(value);
        if value_type != expected_type {
            return Err(InternalError(format!(
                "Expected {expected_type:?}, found {value_type:?}",
            )));
        }
        Ok(value)
    }

    /// Pop an int from the operand stack.
    pub fn pop_int(&mut self, function_builder: &mut FunctionBuilder) -> Result<Value> {
        self.pop_type(function_builder, types::I32)
    }

    /// Pop a long from the operand stack.
    pub fn pop_long(&mut self, function_builder: &mut FunctionBuilder) -> Result<Value> {
        self.pop_type(function_builder, types::I64)
    }

    /// Pop a float from the operand stack.
    pub fn pop_float(&mut self, function_builder: &mut FunctionBuilder) -> Result<Value> {
        self.pop_type(function_builder, types::F32)
    }

    /// Pop a double from the operand stack.
    pub fn pop_double(&mut self, function_builder: &mut FunctionBuilder) -> Result<Value> {
        self.pop_type(function_builder, types::F64)
    }

    /// Returns the number of values on the stack.
    pub fn len(&self) -> usize {
        self.stack.len()
    }

    /// Returns true if the stack is empty.
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    /// Returns the values on the stack
    pub fn as_slice(&self) -> &[Value] {
        &self.stack
    }

    /// Returns the values on the stack as block arguments.
    pub fn as_block_arguments(&self) -> Vec<BlockArg> {
        self.stack
            .iter()
            .map(|value| BlockArg::Value(*value))
            .collect::<Vec<BlockArg>>()
    }

    /// Returns a vector of types based on the values in the stack.
    pub fn to_type_vec(&self, function_builder: &FunctionBuilder) -> Vec<Type> {
        self.stack
            .iter()
            .map(|value| function_builder.func.dfg.value_type(*value))
            .collect()
    }

    /// Sets the stack to the given values.
    pub fn reset(&mut self, function_builder: &mut FunctionBuilder) -> Result<()> {
        let Some(current_block) = function_builder.current_block() else {
            return Err(InternalError("No current block".to_string()));
        };
        let values = function_builder.block_params(current_block);
        // Retain the capacity of the original stack
        self.stack.clear();
        self.stack.extend(values);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::create_function_builder_contexts;
    use cranelift::frontend::FunctionBuilder;
    use cranelift::prelude::InstBuilder;

    #[test]
    fn test_push_and_pop() -> Result<()> {
        let (mut module_context, mut function_context) = create_function_builder_contexts()?;
        let mut function_builder =
            FunctionBuilder::new(&mut module_context.func, &mut function_context);
        let block = function_builder.create_block();
        function_builder.switch_to_block(block);

        let mut operand_stack = OperandStack::with_capacity(1);
        let value = function_builder.ins().iconst(types::I32, 42);
        assert!(operand_stack.is_empty());
        operand_stack.push(value)?;
        assert_eq!(operand_stack.len(), 1);
        assert_eq!(operand_stack.pop()?, value);
        assert!(operand_stack.is_empty());
        Ok(())
    }

    #[test]
    fn test_push_overflow() -> Result<()> {
        let (mut module_context, mut function_context) = create_function_builder_contexts()?;
        let mut function_builder =
            FunctionBuilder::new(&mut module_context.func, &mut function_context);
        let block = function_builder.create_block();
        function_builder.switch_to_block(block);

        let mut operand_stack = OperandStack::with_capacity(1);
        let value = function_builder.ins().iconst(types::I32, 42);
        assert!(operand_stack.is_empty());
        operand_stack.push(value)?;
        assert_eq!(operand_stack.len(), 1);
        assert!(operand_stack.push(value).is_err());
        Ok(())
    }

    #[test]
    fn test_push_invalid_type() -> Result<()> {
        let (mut module_context, mut function_context) = create_function_builder_contexts()?;
        let mut function_builder =
            FunctionBuilder::new(&mut module_context.func, &mut function_context);
        let block = function_builder.create_block();
        function_builder.switch_to_block(block);

        let mut operand_stack = OperandStack::with_capacity(1);
        let value = function_builder.ins().iconst(types::I32, 42);
        assert!(operand_stack.is_empty());
        assert!(
            operand_stack
                .push_type(&mut function_builder, types::F32, value)
                .is_err()
        );
        Ok(())
    }

    #[test]
    fn test_push_and_pop_int() -> Result<()> {
        let (mut module_context, mut function_context) = create_function_builder_contexts()?;
        let mut function_builder =
            FunctionBuilder::new(&mut module_context.func, &mut function_context);
        let block = function_builder.create_block();
        function_builder.switch_to_block(block);

        let mut operand_stack = OperandStack::with_capacity(1);
        let value = function_builder.ins().iconst(types::I32, 42);
        assert!(operand_stack.is_empty());
        operand_stack.push_int(&mut function_builder, value)?;
        assert_eq!(operand_stack.len(), 1);
        assert_eq!(operand_stack.pop_int(&mut function_builder)?, value);
        assert!(operand_stack.is_empty());
        Ok(())
    }

    #[test]
    fn test_push_and_pop_long() -> Result<()> {
        let (mut module_context, mut function_context) = create_function_builder_contexts()?;
        let mut function_builder =
            FunctionBuilder::new(&mut module_context.func, &mut function_context);
        let block = function_builder.create_block();
        function_builder.switch_to_block(block);

        let mut operand_stack = OperandStack::with_capacity(1);
        let value = function_builder.ins().iconst(types::I64, 42);
        assert!(operand_stack.is_empty());
        operand_stack.push_long(&mut function_builder, value)?;
        assert_eq!(operand_stack.len(), 1);
        assert_eq!(operand_stack.pop_long(&mut function_builder)?, value);
        assert!(operand_stack.is_empty());
        Ok(())
    }

    #[test]
    fn test_push_and_pop_float() -> Result<()> {
        let (mut module_context, mut function_context) = create_function_builder_contexts()?;
        let mut function_builder =
            FunctionBuilder::new(&mut module_context.func, &mut function_context);
        let block = function_builder.create_block();
        function_builder.switch_to_block(block);

        let mut operand_stack = OperandStack::with_capacity(1);
        let value = function_builder.ins().f32const(42.1);
        assert!(operand_stack.is_empty());
        operand_stack.push_float(&mut function_builder, value)?;
        assert_eq!(operand_stack.len(), 1);
        assert_eq!(operand_stack.pop_float(&mut function_builder)?, value);
        assert!(operand_stack.is_empty());
        Ok(())
    }

    #[test]
    fn test_push_and_pop_double() -> Result<()> {
        let (mut module_context, mut function_context) = create_function_builder_contexts()?;
        let mut function_builder =
            FunctionBuilder::new(&mut module_context.func, &mut function_context);
        let block = function_builder.create_block();
        function_builder.switch_to_block(block);

        let mut operand_stack = OperandStack::with_capacity(1);
        let value = function_builder.ins().f64const(42.1);
        assert!(operand_stack.is_empty());
        operand_stack.push_double(&mut function_builder, value)?;
        assert_eq!(operand_stack.len(), 1);
        assert_eq!(operand_stack.pop_double(&mut function_builder)?, value);
        assert!(operand_stack.is_empty());
        Ok(())
    }

    #[test]
    fn test_pop_underflow() -> Result<()> {
        let (mut module_context, mut function_context) = create_function_builder_contexts()?;
        let mut function_builder =
            FunctionBuilder::new(&mut module_context.func, &mut function_context);
        let block = function_builder.create_block();
        function_builder.switch_to_block(block);

        let mut operand_stack = OperandStack::with_capacity(1);
        assert!(operand_stack.is_empty());
        assert!(operand_stack.pop().is_err());
        Ok(())
    }

    #[test]
    fn test_pop_invalid_type() -> Result<()> {
        let (mut module_context, mut function_context) = create_function_builder_contexts()?;
        let mut function_builder =
            FunctionBuilder::new(&mut module_context.func, &mut function_context);
        let block = function_builder.create_block();
        function_builder.switch_to_block(block);

        let mut operand_stack = OperandStack::with_capacity(1);
        let value = function_builder.ins().iconst(types::I32, 42);
        operand_stack.push(value)?;
        assert!(operand_stack.pop_long(&mut function_builder).is_err());
        Ok(())
    }

    #[test]
    fn test_as_slice() -> Result<()> {
        let (mut module_context, mut function_context) = create_function_builder_contexts()?;
        let mut function_builder =
            FunctionBuilder::new(&mut module_context.func, &mut function_context);
        let block = function_builder.create_block();
        function_builder.switch_to_block(block);

        let mut operand_stack = OperandStack::with_capacity(4);
        let int_value = function_builder.ins().iconst(types::I32, 42);
        let long_value = function_builder.ins().iconst(types::I64, 42);
        let float_value = function_builder.ins().f32const(42.1);
        let double_value = function_builder.ins().f64const(42.1);

        operand_stack.push(int_value)?;
        operand_stack.push(long_value)?;
        operand_stack.push(float_value)?;
        operand_stack.push(double_value)?;

        assert_eq!(
            operand_stack.as_slice(),
            &[int_value, long_value, float_value, double_value]
        );
        Ok(())
    }

    #[test]
    fn test_as_block_arguments() -> Result<()> {
        let (mut module_context, mut function_context) = create_function_builder_contexts()?;
        let mut function_builder =
            FunctionBuilder::new(&mut module_context.func, &mut function_context);
        let block = function_builder.create_block();
        function_builder.switch_to_block(block);

        let mut operand_stack = OperandStack::with_capacity(4);
        let int_value = function_builder.ins().iconst(types::I32, 42);
        let long_value = function_builder.ins().iconst(types::I64, 42);
        let float_value = function_builder.ins().f32const(42.1);
        let double_value = function_builder.ins().f64const(42.1);

        operand_stack.push(int_value)?;
        operand_stack.push(long_value)?;
        operand_stack.push(float_value)?;
        operand_stack.push(double_value)?;

        assert_eq!(
            operand_stack.as_block_arguments(),
            &[
                BlockArg::Value(int_value),
                BlockArg::Value(long_value),
                BlockArg::Value(float_value),
                BlockArg::Value(double_value),
            ]
        );
        Ok(())
    }

    #[test]
    fn test_to_type_vec() -> Result<()> {
        let (mut module_context, mut function_context) = create_function_builder_contexts()?;
        let mut function_builder =
            FunctionBuilder::new(&mut module_context.func, &mut function_context);
        let block = function_builder.create_block();
        function_builder.switch_to_block(block);

        let mut operand_stack = OperandStack::with_capacity(4);
        let int_value = function_builder.ins().iconst(types::I32, 42);
        let long_value = function_builder.ins().iconst(types::I64, 42);
        let float_value = function_builder.ins().f32const(42.1);
        let double_value = function_builder.ins().f64const(42.1);

        operand_stack.push(int_value)?;
        operand_stack.push(long_value)?;
        operand_stack.push(float_value)?;
        operand_stack.push(double_value)?;

        let types = operand_stack.to_type_vec(&function_builder);
        assert_eq!(types.len(), 4);
        assert_eq!(types, vec![types::I32, types::I64, types::F32, types::F64]);
        Ok(())
    }

    #[test]
    fn test_reset() -> Result<()> {
        let (mut module_context, mut function_context) = create_function_builder_contexts()?;
        let mut function_builder =
            FunctionBuilder::new(&mut module_context.func, &mut function_context);
        let block = function_builder.create_block();
        function_builder.switch_to_block(block);

        let mut operand_stack = OperandStack::with_capacity(1);
        let int_value = function_builder.ins().iconst(types::I32, 42);

        operand_stack.push(int_value)?;
        assert_eq!(operand_stack.len(), 1);
        operand_stack.reset(&mut function_builder)?;
        assert!(operand_stack.is_empty());
        Ok(())
    }

    #[test]
    fn test_reset_no_current_block_error() -> Result<()> {
        let (mut module_context, mut function_context) = create_function_builder_contexts()?;
        let mut function_builder =
            FunctionBuilder::new(&mut module_context.func, &mut function_context);
        let mut operand_stack = OperandStack::with_capacity(1);
        assert!(operand_stack.reset(&mut function_builder).is_err());
        Ok(())
    }
}
