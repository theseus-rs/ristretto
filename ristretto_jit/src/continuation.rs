//! Local native continuations with independently owned runtime roots.
//!
//! A selected backward edge or handler entry consumes one unit. Exhaustion spills typed locals
//! and remaining operands, then returns. Resumption restores that state and
//! enters the destination directly, without repeating the edge or any bytecode.
use super::frame_roots::FrameRoots;
use super::native_frame::word_offset;
use crate::Error::{InternalError, InvalidBlockAddress};
use crate::Result;
use crate::function::ContinuationLayout;
use crate::instruction::{CallTarget, allocation_metadata, call_signature};
use crate::local_variables::LocalVariables;
use crate::runtime_helpers::RuntimeHelpers;
use ahash::AHashMap;
use cranelift::codegen::ir::MemFlagsData;
use cranelift::prelude::*;
use ristretto_classfile::ConstantPool;
use ristretto_classfile::attributes::{ExceptionTableEntry, Instruction};
use std::collections::{BTreeMap, BTreeSet};

#[path = "call_continuation.rs"]
mod call;
pub(crate) use call::CallContinuation;

/// Logical cells needed by an outgoing runtime request; shared by native and heap buffers.
pub(crate) fn argument_count(
    instruction: &Instruction,
    pool: &ConstantPool,
) -> Result<Option<usize>> {
    if crate::instruction::reference_constant_index(instruction, pool).is_some() {
        return Ok(Some(0));
    }
    if let Some(target) = CallTarget::from_instruction(instruction) {
        let (_, receiver, _) = target.metadata();
        return Ok(Some(call_signature(pool, target)?.0.len() + receiver));
    }
    if let Some((kind, _)) = crate::instruction::array_access_metadata(instruction) {
        return Ok(Some(kind.argument_count()));
    }
    if matches!(instruction, Instruction::Aastore) {
        return Ok(Some(3));
    }
    if matches!(
        instruction,
        Instruction::Monitorenter
            | Instruction::Monitorexit
            | Instruction::Checkcast(_)
            | Instruction::Instanceof(_)
    ) {
        return Ok(Some(1));
    }
    if let Some((_, _, count)) = crate::instruction::field_metadata(instruction) {
        return Ok(Some(count));
    }
    Ok(allocation_metadata(instruction).map(|(_, _, count)| count))
}

const MAX_TRANSFER_SLOTS: usize = 64 * 1024;
const INVALID_CONTINUATION_TRAP: u8 = 1;

struct Point {
    target: Block,
    edge: Block,
    restore: Block,
    operands: Vec<Type>,
    reference_operands: Vec<usize>,
}

pub(crate) struct Continuation {
    points: BTreeMap<usize, Point>,
    calls: BTreeMap<usize, Block>,
    pub(crate) exception_dispatch: BTreeMap<usize, Block>,
    locals: Vec<(Variable, Type)>,
    remaining: Variable,
    pub layout: ContinuationLayout,
}

