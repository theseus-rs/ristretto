//! Frame snapshots contain live reference locals and current reference operands.
//! Reference-bearing production eligibility remains closed pending the rest of
//! the GC/ABI qualification and publication-cost measurements.
use super::reference_liveness::ReferenceLiveness;
use crate::compiler::native_frame::{word_offset, word_stack_slot};
use crate::local_variables::LocalVariables;
use crate::runtime_helpers::RuntimeHelpers;
use crate::{Error, Result};
use cranelift::codegen::ir::StackSlot;
use cranelift::prelude::*;
use ristretto_classfile::attributes::{ExceptionTableEntry, Instruction};

pub(super) struct FrameRoots {
    liveness: ReferenceLiveness,
    failure: Block,
    spill: Option<StackSlot>,
    capacity: usize,
}

impl FrameRoots {
    pub(super) fn new(
        builder: &mut FunctionBuilder,
        locals: &LocalVariables,
        max_stack: u16,
        instructions: &[Instruction],
        exceptions: &[ExceptionTableEntry],
        failure: Block,
    ) -> Result<Self> {
        let locals = locals.reference_indices();
        let capacity = locals.len() + usize::from(max_stack);
        if capacity > crate::FRAME_ROOT_CAPACITY {
            return Err(Error::UnsupportedMethod(
                "frame root spill capacity is not qualified".to_owned(),
            ));
        }
        let liveness = ReferenceLiveness::new(instructions, exceptions, locals)?;
        let spill = if capacity == 0 {
            None
        } else {
            Some(word_stack_slot(builder, capacity)?)
        };
        Ok(Self {
            liveness,
            failure,
            spill,
            capacity,
        })
    }

    pub(super) fn publish(
        &mut self,
        builder: &mut FunctionBuilder,
        locals: &LocalVariables,
        operands: impl IntoIterator<Item = Value>,
        helpers: &RuntimeHelpers,
        context: Value,
        program_counter: usize,
    ) -> Result<()> {
        let mut roots = self
            .liveness
            .at(program_counter)?
            .map(|index| locals.get_object(builder, index))
            .collect::<Result<Vec<_>>>()?;
        roots.extend(operands);
        if roots.len() > self.capacity {
            return Err(Error::InternalError("frame root spill overflow".to_owned()));
        }
        let pointer_type = builder.func.dfg.value_type(context);
        let pointer = if roots.is_empty() {
            builder.ins().iconst(pointer_type, 0)
        } else {
            let spill = self
                .spill
                .ok_or_else(|| Error::InternalError("missing root spill".to_owned()))?;
            for (index, value) in roots.iter().enumerate() {
                builder
                    .ins()
                    .stack_store(pointer_type, *value, spill, word_offset(index)?);
            }
            builder.ins().stack_addr(pointer_type, spill, 0)
        };
        let count = builder
            .ins()
            .iconst(types::I32, i64::try_from(roots.len())?);
        let call = builder
            .ins()
            .call(helpers.publish_frame_roots, &[context, pointer, count]);
        let success = *builder.inst_results(call).first().ok_or_else(|| {
            Error::InternalError("root publication returned no status".to_owned())
        })?;
        let continuation = builder.create_block();
        builder
            .ins()
            .brif(success, continuation, &[], self.failure, &[]);
        builder.switch_to_block(continuation);
        // Existing operand SSA values dominate this single-predecessor continuation.
        // Keeping them also preserves their reference kinds without fresh parameters.
        Ok(())
    }
}
