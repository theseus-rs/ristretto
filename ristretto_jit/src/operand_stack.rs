use crate::Error::InternalError;
use crate::Result;
use cranelift::codegen::ir::{StackSlot, Value};
use cranelift::frontend::FunctionBuilder;
use cranelift::prelude::{InstBuilder, StackSlotData, StackSlotKind, Type, types};

const POINTER_SIZE: i32 = 8;

/// Operand stack for the JIT compiler.
///
/// The stack is implemented as a continuous array of 64bit/8byte values.  I32 and F32 values are
/// padded to 8 bytes.  The stack is aligned to 8 bytes for performance reasons.
///
/// ```text
/// |------- 8 bytes -------|----- 8 bytes -----|------- 8 bytes -------|----- 8 bytes -----|
/// |- 4 bytes -|- 4 bytes -|                   |- 4 bytes -|- 4 bytes -|                   |
/// |  padding  |    I32    |        I64        |  padding  |    F32    |        F64        |
/// ```
pub struct OperandStack {
    slot: Option<StackSlot>,
    length: i32,
}

impl OperandStack {
    /// Creates a new operand stack with the specified capacity.
    pub fn with_capacity(function_builder: &mut FunctionBuilder, capacity: u16) -> Result<Self> {
        // Empty methods such as e.g. Object.<init>() will have a capacity of 0. Do not create a
        // stack slot in this case to improve performance.
        let slot = if capacity == 0 {
            None
        } else {
            let pointer_size = u32::try_from(POINTER_SIZE)?;
            let stack_capacity = u32::from(capacity).saturating_mul(pointer_size);
            let stack_slot_data =
                StackSlotData::new(StackSlotKind::ExplicitSlot, stack_capacity, 0);
            let slot = function_builder.create_sized_stack_slot(stack_slot_data);
            Some(slot)
        };
        let operand_stack = OperandStack { slot, length: 0 };
        Ok(operand_stack)
    }

    /// Pushes a value onto the stack.
    fn push_type(
        &mut self,
        function_builder: &mut FunctionBuilder,
        value_type: Type,
        value: Value,
    ) -> Result<()> {
        let Some(slot) = self.slot else {
            return Err(InternalError(
                "OperandStack.slot is not initialized".to_string(),
            ));
        };
        let index = self.length;
        self.length = index + 1;
        let slot_index = index.saturating_mul(POINTER_SIZE);
        let value_type_bytes = value_type.bytes();
        let padding = POINTER_SIZE.saturating_sub_unsigned(value_type_bytes);
        let slot_index = slot_index.saturating_add(padding);
        function_builder.ins().stack_store(value, slot, slot_index);
        Ok(())
    }

    /// Pushes a value onto the stack.
    pub fn push(&mut self, function_builder: &mut FunctionBuilder, value: Value) -> Result<()> {
        // Default to I64/8-bytes.
        self.push_type(function_builder, types::I64, value)
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
    fn pop_type(
        &mut self,
        function_builder: &mut FunctionBuilder,
        value_type: Type,
    ) -> Result<Value> {
        let Some(slot) = self.slot else {
            return Err(InternalError(
                "OperandStack.slot is not initialized".to_string(),
            ));
        };
        let index = self.length - 1;
        let slot_index = index.saturating_mul(POINTER_SIZE);
        let value_type_bytes = value_type.bytes();
        let padding = POINTER_SIZE.saturating_sub_unsigned(value_type_bytes);
        let slot_index = slot_index.saturating_add(padding);
        let value = function_builder
            .ins()
            .stack_load(value_type, slot, slot_index);
        self.length = index;
        Ok(value)
    }

    /// Pops a value from the stack.
    pub fn pop(&mut self, function_builder: &mut FunctionBuilder) -> Result<Value> {
        // Default to I64/8-bytes.
        self.pop_type(function_builder, types::I64)
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
}
