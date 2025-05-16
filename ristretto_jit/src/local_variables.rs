use crate::Error::{InternalError, InvalidLocalVariableIndex};
use crate::Result;
use cranelift::frontend::FunctionBuilder;
use cranelift::prelude::{EntityRef, Type, Value, Variable, types};

/// Represents the local variables in a frame.
#[derive(Clone, Debug)]
pub struct LocalVariables {}

impl LocalVariables {
    /// Create a new local variables
    pub fn new() -> Self {
        LocalVariables {}
    }

    /// Get a value from the local variables.
    ///
    /// # Errors
    /// if the local variable at the given index was not found.
    #[expect(clippy::unused_self)]
    pub fn get(&self, function_builder: &mut FunctionBuilder, index: usize) -> Result<Value> {
        let variable = Variable::new(index);
        let Ok(value) = function_builder.try_use_var(variable) else {
            return Err(InvalidLocalVariableIndex(index));
        };
        Ok(value)
    }

    /// Get a type from the local variables.
    ///
    /// # Errors
    /// if the local variable at the given index was not found or if the value is not an int.
    fn get_type(
        &self,
        function_builder: &mut FunctionBuilder,
        expected_type: Type,
        index: usize,
    ) -> Result<Value> {
        let value = self.get(function_builder, index)?;
        let value_type = function_builder.func.dfg.value_type(value);
        if value_type != expected_type {
            return Err(InternalError(format!(
                "Expected {expected_type:?}, found {value_type:?}",
            )));
        }
        Ok(value)
    }

    /// Get an int from the local variables.
    ///
    /// # Errors
    /// if the local variable at the given index was not found or if the value is not an int.
    pub fn get_int(&self, function_builder: &mut FunctionBuilder, index: usize) -> Result<Value> {
        self.get_type(function_builder, types::I32, index)
    }

    /// Get a long from the local variables.
    ///
    /// # Errors
    /// if the local variable at the given index was not found or if the value is not a long.
    pub fn get_long(&self, function_builder: &mut FunctionBuilder, index: usize) -> Result<Value> {
        self.get_type(function_builder, types::I64, index)
    }

    /// Get a float from the local variables.
    ///
    /// # Errors
    /// if the local variable at the given index was not found or if the value is not a float.
    pub fn get_float(&self, function_builder: &mut FunctionBuilder, index: usize) -> Result<Value> {
        self.get_type(function_builder, types::F32, index)
    }

    /// Get a double from the local variables.
    ///
    /// # Errors
    /// if the local variable at the given index was not found or if the value is not a double.
    pub fn get_double(
        &self,
        function_builder: &mut FunctionBuilder,
        index: usize,
    ) -> Result<Value> {
        self.get_type(function_builder, types::F64, index)
    }

    /// Set a value in the local variables.
    ///
    /// # Errors
    /// if the index is out of bounds.
    #[expect(clippy::unused_self)]
    pub fn set(
        &mut self,
        function_builder: &mut FunctionBuilder,
        index: usize,
        value_type: Type,
        value: Value,
    ) -> Result<()> {
        let variable = Variable::new(index);
        if function_builder.try_def_var(variable, value).is_err() {
            function_builder
                .try_declare_var(variable, value_type)
                .map_err(|error| InternalError(error.to_string()))?;
            function_builder
                .try_def_var(variable, value)
                .map_err(|error| InternalError(error.to_string()))?;
        }
        Ok(())
    }

    /// Set an expected type in the local variables.
    ///
    /// # Errors
    /// if the index is out of bounds or if the value is not an int.
    fn set_type(
        &mut self,
        function_builder: &mut FunctionBuilder,
        index: usize,
        expected_type: Type,
        value: Value,
    ) -> Result<()> {
        let value_type = function_builder.func.dfg.value_type(value);
        if value_type != expected_type {
            return Err(InternalError(format!(
                "Expected {expected_type:?}, found {value_type:?}",
            )));
        }
        self.set(function_builder, index, expected_type, value)
    }

    /// Set an int in the local variables.
    ///
    /// # Errors
    /// if the index is out of bounds or if the value is not an int.
    pub fn set_int(
        &mut self,
        function_builder: &mut FunctionBuilder,
        index: usize,
        value: Value,
    ) -> Result<()> {
        self.set_type(function_builder, index, types::I32, value)
    }

    /// Set a long in the local variables.
    ///
    /// # Errors
    /// if the index is out of bounds or if the value is not a long.
    pub fn set_long(
        &mut self,
        function_builder: &mut FunctionBuilder,
        index: usize,
        value: Value,
    ) -> Result<()> {
        self.set_type(function_builder, index, types::I64, value)
    }

    /// Set a float in the local variables.
    ///
    /// # Errors
    /// if the index is out of bounds or if the value is not a float.
    pub fn set_float(
        &mut self,
        function_builder: &mut FunctionBuilder,
        index: usize,
        value: Value,
    ) -> Result<()> {
        self.set_type(function_builder, index, types::F32, value)
    }

