//! Primitive array operations return to the owning host before accessing heap storage.
use super::CallContinuation;
use super::call::{argument_bits, push_result};
use crate::compiler::native_frame::{word_offset, word_stack_slot};
use crate::function::activation::RequestKind;
use crate::operand_stack::OperandStack;
use crate::{ArrayAccessKind, Result};
use cranelift::prelude::*;
use ristretto_classfile::{BaseType, ConstantPool, FieldType, attributes::Instruction};

#[derive(Clone, Copy)]
pub(crate) enum ArrayRequest {
    New(u8),
    ReferenceNew(u16),
    ReferenceStore,
    Access(ArrayAccessKind, u16),
}
impl ArrayRequest {
    pub fn from_instruction(instruction: &Instruction) -> Option<Self> {
        Some(match instruction {
            Instruction::Newarray(kind) => Self::New(kind.code()),
            Instruction::Anewarray(index) => Self::ReferenceNew(*index),
            Instruction::Aastore => Self::ReferenceStore,
            _ => Self::Access(
                ArrayAccessKind::from_instruction(instruction)?,
                u16::from(instruction.code()),
            ),
        })
    }

    pub fn capacity(self) -> usize {
        match self {
            Self::New(_) | Self::ReferenceNew(_) => 1,
            Self::ReferenceStore => 3,
            Self::Access(kind, _) => kind.argument_count(),
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
        let count = self.capacity();
        let arguments = word_stack_slot(builder, count)?;
        let (tag, index, result) = match self {
            Self::ReferenceStore => {
                let value = stack.pop_object(builder)?;
                let index = stack.pop_int(builder)?;
                let index = builder.ins().sextend(types::I64, index);
                let array = stack.pop_object(builder)?;
                builder.ins().stack_store(types::I64, array, arguments, 0);
                builder
                    .ins()
                    .stack_store(types::I64, index, arguments, word_offset(1)?);
                builder
                    .ins()
                    .stack_store(types::I64, value, arguments, word_offset(2)?);
                (RequestKind::ReferenceArrayStore, 0, None)
            }
            Self::ReferenceNew(index) => {
                if pool.try_get_class(index)?.as_bytes().starts_with(b"[") {
                    return Err(crate::Error::UnsupportedMethod(
                        "reference array component must not be an array".to_owned(),
                    ));
                }
                let length = stack.pop_int(builder)?;
                let bits = builder.ins().sextend(types::I64, length);
                builder.ins().stack_store(types::I64, bits, arguments, 0);
                (RequestKind::NewReferenceArray, index, None)
            }
            Self::New(kind) => {
                let length = stack.pop_int(builder)?;
                let bits = builder.ins().sextend(types::I64, length);
                builder.ins().stack_store(types::I64, bits, arguments, 0);
                (RequestKind::NewPrimitiveArray, u16::from(kind), None)
            }
            Self::Access(kind, code) => {
                let field = if kind == ArrayAccessKind::ReferenceLoad {
                    FieldType::Object("java/lang/Object".into())
                } else {
                    FieldType::Base(match kind {
                        ArrayAccessKind::LongLoad | ArrayAccessKind::LongStore => BaseType::Long,
                        ArrayAccessKind::FloatLoad | ArrayAccessKind::FloatStore => BaseType::Float,
                        ArrayAccessKind::DoubleLoad | ArrayAccessKind::DoubleStore => {
                            BaseType::Double
                        }
                        _ => BaseType::Int,
                    })
                };
                if kind.is_store() {
                    let value = argument_bits(builder, stack, &field)?;
                    builder
                        .ins()
                        .stack_store(types::I64, value, arguments, word_offset(2)?);
                }
                if kind != ArrayAccessKind::Length {
                    let index = stack.pop_int(builder)?;
                    let bits = builder.ins().sextend(types::I64, index);
                    builder
                        .ins()
                        .stack_store(types::I64, bits, arguments, word_offset(1)?);
                }
                let array = stack.pop_object(builder)?;
                builder.ins().stack_store(types::I64, array, arguments, 0);
                (
                    RequestKind::ArrayAccess,
                    code,
                    (!kind.is_store()).then_some(field),
                )
            }
        };
        let bits = continuation.emit(builder, stack, arguments, count, bci, (tag, index))?;
        if let Some(field) = result {
            push_result(builder, stack, &field, bits)?;
        } else if matches!(self, Self::New(_) | Self::ReferenceNew(_)) {
            stack.push_object(builder, bits)?;
        }
        Ok(())
    }
}
