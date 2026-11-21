//! Spill typed scalar state only at taken backward edges; resume at the destination.
//! No native frame remains on the stack while the activation is suspended.
use crate::Error::{InternalError, InvalidBlockAddress};
use crate::Result;
use crate::function::ContinuationLayout;
use crate::local_variables::LocalVariables;
use ahash::AHashMap;
use cranelift::codegen::ir::MemFlagsData;
use cranelift::prelude::*;
use ristretto_classfile::attributes::Instruction;
use std::collections::{BTreeMap, BTreeSet};

struct Point {
    target: Block,
    edge: Block,
    restore: Block,
    operands: Vec<Type>,
}

pub(crate) struct Continuation {
    points: BTreeMap<usize, Point>,
    locals: Vec<(Variable, Type)>,
    remaining: Variable,
    pub layout: ContinuationLayout,
}

impl Continuation {
    pub fn new(
        builder: &mut FunctionBuilder,
        blocks: &AHashMap<usize, Block>,
        instructions: &[Instruction],
        locals: &LocalVariables,
        max_stack: u16,
    ) -> Result<Self> {
        let mut addresses = BTreeSet::new();
        for (pc, instruction) in Instruction::with_offsets(instructions) {
            for offset in instruction.branch_offsets().filter(|offset| *offset <= 0) {
                addresses.insert(usize::try_from(i64::try_from(pc)? + i64::from(offset))?);
            }
        }
        let locals = locals.continuation_variables();
        let frame_slots = locals.len().checked_add(usize::from(max_stack));
        if frame_slots.is_none_or(|slots| {
            slots > 64 * 1024
                || addresses
                    .len()
                    .checked_mul(slots)
                    .is_none_or(|total| total > 64 * 1024)
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
                },
            );
        }
        let max_operands = points
            .values()
            .map(|point| point.operands.len())
            .max()
            .unwrap_or(0);
        let words = ContinuationLayout::HEADER_WORDS
            .checked_add(locals.len())
            .and_then(|words| words.checked_add(max_operands))
            .ok_or_else(|| InternalError("continuation size overflow".to_owned()))?;
        let _ = Self::offset(words)?;
        Ok(Self {
            points,
            locals,
            remaining: builder.declare_var(types::I64),
            layout: ContinuationLayout {
                words,
                instructions: instructions.len(),
            },
        })
    }

    fn offset(cell: usize) -> Result<i32> {
        Ok(i32::try_from(cell.checked_mul(8).ok_or_else(|| {
            InternalError("continuation offset overflow".to_owned())
        })?)?)
    }

    /// Emit in the ABI entry block, after argument locals have been declared.
    pub fn entry(&self, builder: &mut FunctionBuilder, state: Value, initial: Block) -> Result<()> {
        let token = builder
            .ins()
            .load(types::I64, MemFlagsData::trusted(), state, 0);
        let complete = builder.ins().iconst(types::I64, -1);
        builder
            .ins()
            .store(MemFlagsData::trusted(), complete, state, 0);
        let remaining = builder
            .ins()
            .load(types::I64, MemFlagsData::trusted(), state, 8);
        builder.def_var(self.remaining, remaining);
        let invalid = builder.create_block();
        builder.set_cold_block(invalid);
        let mut dispatch = cranelift::frontend::Switch::new();
        dispatch.set_entry(0, initial);
        for (&address, point) in &self.points {
            dispatch.set_entry(u128::try_from(address)? + 1, point.restore);
        }
        dispatch.emit(builder, token, invalid);
        builder.switch_to_block(invalid);
        builder.ins().trap(
            TrapCode::user(1)
                .ok_or_else(|| InternalError("invalid continuation trap code".to_owned()))?,
        );
        for point in self.points.values() {
            builder.switch_to_block(point.restore);
            for (index, &(variable, kind)) in self.locals.iter().enumerate() {
                let value = builder.ins().load(
                    kind,
                    MemFlagsData::trusted(),
                    state,
                    Self::offset(ContinuationLayout::HEADER_WORDS + index)?,
                );
                builder.def_var(variable, value);
            }
            let mut arguments = Vec::new();
            for (index, kind) in point.operands.iter().enumerate() {
                let value = builder.ins().load(
                    *kind,
                    MemFlagsData::trusted(),
                    state,
                    Self::offset(ContinuationLayout::HEADER_WORDS + self.locals.len() + index)?,
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
    pub(super) fn finish(&self, builder: &mut FunctionBuilder, state: Value) -> Result<()> {
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
            for (index, &(variable, _)) in self.locals.iter().enumerate() {
                let value = builder.use_var(variable);
                builder.ins().store(
                    MemFlagsData::trusted(),
                    value,
                    state,
                    Self::offset(ContinuationLayout::HEADER_WORDS + index)?,
                );
            }
            for (index, value) in operands.iter().enumerate() {
                builder.ins().store(
                    MemFlagsData::trusted(),
                    *value,
                    state,
                    Self::offset(ContinuationLayout::HEADER_WORDS + self.locals.len() + index)?,
                );
            }
            let token = builder
                .ins()
                .iconst(types::I64, i64::try_from(address)? + 1);
            builder
                .ins()
                .store(MemFlagsData::trusted(), token, state, 0);
            builder.ins().return_(&[]);
        }
        Ok(())
    }
}
