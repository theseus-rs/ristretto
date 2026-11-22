//! Return a pending Java invocation with no generated caller left on the stack.
use super::Continuation;
use crate::compiler::native_frame::{word_offset, word_stack_slot};
use crate::function::ContinuationLayout;
use crate::function::activation::RequestKind;
use crate::operand_stack::OperandStack;
use crate::{Error::InternalError, Result};
use cranelift::codegen::ir::{MemFlagsData, StackSlot};
use cranelift::prelude::*;

pub(crate) struct CallContinuation<'a> {
    pub continuation: &'a Continuation,
    pub state: Value,
}

impl CallContinuation<'_> {
    #[expect(clippy::too_many_lines)]
    pub(super) fn emit(
        &self,
        builder: &mut FunctionBuilder,
        stack: &mut OperandStack,
        arguments: StackSlot,
        count: usize,
        bci: usize,
        request: (RequestKind, u16),
    ) -> Result<Value> {
        let site = bci;
        let state = self.state;
        let offset = self
            .continuation
            .layout
            .call_offset
            .ok_or_else(|| InternalError("missing call continuation storage".to_owned()))?;
        let restore = *self
            .continuation
            .calls
            .get(&site)
            .ok_or_else(|| InternalError("missing call resume block".to_owned()))?;
        let original = stack.as_slice().to_vec();
        let kinds = original
            .iter()
            .map(|value| builder.func.dfg.value_type(*value))
            .collect::<Vec<_>>();
        let join = builder.create_block();
        for kind in &kinds {
            builder.append_block_param(join, *kind);
        }
        let result = builder.declare_var(types::I64);
        let save = builder.create_block();
        // This path has no synchronous helper import or fallback.
        builder.ins().jump(save, &[]);

        builder.switch_to_block(save);
        // Runtime requests consume the same local budget as loop/handler edges.
        // Embeddings carry this remainder across immediately-ready requests.
        let remaining = builder.use_var(self.continuation.remaining);
        let remaining = builder.ins().iadd_imm_s(remaining, -1);
        builder.ins().store(
            MemFlagsData::trusted(),
            remaining,
            state,
            word_offset(ContinuationLayout::FUEL)?,
        );
        for (index, &(variable, _)) in self.continuation.locals.iter().enumerate() {
            let value = builder.use_var(variable);
            builder.ins().store(
                MemFlagsData::trusted(),
                value,
                state,
                word_offset(ContinuationLayout::HEADER_WORDS + index)?,
            );
        }
        let operands = ContinuationLayout::HEADER_WORDS + self.continuation.locals.len();
        for (index, value) in original.iter().enumerate() {
            builder.ins().store(
                MemFlagsData::trusted(),
                *value,
                state,
                word_offset(operands + index)?,
            );
        }
        let (kind, index) = request;
        for (cell, value) in [
            (ContinuationLayout::CALL_KIND, kind as u64),
            (ContinuationLayout::CALL_BCI, u64::try_from(bci)?),
            (ContinuationLayout::CALL_INDEX, u64::from(index)),
            (ContinuationLayout::CALL_COUNT, u64::try_from(count)?),
            (ContinuationLayout::CALL_RESULT, 0),
        ] {
            let value = builder.ins().iconst(types::I64, i64::try_from(value)?);
            builder.ins().store(
                MemFlagsData::trusted(),
                value,
                state,
                word_offset(offset + cell)?,
            );
        }
        for index in 0..count {
            let value =
                builder
                    .ins()
                    .stack_load(types::I64, types::I64, arguments, word_offset(index)?);
            builder.ins().store(
                MemFlagsData::trusted(),
                value,
                state,
                word_offset(offset + ContinuationLayout::CALL_HEADER_WORDS + index)?,
            );
        }
        let token = builder.ins().iconst(
            types::I64,
            i64::try_from(ContinuationLayout::CALL_TOKEN_BASE + u64::try_from(site)?)?,
        );
        builder.ins().store(
            MemFlagsData::trusted(),
            token,
            state,
            word_offset(ContinuationLayout::RESUME_TOKEN)?,
        );
        builder.ins().return_(&[]);

        builder.switch_to_block(restore);
        for (index, &(variable, kind)) in self.continuation.locals.iter().enumerate() {
            let value = builder.ins().load(
                kind,
                MemFlagsData::trusted(),
                state,
                word_offset(ContinuationLayout::HEADER_WORDS + index)?,
            );
            builder.def_var(variable, value);
        }
        let mut values = Vec::with_capacity(kinds.len());
        for (index, kind) in kinds.iter().enumerate() {
            let value = builder.ins().load(
                *kind,
                MemFlagsData::trusted(),
                state,
                word_offset(operands + index)?,
            );
            values.push(value.into());
        }
        let bits = builder.ins().load(
            types::I64,
            MemFlagsData::trusted(),
            state,
            word_offset(offset + ContinuationLayout::CALL_RESULT)?,
        );
        builder.def_var(result, bits);
        builder.ins().jump(join, &values);

        builder.switch_to_block(join);
        stack.inherit_block_references(builder, join)?;
        stack.reset(builder)?;
        Ok(builder.use_var(result))
    }
}

use ristretto_classfile::attributes::Instruction;
use ristretto_classfile::{BaseType, Constant, ConstantPool, FieldType};

const MAX_PARAMETER_SLOTS: usize = u8::MAX as usize;