impl Continuation {
    #[expect(
        clippy::too_many_arguments,
        clippy::too_many_lines,
        reason = "method metadata, bounded handler sites and compiler state define the private continuation layout"
    )]
    pub fn new(
        builder: &mut FunctionBuilder,
        blocks: &AHashMap<usize, Block>,
        instructions: &[Instruction],
        locals: &LocalVariables,
        references: &[Value],
        requires_context: bool,
        max_stack: u16,
        pool: &ConstantPool,
        exception_table: &[ExceptionTableEntry],
        requests_only: bool,
    ) -> Result<Self> {
        let mut points = BTreeMap::new();
        let mut calls = BTreeMap::new();
        let mut exception_dispatch = BTreeMap::new();
        let typed_entries = exception_table
            .iter()
            .enumerate()
            .filter(|(_, entry)| entry.catch_type != 0)
            .collect::<Vec<_>>();
        // Bound table expansion before allocating Cranelift blocks. Even unreachable
        // protected bytecodes can otherwise multiply a large exception table.
        if instructions
            .len()
            .checked_mul(typed_entries.len())
            .is_none_or(|sites| sites > 64 * 1024)
        {
            return Err(crate::Error::UnsupportedMethod(
                "exception continuation expansion limit exceeded".to_owned(),
            ));
        }
        let mut call_capacity = 0;
        let mut addresses: BTreeSet<_> = exception_table
            .iter()
            .map(|entry| usize::from(entry.handler_pc))
            .collect();
        for (pc, instruction) in Instruction::with_offsets(instructions) {
            for &(ordinal, entry) in &typed_entries {
                if entry.catch_type != 0 && entry.range_pc.contains(&u16::try_from(pc)?) {
                    exception_dispatch
                        .entry(pc)
                        .or_insert_with(|| builder.create_block());
                    calls.insert(Self::exception_site(pc, ordinal)?, builder.create_block());
                }
            }
            if let Some(count) = argument_count(instruction, pool)? {
                call_capacity = call_capacity.max(count);
                calls.insert(pc, builder.create_block());
            }
            for offset in instruction.branch_offsets().filter(|offset| *offset <= 0) {
                let address = usize::try_from(i64::try_from(pc)? + i64::from(offset))?;
                addresses.insert(address);
            }
        }
        let locals = locals.continuation_variables();
        // Every restore edge carries declared typed locals and potentially an
        // operand stack. Reject excessive state expansion before emitting those
        // stores/loads and SSA joins; otherwise a small class file can create
        // millions of IR instructions and monopolize compilation.
        let frame_slots = locals.len().checked_add(usize::from(max_stack));
        if frame_slots.is_none_or(|slots| {
            slots > MAX_TRANSFER_SLOTS
                || addresses
                    .len()
                    .saturating_add(calls.len())
                    .checked_mul(slots)
                    .is_none_or(|total| total > MAX_TRANSFER_SLOTS)
        }) {
            return Err(crate::Error::UnsupportedMethod(
                "continuation state-transfer limit exceeded".to_owned(),
            ));
        }
        for address in addresses {
            let target = *blocks.get(&address).ok_or(InvalidBlockAddress(address))?;
            // Capture explicit operands before SSA appends implicit locals.
            let operands: Vec<_> = builder
                .block_params(target)
                .iter()
                .map(|value| builder.func.dfg.value_type(*value))
                .collect();
            let reference_operands = builder
                .block_params(target)
                .iter()
                .enumerate()
                .filter_map(|(index, value)| references.contains(value).then_some(index))
                .collect();
            let edge = builder.create_block();
            let restore = builder.create_block();
            for kind in &operands {
                builder.append_block_param(edge, *kind);
            }
            points.insert(
                address,
                Point {
                    target,
                    edge,
                    restore,
                    operands,
                    reference_operands,
                },
            );
        }
        let max_operands = if calls.is_empty() {
            points
                .values()
                .map(|point| point.operands.len())
                .max()
                .unwrap_or(0)
        } else {
            usize::from(max_stack)
        };
        let mut words = ContinuationLayout::HEADER_WORDS
            .checked_add(locals.len())
            .and_then(|words| words.checked_add(max_operands))
            .ok_or_else(|| InternalError("continuation size overflow".to_owned()))?;
        let call_offset = if calls.is_empty() {
            None
        } else {
            let offset = words;
            words = words
                .checked_add(ContinuationLayout::CALL_HEADER_WORDS)
                .and_then(|words| words.checked_add(call_capacity))
                .ok_or_else(|| InternalError("call continuation size overflow".to_owned()))?;
            Some(offset)
        };
        let _ = word_offset(words)?;
        Ok(Self {
            points,
            calls,
            exception_dispatch,
            locals,
            remaining: builder.declare_var(types::I64),
            layout: ContinuationLayout {
                words,
                instructions: instructions.len(),
                requires_context,
                call_offset,
                call_capacity,
                requests_only,
            },
        })
    }

    /// Separate each typed table entry from both bytecode calls and other entries
    /// at the same throwing BCI, including duplicate catch types.
    pub(crate) fn exception_site(bci: usize, ordinal: usize) -> Result<usize> {
        let bci = u16::try_from(bci)?;
        ordinal
            .checked_add(1)
            .and_then(|entry| entry.checked_mul(1 << 16))
            .and_then(|base| base.checked_add(usize::from(bci)))
            .ok_or_else(|| InternalError("exception continuation token overflow".to_owned()))
    }

    /// Emit in the ABI entry block, after argument locals have been declared.
    pub fn entry(&self, builder: &mut FunctionBuilder, state: Value, initial: Block) -> Result<()> {
        let token = builder.ins().load(
            types::I64,
            MemFlagsData::trusted(),
            state,
            word_offset(ContinuationLayout::RESUME_TOKEN)?,
        );
        let complete = builder
            .ins()
            .iconst(types::I64, ContinuationLayout::RETURNED_TOKEN.cast_signed());
        builder.ins().store(
            MemFlagsData::trusted(),
            complete,
            state,
            word_offset(ContinuationLayout::RESUME_TOKEN)?,
        );
        let remaining = builder.ins().load(
            types::I64,
            MemFlagsData::trusted(),
            state,
            word_offset(ContinuationLayout::FUEL)?,
        );
        builder.def_var(self.remaining, remaining);
        let invalid = builder.create_block();
        builder.set_cold_block(invalid);
        let mut dispatch = cranelift::frontend::Switch::new();
        dispatch.set_entry(u128::from(ContinuationLayout::INITIAL_TOKEN), initial);
        for (&address, point) in &self.points {
            dispatch.set_entry(
                u128::try_from(address)? + u128::from(ContinuationLayout::BACKEDGE_TOKEN_BASE),
                point.restore,
            );
        }
        for (&address, block) in &self.calls {
            dispatch.set_entry(
                u128::from(ContinuationLayout::CALL_TOKEN_BASE) + u128::try_from(address)?,
                *block,
            );
        }
        dispatch.emit(builder, token, invalid);
        builder.switch_to_block(invalid);
        builder.ins().trap(
            TrapCode::user(INVALID_CONTINUATION_TRAP)
                .ok_or_else(|| InternalError("invalid continuation trap code".to_owned()))?,
        );
        for point in self.points.values() {
            builder.switch_to_block(point.restore);
            for (index, &(variable, kind)) in self.locals.iter().enumerate() {
                let value = builder.ins().load(
                    kind,
                    MemFlagsData::trusted(),
                    state,
                    word_offset(ContinuationLayout::HEADER_WORDS + index)?,
                );
                builder.def_var(variable, value);
            }
            let mut arguments = Vec::new();
            for (index, kind) in point.operands.iter().enumerate() {
                let value = builder.ins().load(
                    *kind,
                    MemFlagsData::trusted(),
                    state,
                    word_offset(ContinuationLayout::HEADER_WORDS + self.locals.len() + index)?,
                );
                arguments.push(value.into());
            }
            builder.ins().jump(point.target, &arguments);
        }
        Ok(())
    }

    pub fn redirect(
        &self,
        blocks: &mut AHashMap<usize, Block>,
        pc: usize,
        instruction: &Instruction,
    ) -> Result<()> {
        for offset in instruction.branch_offsets().filter(|offset| *offset <= 0) {
            let address = usize::try_from(i64::try_from(pc)? + i64::from(offset))?;
            let point = self
                .points
                .get(&address)
                .ok_or(InvalidBlockAddress(address))?;
            *blocks
                .get_mut(&address)
                .ok_or(InvalidBlockAddress(address))? = point.edge;
        }
        Ok(())
    }

    /// Exception dispatch has its own destination map. Normal branches to a
    /// handler BCI still use their ordinary forward/backward branch semantics.
    pub fn handler_targets(
        &self,
        blocks: &AHashMap<usize, Block>,
        exception_table: &[ExceptionTableEntry],
    ) -> Result<AHashMap<usize, Block>> {
        let mut targets = blocks.clone();
        for entry in exception_table {
            let address = usize::from(entry.handler_pc);
            let point = self
                .points
                .get(&address)
                .ok_or(InvalidBlockAddress(address))?;
            targets.insert(address, point.edge);
        }
        Ok(targets)
    }

    /// Reuse one target map for the method; do not clone the whole CFG per edge.
    /// Repeated switch destinations restore the same original block idempotently.
    pub fn restore_targets(
        blocks: &mut AHashMap<usize, Block>,
        originals: &AHashMap<usize, Block>,
        pc: usize,
        instruction: &Instruction,
    ) -> Result<()> {
        for offset in instruction.branch_offsets().filter(|offset| *offset <= 0) {
            let address = usize::try_from(i64::try_from(pc)? + i64::from(offset))?;
            *blocks
                .get_mut(&address)
                .ok_or(InvalidBlockAddress(address))? = *originals
                .get(&address)
                .ok_or(InvalidBlockAddress(address))?;
        }
        Ok(())
    }

    /// Emit once after all bytecode blocks: SSA supplies the locals from whichever
    /// predecessor selected this edge. Only explicit operands are forwarded.
    pub(super) fn finish(
        &self,
        builder: &mut FunctionBuilder,
        state: Value,
        locals: &LocalVariables,
        mut frame_roots: Option<&mut FrameRoots>,
        helpers: &RuntimeHelpers,
        context: Value,
    ) -> Result<()> {
        // Unreachable bytecodes never emitted their call body. A private token
        // cannot validly select them; keep the dispatch total with a cold trap.
        for block in self.calls.values() {
            if !builder.func.layout.is_block_inserted(*block) {
                builder.switch_to_block(*block);
                builder.ins().trap(
                    TrapCode::user(crate::instruction::TRAP_INTERNAL_ERROR).ok_or_else(|| {
                        InternalError("invalid continuation trap code".to_owned())
                    })?,
                );
            }
        }
        for (&address, point) in &self.points {
            builder.switch_to_block(point.edge);
            let remaining = builder.use_var(self.remaining);
            let remaining = builder.ins().iadd_imm_s(remaining, -1);
            builder.def_var(self.remaining, remaining);
            let operands: Vec<_> = builder
                .block_params(point.edge)
                .iter()
                .take(point.operands.len())
                .copied()
                .collect();
            let arguments: Vec<_> = operands.iter().map(|value| (*value).into()).collect();
            let spill = builder.create_block();
            builder.set_cold_block(spill);
            builder
                .ins()
                .brif(remaining, point.target, &arguments, spill, &[]);
            builder.switch_to_block(spill);
            if let Some(roots) = &mut frame_roots {
                // Publish the destination's live set, not the predecessor's:
                // consumed reference conditions and dead locals must not remain
                // rooted for the duration of an arbitrarily long suspension.
                let references = point
                    .reference_operands
                    .iter()
                    .map(|index| {
                        operands.get(*index).copied().ok_or_else(|| {
                            InternalError("missing continuation reference operand".to_owned())
                        })
                    })
                    .collect::<Result<Vec<_>>>()?;
                roots.publish(builder, locals, references, helpers, context, address)?;
            }
            for (index, &(variable, _)) in self.locals.iter().enumerate() {
                let value = builder.use_var(variable);
                builder.ins().store(
                    MemFlagsData::trusted(),
                    value,
                    state,
                    word_offset(ContinuationLayout::HEADER_WORDS + index)?,
                );
            }
            for (index, value) in operands.iter().enumerate() {
                builder.ins().store(
                    MemFlagsData::trusted(),
                    *value,
                    state,
                    word_offset(ContinuationLayout::HEADER_WORDS + self.locals.len() + index)?,
                );
            }
            let token = builder.ins().iconst(
                types::I64,
                i64::try_from(address)? + ContinuationLayout::BACKEDGE_TOKEN_BASE.cast_signed(),
            );
            builder.ins().store(
                MemFlagsData::trusted(),
                token,
                state,
                word_offset(ContinuationLayout::RESUME_TOKEN)?,
            );
            builder.ins().return_(&[]);
        }
        // A syntactically present call can be unreachable. Keep its private dispatch
        // target fail-closed instead of leaving an unterminated native block.
        for block in self.calls.values() {
            if builder.func.layout.block_insts(*block).next().is_none() {
                builder.switch_to_block(*block);
                builder.ins().trap(
                    TrapCode::user(INVALID_CONTINUATION_TRAP).ok_or_else(|| {
                        InternalError("invalid continuation trap code".to_owned())
                    })?,
                );
            }
        }
        Ok(())
    }
}
