//! Spill typed scalar state only at taken backward edges; resume at the destination.
//! No native frame remains on the stack while the activation is suspended.
use super::frame_roots::FrameRoots;
use super::native_frame::word_offset;
use crate::Error::{InternalError, InvalidBlockAddress};
use crate::Result;
use crate::function::ContinuationLayout;
use crate::local_variables::LocalVariables;
use crate::runtime_helpers::RuntimeHelpers;
use ahash::AHashMap;
use cranelift::codegen::ir::MemFlagsData;
use cranelift::prelude::*;
use ristretto_classfile::ConstantPool;
use ristretto_classfile::attributes::Instruction;

#[path = "call_continuation.rs"]
mod call;
pub(crate) use call::{CallContinuation, CallTarget};
#[path = "heap_continuation.rs"]
mod heap;
pub(crate) use heap::HeapRequest;
#[path = "array_continuation.rs"]
mod array;
pub(crate) use array::ArrayRequest;
use std::collections::{BTreeMap, BTreeSet};

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
    locals: Vec<(Variable, Type)>,
    remaining: Variable,
    pub layout: ContinuationLayout,
}

impl Continuation {
    #[expect(
        clippy::too_many_arguments,
        reason = "bounded continuation layout metadata"
    )]
    pub fn new(
        builder: &mut FunctionBuilder,
        blocks: &AHashMap<usize, Block>,
        instructions: &[Instruction],
        locals: &LocalVariables,
        max_stack: u16,
        references: &[Value],
        requires_context: bool,
        pool: &ConstantPool,
    ) -> Result<Self> {
        let mut addresses = BTreeSet::new();
        let mut calls = BTreeMap::new();
        let mut call_capacity = 0;
        for (pc, instruction) in Instruction::with_offsets(instructions) {
            if let Some(target) = CallTarget::from_instruction(instruction) {
                let (parameters, _) = target.signature(pool)?;
                call_capacity = call_capacity.max(parameters.len() + target.receiver());
                calls.insert(pc, builder.create_block());
            } else if let Some(request) = HeapRequest::from_instruction(instruction) {
                call_capacity = call_capacity.max(request.capacity(pool)?);
                calls.insert(pc, builder.create_block());
            } else if let Some(request) = ArrayRequest::from_instruction(instruction) {
                call_capacity = call_capacity.max(request.capacity());
                calls.insert(pc, builder.create_block());
            }
            for offset in instruction.branch_offsets().filter(|offset| *offset <= 0) {
                addresses.insert(usize::try_from(i64::try_from(pc)? + i64::from(offset))?);
            }
        }
        let locals = locals.continuation_variables();
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
        let mut points = BTreeMap::new();
        for address in addresses {
            let target = *blocks.get(&address).ok_or(InvalidBlockAddress(address))?;
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
        let max_operands = points
            .values()
            .map(|point| point.operands.len())
            .max()
            .unwrap_or(0)
            .max(if calls.is_empty() {
                0
            } else {
                usize::from(max_stack)
            });
        let words = ContinuationLayout::HEADER_WORDS
            .checked_add(locals.len())
            .and_then(|words| words.checked_add(max_operands))
            .ok_or_else(|| InternalError("continuation size overflow".to_owned()))?;
        let call_offset = (!calls.is_empty()).then_some(words);
        let words = if calls.is_empty() {
            words
        } else {
            words
                .checked_add(ContinuationLayout::CALL_HEADER_WORDS)
                .and_then(|words| words.checked_add(call_capacity))
                .ok_or_else(|| InternalError("call continuation size overflow".to_owned()))?
        };
        let _ = word_offset(words)?;
        Ok(Self {
            points,
            calls,
            locals,
            remaining: builder.declare_var(types::I64),
            layout: ContinuationLayout {
                words,
                instructions: instructions.len(),
                requires_context,
                call_offset,
                call_capacity,
            },
        })
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

    /// Emit one spill point per selected backward destination. SSA supplies locals.
    pub(super) fn finish(
        &self,
        builder: &mut FunctionBuilder,
        state: Value,
        locals: &LocalVariables,
        mut frame_roots: Option<&mut FrameRoots>,
        helpers: &RuntimeHelpers,
        context: Value,
    ) -> Result<()> {
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
                let references = point
                    .reference_operands
                    .iter()
                    .map(|index| {
                        operands.get(*index).copied().ok_or_else(|| {
                            InternalError("missing continuation reference operand".to_owned())
                        })
                    })
                    .collect::<Result<Vec<_>>>()?;
                // The destination live set is authoritative while suspended.
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
