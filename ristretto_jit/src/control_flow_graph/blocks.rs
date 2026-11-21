use crate::Error::{InternalError, InvalidBlockAddress, UnsupportedInstruction};
use crate::Result;
use crate::control_flow_graph::instruction;
use crate::control_flow_graph::type_stack::TypeStack;
use crate::local_type::StackType;
use ahash::AHashMap;
use cranelift::prelude::{Block, FunctionBuilder, Type, Value};
use ristretto_classfile::ConstantPool;
use ristretto_classfile::attributes::{ExceptionTableEntry, Instruction};

/// Analyzes reachable operand stacks before creating blocks. Each incoming edge,
/// including fallthrough, must agree on Java kinds rather than machine widths.
/// The worklist discovers successors even when a block is first reached backward.
/// Exception handlers start with exactly one reference, independent of the
/// throwing instruction's operand stack.
///
/// Returns the entry block, byte-offset block map and reference block parameters.
pub(crate) fn get_blocks(
    function_builder: &mut FunctionBuilder,
    constant_pool: &ConstantPool,
    instructions: &[Instruction],
    exception_table: &[ExceptionTableEntry],
) -> Result<(Block, AHashMap<usize, Block>, Vec<Value>)> {
    let states = analyze_stacks(constant_pool, instructions, exception_table)?;
    let entry_block = function_builder.create_block();
    let mut blocks = AHashMap::default();
    // Keep ABI setup separate from Java bytecode 0. The setup can branch to a
    // cold metadata path; Java backedges must never repeat ABI initialization.
    let mut addresses = states.keys().copied().collect::<Vec<_>>();
    addresses.sort_unstable();
    for address in addresses {
        create_block_with_parameters(function_builder, &states, address, &mut blocks);
    }
    let mut reference_parameters = Vec::new();
    for (address, block) in &blocks {
        let stack = states.get(address).ok_or(InvalidBlockAddress(*address))?;
        let parameters = function_builder.func.dfg.block_params(*block);
        if stack.len() != parameters.len() {
            return Err(InternalError("block stack arity mismatch".to_owned()));
        }
        for (kind, value) in stack.as_slice().iter().zip(parameters) {
            if *kind == StackType::Reference {
                reference_parameters.push(*value);
            }
        }
    }
    Ok((entry_block, blocks, reference_parameters))
}

/// Retain states only at block boundaries. A long straight-line sequence must
/// not copy and retain its entire operand stack at every instruction.
fn analyze_stacks(
    constant_pool: &ConstantPool,
    instructions: &[Instruction],
    exception_table: &[ExceptionTableEntry],
) -> Result<AHashMap<usize, TypeStack>> {
    let by_address: AHashMap<_, _> = Instruction::with_offsets(instructions).collect();
    let mut leaders = std::collections::BTreeSet::from([0]);
    for (address, instruction) in Instruction::with_offsets(instructions) {
        if matches!(
            instruction,
            Instruction::Jsr(..)
                | Instruction::Jsr_w(..)
                | Instruction::Ret(..)
                | Instruction::Ret_w(..)
        ) {
            // No returnAddress dataflow: legacy subroutines remain interpreted.
            return Err(UnsupportedInstruction(instruction.clone()));
        }
        let mut branches = false;
        for offset in instruction.branch_offsets() {
            branches = true;
            let target = usize::try_from(i64::try_from(address)? + i64::from(offset))?;
            if !by_address.contains_key(&target) {
                return Err(InvalidBlockAddress(target));
            }
            leaders.insert(target);
        }
        if branches && falls_through(instruction) {
            leaders.insert(address + instruction.byte_len(address));
        }
    }
    let mut states = AHashMap::default();
    let mut pending = std::collections::VecDeque::new();
    enqueue_stack(&by_address, &mut states, &mut pending, 0, &TypeStack::new())?;
    let mut handler_stack = TypeStack::new();
    handler_stack.push_object()?;
    for entry in exception_table {
        let address = usize::from(entry.handler_pc);
        leaders.insert(address);
        enqueue_stack(
            &by_address,
            &mut states,
            &mut pending,
            address,
            &handler_stack,
        )?;
    }
    while let Some(mut address) = pending.pop_front() {
        let mut stack = states
            .get(&address)
            .ok_or(InvalidBlockAddress(address))?
            .clone();
        loop {
            let instruction = by_address
                .get(&address)
                .ok_or(InvalidBlockAddress(address))?;
            instruction::simulate(&mut stack, constant_pool, instruction)?;
            for offset in instruction.branch_offsets() {
                let target = usize::try_from(i64::try_from(address)? + i64::from(offset))?;
                enqueue_stack(&by_address, &mut states, &mut pending, target, &stack)?;
            }
            if !falls_through(instruction) {
                break;
            }
            address = address
                .checked_add(instruction.byte_len(address))
                .ok_or_else(|| InternalError("instruction address overflow".to_owned()))?;
            if leaders.contains(&address) {
                enqueue_stack(&by_address, &mut states, &mut pending, address, &stack)?;
                break;
            }
        }
    }
    Ok(states)
}

