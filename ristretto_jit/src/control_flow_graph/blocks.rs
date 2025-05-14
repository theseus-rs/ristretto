use crate::Error::InternalError;
use crate::Result;
use crate::control_flow_graph::instruction;
use crate::control_flow_graph::type_stack::TypeStack;
use cranelift::prelude::{Block, FunctionBuilder};
use ristretto_classfile::ConstantPool;
use ristretto_classfile::attributes::Instruction;
use std::collections::HashMap;

/// Creates a control flow graph of blocks for a function by analyzing the instructions.
///
/// # Arguments
/// * `function_builder` - The function builder to create blocks with
/// * `constant_pool` - The constant pool for the class
/// * `instructions` - The Java bytecode instructions
///
/// # Returns
/// A map from instruction addresses to Cranelift blocks
///
/// # Errors
/// * If the address calculation overflows
/// * If the address is not valid
#[expect(clippy::too_many_lines)]
pub(crate) fn get_blocks(
    function_builder: &mut FunctionBuilder,
    constant_pool: &ConstantPool,
    instructions: &[Instruction],
) -> Result<HashMap<usize, Block>> {
    let mut blocks = HashMap::new();
    let mut stack_states: HashMap<usize, TypeStack> = HashMap::new();
    let mut stack = TypeStack::new();

    // The first block is always the function entry point (with empty stack)
    blocks.insert(0, function_builder.create_block());
    stack_states.insert(0, stack.clone());

    for (program_counter, instruction) in instructions.iter().enumerate() {
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
            | Instruction::If_icmple(address) => {
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

/// Inserts stack for address
pub fn insert_stack(
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

/// Utility function to create a block with parameters matching the expected stack state
fn create_block_with_parameters(
    function_builder: &mut FunctionBuilder,
    stack_states: &HashMap<usize, TypeStack>,
    address: usize,
    blocks: &mut HashMap<usize, Block>,
) {
    blocks.entry(address).or_insert_with(|| {
        let block = function_builder.create_block();

        if let Some(stack_types) = stack_states.get(&address) {
            let stack_types = stack_types.to_vec();
            for value_type in stack_types {
                function_builder.append_block_param(block, value_type);
            }
        }

        block
    });
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

        // Create the control_flow_graph
        let blocks = get_blocks(&mut function_builder, &constant_pool, &instructions)?;
        assert_eq!(blocks.len(), 4);
        let _block_0 = blocks.get(&0).expect("block0");
        let _block_3 = blocks.get(&3).expect("block3");
        let _block_5 = blocks.get(&5).expect("block5");
        let _block_6 = blocks.get(&6).expect("block6");
        Ok(())
    }
}