#[derive(Clone, Copy)]
pub(crate) enum CallTarget {
    Static(u16),
    Special(u16),
    Virtual(u16),
    Interface(u16, u8),
}
impl CallTarget {
    pub fn from_instruction(instruction: &Instruction) -> Option<Self> {
        Some(match *instruction {
            Instruction::Invokestatic(i) => Self::Static(i),
            Instruction::Invokespecial(i) => Self::Special(i),
            Instruction::Invokevirtual(i) => Self::Virtual(i),
            Instruction::Invokeinterface(i, c) => Self::Interface(i, c),
            _ => return None,
        })
    }

    fn metadata(self) -> (RequestKind, u16) {
        match self {
            Self::Static(i) => (RequestKind::InvokeStatic, i),
            Self::Special(i) => (RequestKind::InvokeSpecial, i),
            Self::Virtual(i) => (RequestKind::InvokeVirtual, i),
            Self::Interface(i, _) => (RequestKind::InvokeInterface, i),
        }
    }

    pub fn receiver(self) -> usize {
        usize::from(!matches!(self, Self::Static(_)))
    }

    pub fn signature(self, pool: &ConstantPool) -> Result<(Vec<FieldType>, Option<FieldType>)> {
        let (_, index) = self.metadata();
        let entry = pool.try_get(index)?;
        let nt = match (self, entry) {
            (
                Self::Interface(..),
                Constant::InterfaceMethodRef {
                    name_and_type_index,
                    ..
                },
            )
            | (
                Self::Virtual(_),
                Constant::MethodRef {
                    name_and_type_index,
                    ..
                },
            )
            | (
                Self::Static(_) | Self::Special(_),
                Constant::MethodRef {
                    name_and_type_index,
                    ..
                }
                | Constant::InterfaceMethodRef {
                    name_and_type_index,
                    ..
                },
            ) => *name_and_type_index,
            _ => {
                return Err(ristretto_classfile::Error::InvalidConstantPoolIndexType(index).into());
            }
        };
        let (_, descriptor) = pool.try_get_name_and_type(nt)?;
        let signature = FieldType::parse_method_descriptor(pool.try_get_utf8(*descriptor)?)?;
        let slots = signature
            .0
            .iter()
            .map(|kind| usize::from(kind.slot_count()))
            .sum::<usize>()
            + self.receiver();
        if slots > MAX_PARAMETER_SLOTS {
            return Err(crate::Error::UnsupportedMethod(
                "compiled call exceeds the Java parameter-slot limit".to_owned(),
            ));
        }
        if let Self::Interface(_, count) = self
            && usize::from(count) != slots
        {
            return Err(crate::Error::UnsupportedMethod(
                "interface call count disagrees with the Java parameter-slot count".to_owned(),
            ));
        }
        Ok(signature)
    }
}

impl CallContinuation<'_> {
    pub fn invoke(
        &self,
        pool: &ConstantPool,
        builder: &mut FunctionBuilder,
        stack: &mut OperandStack,
        target: CallTarget,
        bci: usize,
    ) -> Result<()> {
        let (parameters, result) = target.signature(pool)?;
        let receiver = target.receiver();
        let count = parameters.len() + receiver;
        let arguments = word_stack_slot(builder, count.max(1))?;
        for (position, kind) in parameters.iter().enumerate().rev() {
            let bits = argument_bits(builder, stack, kind)?;
            builder.ins().stack_store(
                types::I64,
                bits,
                arguments,
                word_offset(position + receiver)?,
            );
        }
        if receiver != 0 {
            let value = stack.pop_object(builder)?;
            builder.ins().stack_store(types::I64, value, arguments, 0);
        }
        let bits = self.emit(builder, stack, arguments, count, bci, target.metadata())?;
        if let Some(kind) = result {
            push_result(builder, stack, &kind, bits)?;
        }
        Ok(())
    }
}

pub(super) fn argument_bits(
    builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
    kind: &FieldType,
) -> Result<Value> {
    Ok(match kind {
        FieldType::Object(_) | FieldType::Array(_) => stack.pop_object(builder)?,
        FieldType::Base(BaseType::Long) => stack.pop_long(builder)?,
        FieldType::Base(BaseType::Double) => {
            let value = stack.pop_double(builder)?;
            builder
                .ins()
                .bitcast(types::I64, MemFlagsData::new(), value)
        }
        FieldType::Base(BaseType::Float) => {
            let value = stack.pop_float(builder)?;
            let bits = builder
                .ins()
                .bitcast(types::I32, MemFlagsData::new(), value);
            builder.ins().uextend(types::I64, bits)
        }
        FieldType::Base(_) => {
            let value = stack.pop_int(builder)?;
            builder.ins().sextend(types::I64, value)
        }
    })
}

pub(super) fn push_result(
    builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
    kind: &FieldType,
    bits: Value,
) -> Result<()> {
    match kind {
        FieldType::Object(_) | FieldType::Array(_) => stack.push_object(builder, bits),
        FieldType::Base(BaseType::Long) => stack.push_long(builder, bits),
        FieldType::Base(BaseType::Double) => {
            let value = builder.ins().bitcast(types::F64, MemFlagsData::new(), bits);
            stack.push_double(builder, value)
        }
        FieldType::Base(BaseType::Float) => {
            let bits = builder.ins().ireduce(types::I32, bits);
            let value = builder.ins().bitcast(types::F32, MemFlagsData::new(), bits);
            stack.push_float(builder, value)
        }
        FieldType::Base(_) => {
            let value = builder.ins().ireduce(types::I32, bits);
            stack.push_int(builder, value)
        }
    }
}
