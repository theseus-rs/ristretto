//! Diagnostic counts of taken explicit Java branches to the same or an earlier BCI.
//! Forwarding blocks preserve operand values and count only the selected edge.

use crate::Error::InvalidBlockAddress;
use crate::Result;
use ahash::AHashMap;
use cranelift::codegen::ir::FuncRef;
use cranelift::jit::JITModule;
use cranelift::module::{Linkage, Module};
use cranelift::prelude::*;
use portable_atomic::{AtomicU64, Ordering};
use ristretto_classfile::attributes::Instruction;
use std::sync::Arc;

pub(super) const SYMBOL: &str = "ristretto_profile_backedge";

/// Called only by code whose owning Function retains the counter allocation.
pub(super) unsafe extern "C" fn record(counter: *const AtomicU64) {
    // SAFETY: compilation embeds the address of a retained Arc allocation. The
    // Function owns both code and counter through all executing calls and clones.
    unsafe { &*counter }.fetch_add(1, Ordering::Relaxed);
}

pub(super) struct BackedgeProfile {
    pub counter: Arc<AtomicU64>,
    helper: FuncRef,
    pointer_type: Type,
    operands: AHashMap<usize, Vec<Type>>,
}

pub(super) struct Edges {
    pub blocks: AHashMap<usize, Block>,
    forwards: Vec<(Block, Block, usize)>,
}

impl BackedgeProfile {
    pub fn new(
        module: &mut JITModule,
        builder: &mut FunctionBuilder,
        blocks: &AHashMap<usize, Block>,
    ) -> Result<Self> {
        let pointer_type = module.target_config().pointer_type();
        let mut signature = module.make_signature();
        signature.params.push(AbiParam::new(pointer_type));
        let function = module.declare_function(SYMBOL, Linkage::Import, &signature)?;
        let helper = module.declare_func_in_func(function, builder.func);
        Ok(Self {
            counter: Arc::new(AtomicU64::new(0)),
            helper,
            pointer_type,
            // Capture only explicit Java operands before Cranelift's SSA builder
            // appends implicit local-variable parameters to these blocks.
            operands: blocks
                .iter()
                .map(|(&address, &block)| {
                    let kinds = builder
                        .block_params(block)
                        .iter()
                        .map(|value| builder.func.dfg.value_type(*value))
                        .collect();
                    (address, kinds)
                })
                .collect(),
        })
    }

    pub fn edges(
        &self,
        builder: &mut FunctionBuilder,
        blocks: &AHashMap<usize, Block>,
        pc: usize,
        instruction: &Instruction,
    ) -> Result<Option<Edges>> {
        let mut addresses = std::collections::BTreeSet::new();
        for offset in instruction.branch_offsets().filter(|offset| *offset <= 0) {
            addresses.insert(usize::try_from(i64::try_from(pc)? + i64::from(offset))?);
        }
        if addresses.is_empty() {
            return Ok(None);
        }
        let mut edges = Edges {
            blocks: blocks.clone(),
            forwards: Vec::with_capacity(addresses.len()),
        };
        for address in addresses {
            let target = *blocks.get(&address).ok_or(InvalidBlockAddress(address))?;
            let forward = builder.create_block();
            let types = self
                .operands
                .get(&address)
                .ok_or(InvalidBlockAddress(address))?;
            for kind in types {
                builder.append_block_param(forward, *kind);
            }
            edges.blocks.insert(address, forward);
            edges.forwards.push((forward, target, types.len()));
        }
        Ok(Some(edges))
    }

    pub fn emit(&self, builder: &mut FunctionBuilder, edges: Edges) {
        for (forward, target, operands) in edges.forwards {
            builder.switch_to_block(forward);
            #[expect(
                clippy::cast_possible_wrap,
                reason = "preserve pointer bits in the signed Cranelift immediate"
            )]
            let address = Arc::as_ptr(&self.counter) as usize as i64;
            let counter = builder.ins().iconst(self.pointer_type, address);
            builder.ins().call(self.helper, &[counter]);
            let arguments: Vec<_> = builder
                .block_params(forward)
                .iter()
                .take(operands)
                .map(|value| (*value).into())
                .collect();
            builder.ins().jump(target, &arguments);
        }
    }
}
