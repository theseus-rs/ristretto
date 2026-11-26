use crate::Error::{InternalError, InvalidLocalVariableIndex};
use crate::Result;
use crate::control_flow_graph::StackType;
use ahash::AHashMap;
use cranelift::frontend::FunctionBuilder;
use cranelift::prelude::{InstBuilder, Value, Variable, types};

/// Represents the local variables in a frame.
#[derive(Clone, Debug)]
pub struct LocalVariables {
    // A JVM slot may be reused for unrelated types. Keep distinct SSA variables
    // even when the machine types coincide (reference and long both use I64).
    variables: AHashMap<(usize, StackType), Variable>,
}

impl LocalVariables {
    /// Stable cell order for saving/restoring typed SSA locals between native slices.
    pub(crate) fn continuation_variables(&self) -> Vec<(Variable, cranelift::prelude::Type)> {
        let mut variables: Vec<_> = self
            .variables
            .iter()
            .map(|((_, kind), variable)| (*variable, kind.machine_type()))
            .collect();
        variables.sort_unstable_by_key(|(variable, _)| variable.as_u32());
        variables
    }

    /// Create a new local variables
    pub fn new() -> Self {
        LocalVariables {
            variables: AHashMap::default(),
        }
    }

    pub(crate) fn reference_indices(&self) -> Vec<usize> {
        let mut indices = self
            .variables
            .keys()
            .filter_map(|(index, kind)| (*kind == StackType::Reference).then_some(*index))
            .collect::<Vec<_>>();
        indices.sort_unstable();
        indices
    }

    /// Declare one typed use of a Java local slot. Zero initialization gives
    /// Cranelift complete SSA definitions; bytecode verification determines which
    /// local reads are legal. This is not a Java initialization/liveness analysis.
    pub(crate) fn declare(
        &mut self,
        function_builder: &mut FunctionBuilder,
        index: usize,
        kind: StackType,
    ) -> Variable {
        *self.variables.entry((index, kind)).or_insert_with(|| {
            let machine = kind.machine_type();
            let variable = function_builder.declare_var(machine);
            let zero = match machine {
                types::F32 => function_builder.ins().f32const(0.0),
                types::F64 => function_builder.ins().f64const(0.0),
                _ => function_builder.ins().iconst(machine, 0),
            };
            function_builder.def_var(variable, zero);
            variable
        })
    }

    fn get_type(
        &self,
        function_builder: &mut FunctionBuilder,
        expected_type: impl Into<StackType>,
        index: usize,
    ) -> Result<Value> {
        let kind = expected_type.into();
        let variable = *self
            .variables
            .get(&(index, kind))
            .ok_or(InvalidLocalVariableIndex(index))?;
        function_builder
            .try_use_var(variable)
            .map_err(|error| InternalError(error.to_string()))
    }

    /// Get an int from the local variables.
    ///
    /// # Errors
    ///
    /// if the local variable at the given index was not found or if the value is not an int.
    pub fn get_int(&self, function_builder: &mut FunctionBuilder, index: usize) -> Result<Value> {
        self.get_type(function_builder, types::I32, index)
    }

    /// Get a long from the local variables.
    ///
    /// # Errors
    ///
    /// if the local variable at the given index was not found or if the value is not a long.
    pub fn get_long(&self, function_builder: &mut FunctionBuilder, index: usize) -> Result<Value> {
        self.get_type(function_builder, types::I64, index)
    }

    /// Get a float from the local variables.
    ///
    /// # Errors
    ///
    /// if the local variable at the given index was not found or if the value is not a float.
    pub fn get_float(&self, function_builder: &mut FunctionBuilder, index: usize) -> Result<Value> {
        self.get_type(function_builder, types::F32, index)
    }

    /// Get a double from the local variables.
    ///
    /// # Errors
    ///
    /// if the local variable at the given index was not found or if the value is not a double.
    pub fn get_double(
        &self,
        function_builder: &mut FunctionBuilder,
        index: usize,
    ) -> Result<Value> {
        self.get_type(function_builder, types::F64, index)
    }

