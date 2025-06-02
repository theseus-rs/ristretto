use crate::Error::InternalError;
use crate::Result;
use crate::control_flow_graph::instruction;
use crate::control_flow_graph::type_stack::TypeStack;
use cranelift::prelude::{Block, FunctionBuilder, Type};
use ristretto_classfile::ConstantPool;
use ristretto_classfile::attributes::{ExceptionTableEntry, Instruction};
use std::collections::HashMap;

/// Creates a control flow graph of blocks for a function by analyzing the instructions.
///
/// This function analyzes Java bytecode instructions to create a Cranelift IR control flow graph
/// representation. It maps each instruction address that can be a target of control flow (branch
/// targets, exception handlers, etc.) to a Cranelift Block. The function also calculates the
/// operand stack state at each point and ensures consistent stack states across different paths to
/// the same instruction.
#[expect(clippy::too_many_lines)]
pub(crate) fn get_blocks(
    function_builder: &mut FunctionBuilder,
    constant_pool: &ConstantPool,
    instructions: &[Instruction],
    exception_table: &[ExceptionTableEntry],
) -> Result<HashMap<usize, Block>> {
    let mut blocks = HashMap::new();
    let mut stack_states: HashMap<usize, TypeStack> = HashMap::new();
    let exception_handler_addresses = exception_table
        .iter()
        .map(|entry| usize::from(entry.handler_pc))
        .collect::<Vec<_>>();
    let mut stack = TypeStack::new();

    // The first block is always the function entry point (with empty stack)
    blocks.insert(0, function_builder.create_block());
    stack_states.insert(0, stack.clone());

    for (program_counter, instruction) in instructions.iter().enumerate() {
        if exception_handler_addresses.contains(&program_counter) {
            // Push an object onto the stack for the exception object
            stack.push_object()?;
            insert_stack(&mut stack_states, program_counter, &stack)?;
            create_block_with_parameters(
                function_builder,
                &stack_states,
                program_counter,
                &mut blocks,
            );
        }
        if let Some(new_stack) = stack_states.get(&program_counter) {
            stack = new_stack.clone();
        }

        // Simulate the instruction to determine the stack state
        instruction::simulate(&mut stack, constant_pool, instruction)?;

        match instruction {
            Instruction::Ifeq(address)
            | Instruction::Ifne(address)
            | Instruction::Iflt(address)
            | Instruction::Ifge(address)
            | Instruction::Ifgt(address)
            | Instruction::Ifle(address)
            | Instruction::If_icmpeq(address)
            | Instruction::If_icmpne(address)
            | Instruction::If_icmplt(address)
            | Instruction::If_icmpge(address)
            | Instruction::If_icmpgt(address)
            | Instruction::If_icmple(address)
            | Instruction::If_acmpeq(address)
            | Instruction::If_acmpne(address)
            | Instruction::Ifnull(address)
            | Instruction::Ifnonnull(address) => {
                let then_address = usize::from(*address);
                insert_stack(&mut stack_states, then_address, &stack)?;
                create_block_with_parameters(
                    function_builder,
                    &stack_states,
                    then_address,
                    &mut blocks,
                );

                let Some(else_address) = program_counter.checked_add(1) else {
                    return Err(InternalError(format!(
                        "Address overflow: {program_counter} + 1"
                    )));
                };
                insert_stack(&mut stack_states, else_address, &stack)?;
                create_block_with_parameters(
                    function_builder,
                    &stack_states,
                    else_address,
                    &mut blocks,
                );
            }
            Instruction::Goto(address) | Instruction::Jsr(address) => {
                let address = usize::from(*address);
                insert_stack(&mut stack_states, address, &stack)?;
                create_block_with_parameters(function_builder, &stack_states, address, &mut blocks);
            }
            Instruction::Goto_w(address) | Instruction::Jsr_w(address) => {
                let address = usize::try_from(*address)?;
                insert_stack(&mut stack_states, address, &stack)?;
                create_block_with_parameters(function_builder, &stack_states, address, &mut blocks);
            }
            Instruction::Ret(address) => {
                let address = usize::from(*address);
                insert_stack(&mut stack_states, address, &stack)?;
                create_block_with_parameters(function_builder, &stack_states, address, &mut blocks);
            }
            Instruction::Ret_w(address) => {
                let address = usize::from(*address);
                insert_stack(&mut stack_states, address, &stack)?;
                create_block_with_parameters(function_builder, &stack_states, address, &mut blocks);
            }
            Instruction::Tableswitch {
                default, offsets, ..
            } => {
                let default = usize::try_from(*default)?;
                let default = program_counter.checked_add(default).ok_or_else(|| {
                    InternalError(format!(
                        "Invalid address calculation: {program_counter} + {default}"
                    ))
                })?;
                insert_stack(&mut stack_states, default, &stack)?;
                create_block_with_parameters(function_builder, &stack_states, default, &mut blocks);

                for offset in offsets {
                    let address = usize::try_from(*offset)?;
                    let address = program_counter.checked_add(address).ok_or_else(|| {
                        InternalError(format!(
                            "Invalid address calculation: {program_counter} + {address}"
                        ))
                    })?;
                    insert_stack(&mut stack_states, address, &stack)?;
                    create_block_with_parameters(
                        function_builder,
                        &stack_states,
                        address,
                        &mut blocks,
                    );
                }
            }
            Instruction::Lookupswitch { default, pairs } => {
                let default = usize::try_from(*default)?;
                let default = program_counter.checked_add(default).ok_or_else(|| {
                    InternalError(format!(
                        "Invalid address calculation: {program_counter} + {default}"
                    ))
                })?;
                insert_stack(&mut stack_states, default, &stack)?;
                create_block_with_parameters(function_builder, &stack_states, default, &mut blocks);

                for (_key, offset) in pairs {
                    let address = usize::try_from(*offset)?;
                    let address = program_counter.checked_add(address).ok_or_else(|| {
                        InternalError(format!(
                            "Invalid address calculation: {program_counter} + {address}"
                        ))
                    })?;
                    insert_stack(&mut stack_states, address, &stack)?;
                    create_block_with_parameters(
                        function_builder,
                        &stack_states,
                        address,
                        &mut blocks,
                    );
                }
            }
            _ => {}
        }
    }

    Ok(blocks)
}

