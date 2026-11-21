//! Primitive array operations return to the owning host before accessing heap storage.
use super::CallContinuation;
use super::call::{argument_bits, push_result};
use crate::operand_stack::OperandStack;
use crate::{ArrayAccessKind, Result};
use cranelift::prelude::*;
use ristretto_classfile::{BaseType, FieldType, attributes::Instruction};

#[derive(Clone, Copy)]
pub(crate) enum ArrayRequest {
    New(u8),
    Access(ArrayAccessKind, u16),
}
impl ArrayRequest {
    pub fn from_instruction(instruction: &Instruction) -> Option<Self> {
        use ArrayAccessKind as A;
        Some(match instruction {
            Instruction::Newarray(kind) => Self::New(kind.code()),
            Instruction::Iaload => Self::Access(A::IntLoad, 46),
            Instruction::Laload => Self::Access(A::LongLoad, 47),
            Instruction::Faload => Self::Access(A::FloatLoad, 48),
            Instruction::Daload => Self::Access(A::DoubleLoad, 49),
            Instruction::Baload => Self::Access(A::ByteLoad, 51),
            Instruction::Caload => Self::Access(A::CharLoad, 52),
            Instruction::Saload => Self::Access(A::ShortLoad, 53),
            Instruction::Iastore => Self::Access(A::IntStore, 79),
            Instruction::Lastore => Self::Access(A::LongStore, 80),
            Instruction::Fastore => Self::Access(A::FloatStore, 81),
            Instruction::Dastore => Self::Access(A::DoubleStore, 82),
            Instruction::Bastore => Self::Access(A::ByteStore, 84),
            Instruction::Castore => Self::Access(A::CharStore, 85),
            Instruction::Sastore => Self::Access(A::ShortStore, 86),
            Instruction::Arraylength => Self::Access(A::Length, 190),
            _ => return None,
        })
    }
    pub fn capacity(self) -> usize {
        match self {
            Self::New(_) => 1,
            Self::Access(kind, _) => kind.argument_count(),
        }
    }
    pub fn emit(
        self,
        continuation: &CallContinuation<'_>,
        builder: &mut FunctionBuilder,
        stack: &mut OperandStack,
        bci: usize,
    ) -> Result<()> {
        use ArrayAccessKind as A;
        let count = self.capacity();
        let arguments = builder.create_sized_stack_slot(StackSlotData::new(
            StackSlotKind::ExplicitSlot,
            u32::try_from(count * 8)?,
            3,
        ));
        let (tag, index, result) = match self {
            Self::New(kind) => {
                let length = stack.pop_int(builder)?;
                let bits = builder.ins().sextend(types::I64, length);
                builder.ins().stack_store(types::I64, bits, arguments, 0);
                (7, u16::from(kind), None)
            }
            Self::Access(kind, code) => {
                let field = FieldType::Base(match kind {
                    A::LongLoad | A::LongStore => BaseType::Long,
                    A::FloatLoad | A::FloatStore => BaseType::Float,
                    A::DoubleLoad | A::DoubleStore => BaseType::Double,
                    _ => BaseType::Int,
                });
                if count == 3 {
                    let value = argument_bits(builder, stack, &field)?;
                    builder.ins().stack_store(types::I64, value, arguments, 16);
                }
                if kind != A::Length {
                    let index = stack.pop_int(builder)?;
                    let bits = builder.ins().sextend(types::I64, index);
                    builder.ins().stack_store(types::I64, bits, arguments, 8);
                }
                let array = stack.pop_object(builder)?;
                builder.ins().stack_store(types::I64, array, arguments, 0);
                (17, code, (count != 3).then_some(field))
            }
        };
        let bits = continuation.emit(builder, stack, arguments, count, bci, (tag, index))?;
        if let Some(field) = result {
            push_result(builder, stack, &field, bits)?;
        } else if matches!(self, Self::New(_)) {
            stack.push_object(builder, bits)?;
        }
        Ok(())
    }
}