    /// Get an object reference (pointer) from the local variables.
    ///
    /// # Errors
    ///
    /// if the local variable at the given index was not found or if the value is not a pointer.
    pub fn get_object(
        &self,
        function_builder: &mut FunctionBuilder,
        index: usize,
    ) -> Result<Value> {
        self.get_type(function_builder, StackType::Reference, index)
    }

    fn set_type(
        &mut self,
        function_builder: &mut FunctionBuilder,
        index: usize,
        expected_type: impl Into<StackType>,
        value: Value,
    ) -> Result<()> {
        let kind = expected_type.into();
        let value_type = function_builder.func.dfg.value_type(value);
        if value_type != kind.machine_type() {
            return Err(InternalError(format!(
                "Expected {kind:?}, found {value_type:?}"
            )));
        }
        let variable = *self
            .variables
            .get(&(index, kind))
            .ok_or(InvalidLocalVariableIndex(index))?;
        function_builder
            .try_def_var(variable, value)
            .map_err(|error| InternalError(error.to_string()))
    }

    /// Set an int in the local variables.
    ///
    /// # Errors
    ///
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
    ///
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
    ///
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
    ///
    /// if the index is out of bounds or if the value is not a double.
    pub fn set_double(
        &mut self,
        function_builder: &mut FunctionBuilder,
        index: usize,
        value: Value,
    ) -> Result<()> {
        self.set_type(function_builder, index, types::F64, value)
    }

    /// Set an object reference (pointer) in the local variables.
    ///
    /// # Errors
    ///
    /// if the index is out of bounds or if the value is not a pointer.
    pub fn set_object(
        &mut self,
        function_builder: &mut FunctionBuilder,
        index: usize,
        value: Value,
    ) -> Result<()> {
        self.set_type(function_builder, index, StackType::Reference, value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::create_function_builder_contexts;
    use cranelift::prelude::Type;

    #[test]
    fn references_and_longs_use_distinct_ssa_variables() -> Result<()> {
        let (mut module, mut context) = create_function_builder_contexts()?;
        let mut builder = FunctionBuilder::new(&mut module.func, &mut context);
        let block = builder.create_block();
        builder.switch_to_block(block);
        let mut locals = LocalVariables::new();
        let reference = locals.declare(&mut builder, 0, StackType::Reference);
        let long = locals.declare(&mut builder, 0, types::I64.into());
        assert_ne!(reference, long);
        let object_value = builder.ins().iconst(types::I64, 0);
        let long_value = builder.ins().iconst(types::I64, 42);
        locals.set_object(&mut builder, 0, object_value)?;
        locals.set_long(&mut builder, 0, long_value)?;
        assert_eq!(locals.get_object(&mut builder, 0)?, object_value);
        assert_eq!(locals.get_long(&mut builder, 0)?, long_value);
        Ok(())
    }

    fn create_locals(
        function_builder: &mut FunctionBuilder,
        number_of_locals: usize,
        local_type: Type,
    ) -> LocalVariables {
        let mut locals = LocalVariables::new();
        for index in 0..number_of_locals {
            locals.declare(function_builder, index, local_type.into());
        }
        locals
    }

    #[test]
    fn test_get_and_set() -> Result<()> {
        let (mut module_context, mut function_context) = create_function_builder_contexts()?;
        let mut function_builder =
            FunctionBuilder::new(&mut module_context.func, &mut function_context);
        let block = function_builder.create_block();
        function_builder.switch_to_block(block);
        let mut local_variables = create_locals(&mut function_builder, 1, types::I32);
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
        let mut local_variables = create_locals(&mut function_builder, 1, types::I32);
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
        let mut local_variables = create_locals(&mut function_builder, 1, types::I64);
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
        let mut local_variables = create_locals(&mut function_builder, 1, types::F32);
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
        let mut local_variables = create_locals(&mut function_builder, 1, types::F64);
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
        let mut local_variables = create_locals(&mut function_builder, 1, types::I32);
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
        let mut local_variables = create_locals(&mut function_builder, 1, types::I32);
        let index = 0;
        let value = function_builder.ins().iconst(types::I32, 42);
        local_variables.set_int(&mut function_builder, index, value)?;
        let result = local_variables.set_long(&mut function_builder, index, value);
        assert!(result.is_err());
        Ok(())
    }
}