    /// Set a double in the local variables.
    ///
    /// # Errors
    /// if the index is out of bounds or if the value is not a double.
    pub fn set_double(
        &mut self,
        function_builder: &mut FunctionBuilder,
        index: usize,
        value: Value,
    ) -> Result<()> {
        self.set_type(function_builder, index, types::F64, value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::create_function_builder_contexts;
    use cranelift::prelude::InstBuilder;

    #[test]
    fn test_get_and_set() -> Result<()> {
        let (mut module_context, mut function_context) = create_function_builder_contexts()?;
        let mut function_builder =
            FunctionBuilder::new(&mut module_context.func, &mut function_context);
        let block = function_builder.create_block();
        function_builder.switch_to_block(block);

        let mut local_variables = LocalVariables::new();
        let index = 0;
        let value = function_builder.ins().iconst(types::I32, 42);
        local_variables.set_int(&mut function_builder, index, value)?;
        let result = local_variables.get_int(&mut function_builder, index)?;
        assert_eq!(result, value);
        Ok(())
    }

    #[test]
    fn test_get_error() -> Result<()> {
        let (mut module_context, mut function_context) = create_function_builder_contexts()?;
        let mut function_builder =
            FunctionBuilder::new(&mut module_context.func, &mut function_context);
        let block = function_builder.create_block();
        function_builder.switch_to_block(block);

        let local_variables = LocalVariables::new();
        let result = local_variables.get_int(&mut function_builder, 0);
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_get_and_set_int() -> Result<()> {
        let (mut module_context, mut function_context) = create_function_builder_contexts()?;
        let mut function_builder =
            FunctionBuilder::new(&mut module_context.func, &mut function_context);
        let block = function_builder.create_block();
        function_builder.switch_to_block(block);

        let mut local_variables = LocalVariables::new();
        let index = 0;
        let value = function_builder.ins().iconst(types::I32, 42);
        local_variables.set_int(&mut function_builder, index, value)?;
        let result = local_variables.get_int(&mut function_builder, index)?;
        assert_eq!(result, value);
        Ok(())
    }

    #[test]
    fn test_get_and_set_long() -> Result<()> {
        let (mut module_context, mut function_context) = create_function_builder_contexts()?;
        let mut function_builder =
            FunctionBuilder::new(&mut module_context.func, &mut function_context);
        let block = function_builder.create_block();
        function_builder.switch_to_block(block);

        let mut local_variables = LocalVariables::new();
        let index = 0;
        let value = function_builder.ins().iconst(types::I64, 42);
        local_variables.set_long(&mut function_builder, index, value)?;
        let result = local_variables.get_long(&mut function_builder, index)?;
        assert_eq!(result, value);
        Ok(())
    }

    #[test]
    fn test_get_and_set_float() -> Result<()> {
        let (mut module_context, mut function_context) = create_function_builder_contexts()?;
        let mut function_builder =
            FunctionBuilder::new(&mut module_context.func, &mut function_context);
        let block = function_builder.create_block();
        function_builder.switch_to_block(block);

        let mut local_variables = LocalVariables::new();
        let index = 0;
        let value = function_builder.ins().f32const(42.1);
        local_variables.set_float(&mut function_builder, index, value)?;
        let result = local_variables.get_float(&mut function_builder, index)?;
        assert_eq!(result, value);
        Ok(())
    }

    #[test]
    fn test_get_and_set_double() -> Result<()> {
        let (mut module_context, mut function_context) = create_function_builder_contexts()?;
        let mut function_builder =
            FunctionBuilder::new(&mut module_context.func, &mut function_context);
        let block = function_builder.create_block();
        function_builder.switch_to_block(block);

        let mut local_variables = LocalVariables::new();
        let index = 0;
        let value = function_builder.ins().f64const(42.1);
        local_variables.set_double(&mut function_builder, index, value)?;
        let result = local_variables.get_double(&mut function_builder, index)?;
        assert_eq!(result, value);
        Ok(())
    }

    #[test]
    fn test_get_invalid() -> Result<()> {
        let (mut module_context, mut function_context) = create_function_builder_contexts()?;
        let mut function_builder =
            FunctionBuilder::new(&mut module_context.func, &mut function_context);
        let block = function_builder.create_block();
        function_builder.switch_to_block(block);

        let mut local_variables = LocalVariables::new();
        let index = 0;
        let value = function_builder.ins().iconst(types::I32, 42);
        local_variables.set_int(&mut function_builder, index, value)?;
        let result = local_variables.get_long(&mut function_builder, index);
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn test_set_invalid() -> Result<()> {
        let (mut module_context, mut function_context) = create_function_builder_contexts()?;
        let mut function_builder =
            FunctionBuilder::new(&mut module_context.func, &mut function_context);
        let block = function_builder.create_block();
        function_builder.switch_to_block(block);

        let mut local_variables = LocalVariables::new();
        let index = 0;
        let value = function_builder.ins().iconst(types::I32, 42);
        local_variables.set_int(&mut function_builder, index, value)?;
        let result = local_variables.set_long(&mut function_builder, index, value);
        assert!(result.is_err());
        Ok(())
    }
}
