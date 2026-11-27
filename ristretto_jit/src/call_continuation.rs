//! Return a pending Java invocation with no generated caller left on the stack.
use super::Continuation;
use crate::compiler::native_frame::word_offset;
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
    #[expect(
        clippy::too_many_arguments,
        reason = "default bytecode request uses its BCI as the distinct resume site"
    )]
    pub fn emit(
        &self,
        builder: &mut FunctionBuilder,
        stack: &mut OperandStack,
        arguments: StackSlot,
        count: usize,
        bci: usize,
        request: (RequestKind, u16),
        synchronous: impl FnOnce(&mut FunctionBuilder) -> Result<Value>,
    ) -> Result<Value> {
        self.emit_at(
            builder,
            stack,
            arguments,
            count,
            bci,
            bci,
            request,
            synchronous,
        )
    }

    #[expect(
        clippy::too_many_arguments,
        clippy::too_many_lines,
        reason = "distinct handler sites share the owning request ABI and exact restore/join"
    )]
    pub fn emit_at(
        &self,
        builder: &mut FunctionBuilder,
        stack: &mut OperandStack,
        arguments: StackSlot,
        count: usize,
        bci: usize,
        site: usize,
        request: (RequestKind, u16),
        synchronous: impl FnOnce(&mut FunctionBuilder) -> Result<Value>,
    ) -> Result<Value> {
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
        if self.continuation.layout.requests_only {
            // Do not emit a fallback call, even in unreachable IR: a foreign
            // reference runtime must never link a legacy helper or context cast.
            builder.ins().jump(save, &[]);
        } else {
            let sync = builder.create_block();
            let mode = builder.ins().load(
                types::I64,
                MemFlagsData::trusted(),
                state,
                word_offset(ContinuationLayout::ASYNC_MODE)?,
            );
            builder.ins().brif(mode, save, &[], sync, &[]);
            builder.switch_to_block(sync);
            let bits = synchronous(builder)?;
            builder.def_var(result, bits);
            let values = original
                .iter()
                .map(|value| (*value).into())
                .collect::<Vec<_>>();
            builder.ins().jump(join, &values);
        }

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