/// Inserts a stack state for a specific address, ensuring consistency with any existing state.
///
/// This function is used to track the operand stack state at different points in the program. If a
/// state already exists for the address, it validates that the new state matches the existing one
/// to ensure that all paths to this instruction have compatible stack states.
pub(crate) fn insert_stack(
    stack_states: &mut HashMap<usize, TypeStack>,
    address: usize,
    stack: &TypeStack,
) -> Result<()> {
    match stack_states.get(&address) {
        Some(entry_stack) => {
            if entry_stack != stack {
                return Err(InternalError(format!(
                    "Invalid stack state for address {address}, entry_stack={entry_stack:?} and stack={stack:?}"
                )));
            }
        }
        None => {
            stack_states.insert(address, stack.clone());
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
    stack_states: &HashMap<usize, TypeStack>,
    address: usize,
    blocks: &mut HashMap<usize, Block>,
) {
    blocks.entry(address).or_insert_with(|| {
        let block = function_builder.create_block();

        if let Some(stack_types) = stack_states.get(&address) {
            let stack_types = stack_types.to_vec();
            append_block_params(function_builder, block, &stack_types);
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
            Instruction::If_icmplt(5), // 2: If var0 < var1, branch to instruction 5
            Instruction::Iload_0,      // 3: Load local variable 0
            Instruction::Goto(6),      // 4: Jump to instruction 6
            Instruction::Iload_1,      // 5: Load local variable 1
            Instruction::Ireturn,      // 6: Return value on stack
        ];
        let exception_table = Vec::new();

        // Create the control_flow_graph
        let blocks = get_blocks(
            &mut function_builder,
            &constant_pool,
            &instructions,
            &exception_table,
        )?;
        assert_eq!(blocks.len(), 4);
        let _block_0 = blocks.get(&0).expect("block0");
        let _block_3 = blocks.get(&3).expect("block3");
        let _block_5 = blocks.get(&5).expect("block5");
        let _block_6 = blocks.get(&6).expect("block6");
        Ok(())
    }
}
