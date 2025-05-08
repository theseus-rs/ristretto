use crate::Error::{InternalError, InvalidLocalVariableIndex};
use crate::Result;
use cranelift::frontend::FunctionBuilder;
use cranelift::prelude::{EntityRef, Type, Value, Variable, types};

/// Represents the local variables in a frame.
#[derive(Clone, Debug)]
pub struct LocalVariables {}

impl LocalVariables {
    /// Create a new local variables
    pub fn new(_locals: Vec<Value>) -> Self {
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

    /// Get an int from the local variables.
    ///
    /// # Errors
    /// if the local variable at the given index was not found or if the value is not an int.
    pub fn get_int(&self, function_builder: &mut FunctionBuilder, index: usize) -> Result<Value> {
        self.get(function_builder, index)
    }

    /// Get a long from the local variables.
    ///
    /// # Errors
    /// if the local variable at the given index was not found or if the value is not a long.
    pub fn get_long(&self, function_builder: &mut FunctionBuilder, index: usize) -> Result<Value> {
        self.get(function_builder, index)
    }

    /// Get a float from the local variables.
    ///
    /// # Errors
    /// if the local variable at the given index was not found or if the value is not a float.
    pub fn get_float(&self, function_builder: &mut FunctionBuilder, index: usize) -> Result<Value> {
        self.get(function_builder, index)
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
        self.get(function_builder, index)
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
        ty: Type,
        value: Value,
    ) -> Result<()> {
        let variable = Variable::new(index);
        if function_builder.try_def_var(variable, value).is_err() {
            function_builder
                .try_declare_var(variable, ty)
                .map_err(|error| InternalError(error.to_string()))?;
            function_builder
                .try_def_var(variable, value)
                .map_err(|error| InternalError(error.to_string()))?;
        }
        Ok(())
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
        self.set(function_builder, index, types::I32, value)
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
        self.set(function_builder, index, types::I64, value)
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
        self.set(function_builder, index, types::F32, value)
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
        self.set(function_builder, index, types::F64, value)
    }
}
