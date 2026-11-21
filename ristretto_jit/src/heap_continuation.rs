//! Heap requests share owning activation cells and never call synchronous helpers.
use super::CallContinuation;
use super::call::{argument_bits, push_result};
use crate::Result;
use crate::operand_stack::OperandStack;
use cranelift::prelude::*;
use ristretto_classfile::attributes::Instruction;
use ristretto_classfile::{ConstantPool, FieldType};

#[derive(Clone, Copy)]
pub(crate) enum HeapRequest {
    New(u16),
    Get(u16),
    GetStatic(u16),
    PutStatic(u16),
    Put(u16),
    CheckCast(u16),
}
impl HeapRequest {
    pub fn from_instruction(instruction: &Instruction) -> Option<Self> {
        Some(match *instruction {
            Instruction::New(i) => Self::New(i),
            Instruction::Getfield(i) => Self::Get(i),
            Instruction::Getstatic(i) => Self::GetStatic(i),
            Instruction::Putstatic(i) => Self::PutStatic(i),
            Instruction::Putfield(i) => Self::Put(i),
            Instruction::Checkcast(i) => Self::CheckCast(i),
            _ => return None,
        })
    }
    fn field(pool: &ConstantPool, index: u16) -> Result<FieldType> {
        let (_, name_type) = pool.try_get_field_ref(index)?;
        let (_, descriptor) = pool.try_get_name_and_type(*name_type)?;
        Ok(FieldType::parse_java_str(pool.try_get_utf8(*descriptor)?)?)
    }
    pub fn capacity(self, pool: &ConstantPool) -> Result<usize> {
        match self {
            Self::New(index) => {
                let class = pool.try_get_class(index)?;
                if class.as_bytes().starts_with(b"[") {
                    return Err(crate::Error::UnsupportedMethod(
                        "new requires a non-array class".to_owned(),
                    ));
                }
                Ok(0)
            }
            Self::GetStatic(index) => {
                Self::field(pool, index)?;
                Ok(0)
            }
            Self::PutStatic(index) | Self::Get(index) => {
                Self::field(pool, index)?;
                Ok(1)
            }
            Self::CheckCast(index) => {
                let target = pool.try_get_class(index)?;
                if target.as_bytes().starts_with(b"[[") {
                    return Err(crate::Error::UnsupportedMethod(
                        "multidimensional cast target is not admitted".to_owned(),
                    ));
                }
                if target.as_bytes().starts_with(b"[") {
                    FieldType::parse_java_str(target)?;
                }
                Ok(1)
            }

            Self::Put(index) => {
                Self::field(pool, index)?;
                Ok(2)
            }
        }
    }
    pub fn emit(
        self,
        continuation: &CallContinuation<'_>,
        pool: &ConstantPool,
        builder: &mut FunctionBuilder,
        stack: &mut OperandStack,
        bci: usize,
    ) -> Result<()> {
        let count = self.capacity(pool)?;
        let arguments = builder.create_sized_stack_slot(StackSlotData::new(
            StackSlotKind::ExplicitSlot,
            u32::try_from(count.max(1) * 8)?,
            3,
        ));
        let (tag, index, result) = match self {
            Self::New(index) => (6, index, None),
            Self::GetStatic(index) => (10, index, Some(Self::field(pool, index)?)),
            Self::PutStatic(index) => {
                let bits = argument_bits(builder, stack, &Self::field(pool, index)?)?;
                builder.ins().stack_store(types::I64, bits, arguments, 0);
                (11, index, None)
            }
            Self::CheckCast(index) => {
                let root = stack.pop_object(builder)?;
                builder.ins().stack_store(types::I64, root, arguments, 0);
                (14, index, None)
            }
            Self::Get(index) => {
                let root = stack.pop_object(builder)?;
                builder.ins().stack_store(types::I64, root, arguments, 0);
                (12, index, Some(Self::field(pool, index)?))
            }
            Self::Put(index) => {
                let bits = argument_bits(builder, stack, &Self::field(pool, index)?)?;
                let root = stack.pop_object(builder)?;
                builder.ins().stack_store(types::I64, root, arguments, 0);
                builder.ins().stack_store(types::I64, bits, arguments, 8);
                (13, index, None)
            }
        };
        let bits = continuation.emit(builder, stack, arguments, count, bci, (tag, index))?;
        if let Some(kind) = result {
            push_result(builder, stack, &kind, bits)?;
        } else if matches!(self, Self::New(_) | Self::CheckCast(_)) {
            stack.push_object(builder, bits)?;
        }
        Ok(())
    }
}