pub(crate) fn falls_through(instruction: &Instruction) -> bool {
    !matches!(
        instruction,
        Instruction::Goto(..)
            | Instruction::Goto_w(..)
            | Instruction::Tableswitch(..)
            | Instruction::Lookupswitch(..)
            | Instruction::Return
            | Instruction::Ireturn
            | Instruction::Lreturn
            | Instruction::Freturn
            | Instruction::Dreturn
            | Instruction::Areturn
            | Instruction::Athrow
    )
}

/// Check every incoming edge, and simulate each reachable block once. Java
/// reference subtypes have the same root kind here; class verification remains
/// responsible for assignability, initialization and local-variable validity.
fn enqueue_stack(
    instructions: &AHashMap<usize, &Instruction>,
    states: &mut AHashMap<usize, TypeStack>,
    pending: &mut std::collections::VecDeque<usize>,
    address: usize,
    stack: &TypeStack,
) -> Result<()> {
    if !instructions.contains_key(&address) {
        return Err(InvalidBlockAddress(address));
    }
    match states.entry(address) {
        std::collections::hash_map::Entry::Occupied(entry) => {
            if entry.get() != stack {
                return Err(InternalError(format!(
                    "Invalid stack state for address {address}, entry_stack={:?} and stack={stack:?}",
                    entry.get()
                )));
            }
        }
        std::collections::hash_map::Entry::Vacant(entry) => {
            entry.insert(stack.clone());
            pending.push_back(address);
        }
    }
    Ok(())
}

/// Creates a new block with parameters matching the expected stack state at an address.
///
/// This function creates a Cranelift Block for a specific instruction address and configures it
/// with the appropriate parameters based on the operand stack state expected at that point in the
/// program.
///
/// # Note
///
/// This function only creates a new block if one doesn't already exist for the address. Block
/// parameters are added to match the stack state, enabling proper SSA form across control flow
/// edges.
pub(crate) fn create_block_with_parameters(
    function_builder: &mut FunctionBuilder,
    stack_states: &AHashMap<usize, TypeStack>,
    address: usize,
    blocks: &mut AHashMap<usize, Block>,
) {
    blocks.entry(address).or_insert_with(|| {
        let block = function_builder.create_block();

        if let Some(stack_types) = stack_states.get(&address) {
            let types: Vec<_> = stack_types
                .as_slice()
                .iter()
                .map(|kind| kind.machine_type())
                .collect();
            append_block_params(function_builder, block, &types);
        }

        block
    });
}

/// Appends type parameters to a Cranelift block.
///
/// This function adds formal parameters to a Cranelift Block, with types matching the given type
/// list. These parameters represent the values that will be on the operand stack when control
/// reaches this block.
pub(crate) fn append_block_params(
    function_builder: &mut FunctionBuilder,
    block: Block,
    types: &[Type],
) {
    for value_type in types {
        function_builder.append_block_param(block, *value_type);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cranelift::codegen::ir::Function;
    use cranelift::prelude::*;

    #[test]
    fn test_blocks_for_if_comparison_with_goto() -> Result<()> {
        let constant_pool = ConstantPool::new();
        let mut function_context = FunctionBuilderContext::new();
        let mut function = Function::new();
        let mut function_builder = FunctionBuilder::new(&mut function, &mut function_context);
        let instructions = vec![
            Instruction::Iload_0,      // 0: Load local variable 0
            Instruction::Iload_1,      // 1: Load local variable 1
            Instruction::If_icmplt(7), // 2: If var0 < var1, branch to instruction 5
            Instruction::Iload_0,      // 3: Load local variable 0
            Instruction::Goto(4),      // 4: Jump to instruction 6
            Instruction::Iload_1,      // 5: Load local variable 1
            Instruction::Ireturn,      // 6: Return value on stack
        ];
        let exception_table = Vec::new();

        // Create the control_flow_graph
        let (_entry_block, blocks, _) = get_blocks(
            &mut function_builder,
            &constant_pool,
            &instructions,
            &exception_table,
        )?;
        assert_eq!(blocks.len(), 4);
        let _block_0 = blocks.get(&0).expect("block0");
        let _block_5 = blocks.get(&5).expect("block5");
        let _block_9 = blocks.get(&9).expect("block9");
        let _block_10 = blocks.get(&10).expect("block10");
        Ok(())
    }
}
