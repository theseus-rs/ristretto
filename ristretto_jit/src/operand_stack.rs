use crate::Error::InternalError;
use crate::Result;
use cranelift::codegen::ir::{StackSlot, Value};
use cranelift::frontend::FunctionBuilder;
use cranelift::prelude::{InstBuilder, StackSlotData, StackSlotKind, types};

const POINTER_SIZE: i32 = 8;

/// Operand stack for the JIT compiler.
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
    pub fn push(&mut self, function_builder: &mut FunctionBuilder, value: Value) -> Result<()> {
        let Some(slot) = self.slot else {
            return Err(InternalError(
                "OperandStack.slot is not initialized".to_string(),
            ));
        };
        let index = self.length;
        self.length = index + 1;
        let slot_index = index.saturating_mul(POINTER_SIZE);
        function_builder.ins().stack_store(value, slot, slot_index);
        Ok(())
    }

    /// Push an int value onto the operand stack.
    pub fn push_int(&mut self, function_builder: &mut FunctionBuilder, value: Value) -> Result<()> {
        let value = function_builder.ins().sextend(types::I64, value);
        self.push(function_builder, value)
    }

    /// Push a long value onto the operand stack.
    pub fn push_long(
        &mut self,
        function_builder: &mut FunctionBuilder,
        value: Value,
    ) -> Result<()> {
        self.push(function_builder, value)
    }

    /// Push a float value onto the operand stack.
    pub fn push_float(
        &mut self,
        function_builder: &mut FunctionBuilder,
        value: Value,
    ) -> Result<()> {
        let value = function_builder.ins().fcvt_to_sint(types::I64, value);
        self.push(function_builder, value)
    }

    /// Push a double value onto the operand stack.
    pub fn push_double(
        &mut self,
        function_builder: &mut FunctionBuilder,
        value: Value,
    ) -> Result<()> {
        let value = function_builder.ins().fcvt_to_sint(types::I64, value);
        self.push(function_builder, value)
    }

    /// Pops a value from the stack.
    pub fn pop(&mut self, function_builder: &mut FunctionBuilder) -> Result<Value> {
        let Some(slot) = self.slot else {
            return Err(InternalError(
                "OperandStack.slot is not initialized".to_string(),
            ));
        };
        let index = self.length - 1;
        let slot_index = index.saturating_mul(POINTER_SIZE);
        let value = function_builder
            .ins()
            .stack_load(types::I64, slot, slot_index);
        self.length = index;
        Ok(value)
    }

    /// Pop an int from the operand stack.
    pub fn pop_int(&mut self, function_builder: &mut FunctionBuilder) -> Result<Value> {
        let value = self.pop(function_builder)?;
        let value = function_builder.ins().ireduce(types::I32, value);
        Ok(value)
    }

    /// Pop a long from the operand stack.
    pub fn pop_long(&mut self, function_builder: &mut FunctionBuilder) -> Result<Value> {
        self.pop(function_builder)
    }

    /// Pop a float from the operand stack.
    pub fn pop_float(&mut self, function_builder: &mut FunctionBuilder) -> Result<Value> {
        let value = self.pop(function_builder)?;
        let value = function_builder.ins().fcvt_from_sint(types::F32, value);
        Ok(value)
    }

    /// Pop a double from the operand stack.
    pub fn pop_double(&mut self, function_builder: &mut FunctionBuilder) -> Result<Value> {
        let value = self.pop(function_builder)?;
        let value = function_builder.ins().fcvt_from_sint(types::F64, value);
        Ok(value)
    }
}
