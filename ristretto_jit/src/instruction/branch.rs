use crate::Error::{InternalError, InvalidBlockAddress};
use crate::Result;
use crate::control_flow_graph::append_block_params;
use crate::instruction::TRAP_INTERNAL_ERROR;
use crate::local_variables::LocalVariables;
use crate::operand_stack::OperandStack;
use cranelift::frontend::FunctionBuilder;
use cranelift::prelude::{Block, InstBuilder, IntCC, TrapCode, Value, types};
use std::collections::HashMap;

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.if_cond>
pub(crate) fn ifeq(
    function_builder: &mut FunctionBuilder,
    blocks: &HashMap<usize, Block>,
    stack: &mut OperandStack,
    program_counter: usize,
    address: u16,
) -> Result<()> {
    let value1 = stack.pop()?;
    let address = usize::from(address);

    let then_block = blocks
        .get(&address)
        .ok_or_else(|| InvalidBlockAddress(address))?;
    let else_address = program_counter
        .checked_add(1)
        .ok_or_else(|| InvalidBlockAddress(address))?;
    let else_block = blocks
        .get(&else_address)
        .ok_or_else(|| InvalidBlockAddress(address))?;

    let value2 = function_builder.ins().iconst(types::I32, 0);
    let condition_value = function_builder.ins().icmp(IntCC::Equal, value1, value2);
    let block_arguments = stack.as_block_arguments();
    function_builder.ins().brif(
        condition_value,
        *then_block,
        &block_arguments,
        *else_block,
        &block_arguments,
    );
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.if_cond>
pub(crate) fn ifne(
    function_builder: &mut FunctionBuilder,
    blocks: &HashMap<usize, Block>,
    stack: &mut OperandStack,
    program_counter: usize,
    address: u16,
) -> Result<()> {
    let value1 = stack.pop()?;
    let address = usize::from(address);

    let then_block = blocks
        .get(&address)
        .ok_or_else(|| InvalidBlockAddress(address))?;
    let else_address = program_counter
        .checked_add(1)
        .ok_or_else(|| InvalidBlockAddress(address))?;
    let else_block = blocks
        .get(&else_address)
        .ok_or_else(|| InvalidBlockAddress(address))?;

    let value2 = function_builder.ins().iconst(types::I32, 0);
    let condition_value = function_builder.ins().icmp(IntCC::NotEqual, value1, value2);
    let block_arguments = stack.as_block_arguments();
    function_builder.ins().brif(
        condition_value,
        *then_block,
        &block_arguments,
        *else_block,
        &block_arguments,
    );
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.if_cond>
pub(crate) fn iflt(
    function_builder: &mut FunctionBuilder,
    blocks: &HashMap<usize, Block>,
    stack: &mut OperandStack,
    program_counter: usize,
    address: u16,
) -> Result<()> {
    let value1 = stack.pop()?;
    let address = usize::from(address);

    let then_block = blocks
        .get(&address)
        .ok_or_else(|| InvalidBlockAddress(address))?;
    let else_address = program_counter
        .checked_add(1)
        .ok_or_else(|| InvalidBlockAddress(address))?;
    let else_block = blocks
        .get(&else_address)
        .ok_or_else(|| InvalidBlockAddress(address))?;

    let value2 = function_builder.ins().iconst(types::I32, 0);
    let condition_value = function_builder
        .ins()
        .icmp(IntCC::SignedLessThan, value1, value2);
    let block_arguments = stack.as_block_arguments();
    function_builder.ins().brif(
        condition_value,
        *then_block,
        &block_arguments,
        *else_block,
        &block_arguments,
    );
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.if_cond>
pub(crate) fn ifge(
    function_builder: &mut FunctionBuilder,
    blocks: &HashMap<usize, Block>,
    stack: &mut OperandStack,
    program_counter: usize,
    address: u16,
) -> Result<()> {
    let value1 = stack.pop()?;
    let address = usize::from(address);

    let then_block = blocks
        .get(&address)
        .ok_or_else(|| InvalidBlockAddress(address))?;
    let else_address = program_counter
        .checked_add(1)
        .ok_or_else(|| InvalidBlockAddress(address))?;
    let else_block = blocks
        .get(&else_address)
        .ok_or_else(|| InvalidBlockAddress(address))?;

    let value2 = function_builder.ins().iconst(types::I32, 0);
    let condition_value =
        function_builder
            .ins()
            .icmp(IntCC::SignedGreaterThanOrEqual, value1, value2);
    let block_arguments = stack.as_block_arguments();
    function_builder.ins().brif(
        condition_value,
        *then_block,
        &block_arguments,
        *else_block,
        &block_arguments,
    );
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.if_cond>
pub(crate) fn ifgt(
    function_builder: &mut FunctionBuilder,
    blocks: &HashMap<usize, Block>,
    stack: &mut OperandStack,
    program_counter: usize,
    address: u16,
) -> Result<()> {
    let value1 = stack.pop()?;
    let address = usize::from(address);

    let then_block = blocks
        .get(&address)
        .ok_or_else(|| InvalidBlockAddress(address))?;
    let else_address = program_counter
        .checked_add(1)
        .ok_or_else(|| InvalidBlockAddress(address))?;
    let else_block = blocks
        .get(&else_address)
        .ok_or_else(|| InvalidBlockAddress(address))?;

    let value2 = function_builder.ins().iconst(types::I32, 0);
    let condition_value = function_builder
        .ins()
        .icmp(IntCC::SignedGreaterThan, value1, value2);
    let block_arguments = stack.as_block_arguments();
    function_builder.ins().brif(
        condition_value,
        *then_block,
        &block_arguments,
        *else_block,
        &block_arguments,
    );
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.if_cond>
pub(crate) fn ifle(
    function_builder: &mut FunctionBuilder,
    blocks: &HashMap<usize, Block>,
    stack: &mut OperandStack,
    program_counter: usize,
    address: u16,
) -> Result<()> {
    let value1 = stack.pop()?;
    let address = usize::from(address);

    let then_block = blocks
        .get(&address)
        .ok_or_else(|| InvalidBlockAddress(address))?;
    let else_address = program_counter
        .checked_add(1)
        .ok_or_else(|| InvalidBlockAddress(address))?;
    let else_block = blocks
        .get(&else_address)
        .ok_or_else(|| InvalidBlockAddress(address))?;

    let value2 = function_builder.ins().iconst(types::I32, 0);
    let condition_value = function_builder
        .ins()
        .icmp(IntCC::SignedLessThanOrEqual, value1, value2);
    let block_arguments = stack.as_block_arguments();
    function_builder.ins().brif(
        condition_value,
        *then_block,
        &block_arguments,
        *else_block,
        &block_arguments,
    );
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.if_icmp_cond>
pub(crate) fn if_icmpeq(
    function_builder: &mut FunctionBuilder,
    blocks: &HashMap<usize, Block>,
    stack: &mut OperandStack,
    program_counter: usize,
    address: u16,
) -> Result<()> {
    let value2 = stack.pop()?;
    let value1 = stack.pop()?;
    let address = usize::from(address);

    let then_block = blocks
        .get(&address)
        .ok_or_else(|| InvalidBlockAddress(address))?;
    let else_address = program_counter
        .checked_add(1)
        .ok_or_else(|| InvalidBlockAddress(address))?;
    let else_block = blocks
        .get(&else_address)
        .ok_or_else(|| InvalidBlockAddress(address))?;

    let condition_value = function_builder.ins().icmp(IntCC::Equal, value1, value2);
    let block_arguments = stack.as_block_arguments();
    function_builder.ins().brif(
        condition_value,
        *then_block,
        &block_arguments,
        *else_block,
        &block_arguments,
    );
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.if_icmp_cond>
pub(crate) fn if_icmpne(
    function_builder: &mut FunctionBuilder,
    blocks: &HashMap<usize, Block>,
    stack: &mut OperandStack,
    program_counter: usize,
    address: u16,
) -> Result<()> {
    let value2 = stack.pop()?;
    let value1 = stack.pop()?;
    let address = usize::from(address);

    let then_block = blocks
        .get(&address)
        .ok_or_else(|| InvalidBlockAddress(address))?;
    let else_address = program_counter
        .checked_add(1)
        .ok_or_else(|| InvalidBlockAddress(address))?;
    let else_block = blocks
        .get(&else_address)
        .ok_or_else(|| InvalidBlockAddress(address))?;

    let condition_value = function_builder.ins().icmp(IntCC::NotEqual, value1, value2);
    let block_arguments = stack.as_block_arguments();
    function_builder.ins().brif(
        condition_value,
        *then_block,
        &block_arguments,
        *else_block,
        &block_arguments,
    );
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.if_icmp_cond>
pub(crate) fn if_icmplt(
    function_builder: &mut FunctionBuilder,
    blocks: &HashMap<usize, Block>,
    stack: &mut OperandStack,
    program_counter: usize,
    address: u16,
) -> Result<()> {
    let value2 = stack.pop()?;
    let value1 = stack.pop()?;
    let address = usize::from(address);

    let then_block = blocks
        .get(&address)
        .ok_or_else(|| InvalidBlockAddress(address))?;
    let else_address = program_counter
        .checked_add(1)
        .ok_or_else(|| InvalidBlockAddress(address))?;
    let else_block = blocks
        .get(&else_address)
        .ok_or_else(|| InvalidBlockAddress(else_address))?;

    let condition_value = function_builder
        .ins()
        .icmp(IntCC::SignedLessThan, value1, value2);
    let block_arguments = stack.as_block_arguments();
    function_builder.ins().brif(
        condition_value,
        *then_block,
        &block_arguments,
        *else_block,
        &block_arguments,
    );
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.if_icmp_cond>
pub(crate) fn if_icmpge(
    function_builder: &mut FunctionBuilder,
    blocks: &HashMap<usize, Block>,
    stack: &mut OperandStack,
    program_counter: usize,
    address: u16,
) -> Result<()> {
    let value2 = stack.pop()?;
    let value1 = stack.pop()?;
    let address = usize::from(address);

    let then_block = blocks
        .get(&address)
        .ok_or_else(|| InvalidBlockAddress(address))?;
    let else_address = program_counter
        .checked_add(1)
        .ok_or_else(|| InvalidBlockAddress(address))?;
    let else_block = blocks
        .get(&else_address)
        .ok_or_else(|| InvalidBlockAddress(address))?;

    let condition_value =
        function_builder
            .ins()
            .icmp(IntCC::SignedGreaterThanOrEqual, value1, value2);
    let block_arguments = stack.as_block_arguments();
    function_builder.ins().brif(
        condition_value,
        *then_block,
        &block_arguments,
        *else_block,
        &block_arguments,
    );
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.if_icmp_cond>
pub(crate) fn if_icmpgt(
    function_builder: &mut FunctionBuilder,
    blocks: &HashMap<usize, Block>,
    stack: &mut OperandStack,
    program_counter: usize,
    address: u16,
) -> Result<()> {
    let value2 = stack.pop()?;
    let value1 = stack.pop()?;
    let address = usize::from(address);

    let then_block = blocks
        .get(&address)
        .ok_or_else(|| InvalidBlockAddress(address))?;
    let else_address = program_counter
        .checked_add(1)
        .ok_or_else(|| InvalidBlockAddress(address))?;
    let else_block = blocks
        .get(&else_address)
        .ok_or_else(|| InvalidBlockAddress(address))?;

    let condition_value = function_builder
        .ins()
        .icmp(IntCC::SignedGreaterThan, value1, value2);
    let block_arguments = stack.as_block_arguments();
    function_builder.ins().brif(
        condition_value,
        *then_block,
        &block_arguments,
        *else_block,
        &block_arguments,
    );
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.if_icmp_cond>
pub(crate) fn if_icmple(
    function_builder: &mut FunctionBuilder,
    blocks: &HashMap<usize, Block>,
    stack: &mut OperandStack,
    program_counter: usize,
    address: u16,
) -> Result<()> {
    let value2 = stack.pop()?;
    let value1 = stack.pop()?;
    let address = usize::from(address);

    let then_block = blocks
        .get(&address)
        .ok_or_else(|| InvalidBlockAddress(address))?;
    let else_address = program_counter
        .checked_add(1)
        .ok_or_else(|| InvalidBlockAddress(address))?;
    let else_block = blocks
        .get(&else_address)
        .ok_or_else(|| InvalidBlockAddress(address))?;

    let condition_value = function_builder
        .ins()
        .icmp(IntCC::SignedLessThanOrEqual, value1, value2);
    let block_arguments = stack.as_block_arguments();
    function_builder.ins().brif(
        condition_value,
        *then_block,
        &block_arguments,
        *else_block,
        &block_arguments,
    );
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.goto>
pub(crate) fn goto(
    function_builder: &mut FunctionBuilder,
    blocks: &HashMap<usize, Block>,
    stack: &mut OperandStack,
    address: u16,
) -> Result<()> {
    let address = i32::from(address);
    goto_w(function_builder, blocks, stack, address)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.goto_w>
/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.wide>
pub(crate) fn goto_w(
    function_builder: &mut FunctionBuilder,
    blocks: &HashMap<usize, Block>,
    stack: &mut OperandStack,
    address: i32,
) -> Result<()> {
    let address = usize::try_from(address)?;

    let block = blocks
        .get(&address)
        .ok_or_else(|| InvalidBlockAddress(address))?;

    let block_arguments = stack.as_block_arguments();
    function_builder.ins().jump(*block, &block_arguments);
    stack.reset(function_builder)?;
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.jsr>
pub(crate) fn jsr(
    function_builder: &mut FunctionBuilder,
    blocks: &HashMap<usize, Block>,
    stack: &mut OperandStack,
    program_counter: usize,
    address: u16,
) -> Result<()> {
    let address = i32::from(address);
    jsr_w(function_builder, blocks, stack, program_counter, address)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.jsr>
/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.wide>
pub(crate) fn jsr_w(
    function_builder: &mut FunctionBuilder,
    blocks: &HashMap<usize, Block>,
    stack: &mut OperandStack,
    program_counter: usize,
    address: i32,
) -> Result<()> {
    let next_address = i64::try_from(program_counter)?;
    let next_address = next_address
        .checked_add(1)
        .ok_or_else(|| InvalidBlockAddress(program_counter))?;
    let value = function_builder.ins().iconst(types::I32, next_address);
    stack.push(value)?;

    let address = usize::try_from(address)?;
    let block = blocks
        .get(&address)
        .ok_or_else(|| InvalidBlockAddress(address))?;
    let block_arguments = stack.as_block_arguments();
    function_builder.ins().jump(*block, &block_arguments);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.ret>
pub(crate) fn ret(
    function_builder: &mut FunctionBuilder,
    blocks: &HashMap<usize, Block>,
    locals: &LocalVariables,
    stack: &mut OperandStack,
    index: u8,
) -> Result<()> {
    let index = u16::from(index);
    ret_w(function_builder, blocks, locals, stack, index)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.ret>
/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.wide>
pub(crate) fn ret_w(
    function_builder: &mut FunctionBuilder,
    blocks: &HashMap<usize, Block>,
    locals: &LocalVariables,
    stack: &mut OperandStack,
    index: u16,
) -> Result<()> {
    let index = usize::from(index);
    let return_address = locals.get_int(function_builder, index)?;

    let mut sorted_block_entries: Vec<(&usize, &Block)> = blocks.iter().collect();
    sorted_block_entries.sort_by_key(|(address, _)| *address);
    let block_arguments = stack.as_block_arguments();

    for (index, (address, block)) in sorted_block_entries.iter().enumerate() {
        // If the address is 0, it means this is the entry block, which should not be used.
        if **address == 0 {
            continue;
        }

        let stack_types = stack.to_type_vec(function_builder);
        let else_block = function_builder.create_block();
        append_block_params(function_builder, else_block, &stack_types);

        let address = i64::try_from(**address)?;
        let block_address = function_builder.ins().iconst(types::I32, address);
        let condition_value =
            function_builder
                .ins()
                .icmp(IntCC::Equal, return_address, block_address);

        function_builder.ins().brif(
            condition_value,
            **block,
            &block_arguments,
            else_block,
            &block_arguments,
        );

        function_builder.switch_to_block(else_block);
        // If this is the last block, create a trap, indicating an invalid RET address.
        if index == sorted_block_entries.len() - 1 {
            let Some(trap_code) = TrapCode::user(TRAP_INTERNAL_ERROR) else {
                return Err(InternalError("Failed to create user trap code".to_string()));
            };
            function_builder.ins().trap(trap_code);
        }
    }

    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.return>
pub(crate) fn r#return(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
    _return_pointer: Value,
) {
    // This optimization relies on the fact that when returning from a "void" method, the default
    // return pointer values should be initialized to 0 which is the same as the following code:
    // let value = function_builder.ins().iconst(types::I64, 0);
    // let discriminate = i64::from(jit_value::NONE);
    // let discriminate = function_builder.ins().iconst(types::I8, discriminate);
    // function_builder
    //     .ins()
    //     .store(MemFlags::new(), discriminate, return_pointer, 0);
    // function_builder
    //     .ins()
    //     .store(MemFlags::new(), value, return_pointer, 8);
    let stack_values = stack.as_slice();
    function_builder.ins().return_(stack_values);
}
