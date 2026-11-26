use crate::Error::InvalidBlockAddress;
use crate::Result;
use crate::control_flow_graph::append_block_params;
use crate::instruction::TRAP_INTERNAL_ERROR;
use crate::local_variables::LocalVariables;
use crate::operand_stack::OperandStack;
use ahash::AHashMap;
use cranelift::frontend::{FunctionBuilder, Switch};
use cranelift::prelude::{Block, InstBuilder, IntCC, TrapCode, Value, types};
use ristretto_classfile::attributes::{LookupSwitch, TableSwitch};

const SHORT_BRANCH_BYTES: usize = size_of::<u8>() + size_of::<i16>();
const WIDE_BRANCH_BYTES: usize = size_of::<u8>() + size_of::<i32>();

/// # References
///
/// - [JVMS §6.5.if_cond](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.if_cond)
pub(crate) fn ifeq(
    function_builder: &mut FunctionBuilder,
    blocks: &AHashMap<usize, Block>,
    stack: &mut OperandStack,
    program_counter: usize,
    address: i16,
) -> Result<()> {
    let value1 = stack.pop()?;
    let address = usize::try_from(i64::try_from(program_counter)? + i64::from(address))?;

    let then_block = blocks
        .get(&address)
        .ok_or_else(|| InvalidBlockAddress(address))?;
    let else_address = program_counter
        .checked_add(SHORT_BRANCH_BYTES)
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

/// # References
///
/// - [JVMS §6.5.if_cond](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.if_cond)
pub(crate) fn ifne(
    function_builder: &mut FunctionBuilder,
    blocks: &AHashMap<usize, Block>,
    stack: &mut OperandStack,
    program_counter: usize,
    address: i16,
) -> Result<()> {
    let value1 = stack.pop()?;
    let address = usize::try_from(i64::try_from(program_counter)? + i64::from(address))?;

    let then_block = blocks
        .get(&address)
        .ok_or_else(|| InvalidBlockAddress(address))?;
    let else_address = program_counter
        .checked_add(SHORT_BRANCH_BYTES)
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

/// # References
///
/// - [JVMS §6.5.if_cond](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.if_cond)
pub(crate) fn iflt(
    function_builder: &mut FunctionBuilder,
    blocks: &AHashMap<usize, Block>,
    stack: &mut OperandStack,
    program_counter: usize,
    address: i16,
) -> Result<()> {
    let value1 = stack.pop()?;
    let address = usize::try_from(i64::try_from(program_counter)? + i64::from(address))?;

    let then_block = blocks
        .get(&address)
        .ok_or_else(|| InvalidBlockAddress(address))?;
    let else_address = program_counter
        .checked_add(SHORT_BRANCH_BYTES)
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

/// # References
///
/// - [JVMS §6.5.if_cond](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.if_cond)
pub(crate) fn ifge(
    function_builder: &mut FunctionBuilder,
    blocks: &AHashMap<usize, Block>,
    stack: &mut OperandStack,
    program_counter: usize,
    address: i16,
) -> Result<()> {
    let value1 = stack.pop()?;
    let address = usize::try_from(i64::try_from(program_counter)? + i64::from(address))?;

    let then_block = blocks
        .get(&address)
        .ok_or_else(|| InvalidBlockAddress(address))?;
    let else_address = program_counter
        .checked_add(SHORT_BRANCH_BYTES)
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

/// # References
///
/// - [JVMS §6.5.if_cond](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.if_cond)
pub(crate) fn ifgt(
    function_builder: &mut FunctionBuilder,
    blocks: &AHashMap<usize, Block>,
    stack: &mut OperandStack,
    program_counter: usize,
    address: i16,
) -> Result<()> {
    let value1 = stack.pop()?;
    let address = usize::try_from(i64::try_from(program_counter)? + i64::from(address))?;

    let then_block = blocks
        .get(&address)
        .ok_or_else(|| InvalidBlockAddress(address))?;
    let else_address = program_counter
        .checked_add(SHORT_BRANCH_BYTES)
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

/// # References
///
/// - [JVMS §6.5.if_cond](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.if_cond)
pub(crate) fn ifle(
    function_builder: &mut FunctionBuilder,
    blocks: &AHashMap<usize, Block>,
    stack: &mut OperandStack,
    program_counter: usize,
    address: i16,
) -> Result<()> {
    let value1 = stack.pop()?;
    let address = usize::try_from(i64::try_from(program_counter)? + i64::from(address))?;

    let then_block = blocks
        .get(&address)
        .ok_or_else(|| InvalidBlockAddress(address))?;
    let else_address = program_counter
        .checked_add(SHORT_BRANCH_BYTES)
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

/// # References
///
/// - [JVMS §6.5.if_icmp_cond](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.if_icmp_cond)
pub(crate) fn if_icmpeq(
    function_builder: &mut FunctionBuilder,
    blocks: &AHashMap<usize, Block>,
    stack: &mut OperandStack,
    program_counter: usize,
    address: i16,
) -> Result<()> {
    let value2 = stack.pop()?;
    let value1 = stack.pop()?;
    let address = usize::try_from(i64::try_from(program_counter)? + i64::from(address))?;

    let then_block = blocks
        .get(&address)
        .ok_or_else(|| InvalidBlockAddress(address))?;
    let else_address = program_counter
        .checked_add(SHORT_BRANCH_BYTES)
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

/// # References
///
/// - [JVMS §6.5.if_icmp_cond](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.if_icmp_cond)
pub(crate) fn if_icmpne(
    function_builder: &mut FunctionBuilder,
    blocks: &AHashMap<usize, Block>,
    stack: &mut OperandStack,
    program_counter: usize,
    address: i16,
) -> Result<()> {
    let value2 = stack.pop()?;
    let value1 = stack.pop()?;
    let address = usize::try_from(i64::try_from(program_counter)? + i64::from(address))?;

    let then_block = blocks
        .get(&address)
        .ok_or_else(|| InvalidBlockAddress(address))?;
    let else_address = program_counter
        .checked_add(SHORT_BRANCH_BYTES)
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

/// # References
///
/// - [JVMS §6.5.if_icmp_cond](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.if_icmp_cond)
pub(crate) fn if_icmplt(
    function_builder: &mut FunctionBuilder,
    blocks: &AHashMap<usize, Block>,
    stack: &mut OperandStack,
    program_counter: usize,
    address: i16,
) -> Result<()> {
    let value2 = stack.pop()?;
    let value1 = stack.pop()?;
    let address = usize::try_from(i64::try_from(program_counter)? + i64::from(address))?;

    let then_block = blocks
        .get(&address)
        .ok_or_else(|| InvalidBlockAddress(address))?;
    let else_address = program_counter
        .checked_add(SHORT_BRANCH_BYTES)
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

/// # References
///
/// - [JVMS §6.5.if_icmp_cond](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.if_icmp_cond)
pub(crate) fn if_icmpge(
    function_builder: &mut FunctionBuilder,
    blocks: &AHashMap<usize, Block>,
    stack: &mut OperandStack,
    program_counter: usize,
    address: i16,
) -> Result<()> {
    let value2 = stack.pop()?;
    let value1 = stack.pop()?;
    let address = usize::try_from(i64::try_from(program_counter)? + i64::from(address))?;

    let then_block = blocks
        .get(&address)
        .ok_or_else(|| InvalidBlockAddress(address))?;
    let else_address = program_counter
        .checked_add(SHORT_BRANCH_BYTES)
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

/// # References
///
/// - [JVMS §6.5.if_icmp_cond](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.if_icmp_cond)
pub(crate) fn if_icmpgt(
    function_builder: &mut FunctionBuilder,
    blocks: &AHashMap<usize, Block>,
    stack: &mut OperandStack,
    program_counter: usize,
    address: i16,
) -> Result<()> {
    let value2 = stack.pop()?;
    let value1 = stack.pop()?;
    let address = usize::try_from(i64::try_from(program_counter)? + i64::from(address))?;

    let then_block = blocks
        .get(&address)
        .ok_or_else(|| InvalidBlockAddress(address))?;
    let else_address = program_counter
        .checked_add(SHORT_BRANCH_BYTES)
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

/// # References
///
/// - [JVMS §6.5.if_icmp_cond](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.if_icmp_cond)
pub(crate) fn if_icmple(
    function_builder: &mut FunctionBuilder,
    blocks: &AHashMap<usize, Block>,
    stack: &mut OperandStack,
    program_counter: usize,
    address: i16,
) -> Result<()> {
    let value2 = stack.pop()?;
    let value1 = stack.pop()?;
    let address = usize::try_from(i64::try_from(program_counter)? + i64::from(address))?;

    let then_block = blocks
        .get(&address)
        .ok_or_else(|| InvalidBlockAddress(address))?;
    let else_address = program_counter
        .checked_add(SHORT_BRANCH_BYTES)
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

/// # References
///
/// - [JVMS §6.5.if_acmp_cond](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.if_acmp_cond)
pub(crate) fn if_acmpeq(
    function_builder: &mut FunctionBuilder,
    blocks: &AHashMap<usize, Block>,
    stack: &mut OperandStack,
    program_counter: usize,
    address: i16,
) -> Result<()> {
    let value2 = stack.pop()?;
    let value1 = stack.pop()?;
    let address = usize::try_from(i64::try_from(program_counter)? + i64::from(address))?;

    let then_block = blocks
        .get(&address)
        .ok_or_else(|| InvalidBlockAddress(address))?;
    let else_address = program_counter
        .checked_add(SHORT_BRANCH_BYTES)
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

/// # References
///
/// - [JVMS §6.5.if_acmp_cond](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.if_acmp_cond)
pub(crate) fn if_acmpne(
    function_builder: &mut FunctionBuilder,
    blocks: &AHashMap<usize, Block>,
    stack: &mut OperandStack,
    program_counter: usize,
    address: i16,
) -> Result<()> {
    let value2 = stack.pop()?;
    let value1 = stack.pop()?;
    let address = usize::try_from(i64::try_from(program_counter)? + i64::from(address))?;

    let then_block = blocks
        .get(&address)
        .ok_or_else(|| InvalidBlockAddress(address))?;
    let else_address = program_counter
        .checked_add(SHORT_BRANCH_BYTES)
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

/// # References
///
/// - [JVMS §6.5.ifnull](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.ifnull)
pub(crate) fn ifnull(
    function_builder: &mut FunctionBuilder,
    blocks: &AHashMap<usize, Block>,
    stack: &mut OperandStack,
    program_counter: usize,
    address: i16,
) -> Result<()> {
    let value = stack.pop()?;
    let address = usize::try_from(i64::try_from(program_counter)? + i64::from(address))?;

    let then_block = blocks
        .get(&address)
        .ok_or_else(|| InvalidBlockAddress(address))?;
    let else_address = program_counter
        .checked_add(SHORT_BRANCH_BYTES)
        .ok_or_else(|| InvalidBlockAddress(address))?;
    let else_block = blocks
        .get(&else_address)
        .ok_or_else(|| InvalidBlockAddress(address))?;

    let null = function_builder.ins().iconst(types::I64, 0);
    let condition_value = function_builder.ins().icmp(IntCC::Equal, value, null);
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

/// # References
///
/// - [JVMS §6.5.ifnonnull](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.ifnonnull)
pub(crate) fn ifnonnull(
    function_builder: &mut FunctionBuilder,
    blocks: &AHashMap<usize, Block>,
    stack: &mut OperandStack,
    program_counter: usize,
    address: i16,
) -> Result<()> {
    let value = stack.pop()?;
    let address = usize::try_from(i64::try_from(program_counter)? + i64::from(address))?;

    let then_block = blocks
        .get(&address)
        .ok_or_else(|| InvalidBlockAddress(address))?;
    let else_address = program_counter
        .checked_add(SHORT_BRANCH_BYTES)
        .ok_or_else(|| InvalidBlockAddress(address))?;
    let else_block = blocks
        .get(&else_address)
        .ok_or_else(|| InvalidBlockAddress(address))?;

    let null = function_builder.ins().iconst(types::I64, 0);
    let condition_value = function_builder.ins().icmp(IntCC::NotEqual, value, null);
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

/// # References
///
/// - [JVMS §6.5.goto](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.goto)
pub(crate) fn goto(
    function_builder: &mut FunctionBuilder,
    blocks: &AHashMap<usize, Block>,
    stack: &mut OperandStack,
    program_counter: usize,
    address: i16,
) -> Result<()> {
    let address = i32::from(address);
    goto_w(function_builder, blocks, stack, program_counter, address)
}

/// # References
///
/// - [JVMS §6.5.goto_w](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.goto_w)
/// - [JVMS §6.5.wide](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.wide)
pub(crate) fn goto_w(
    function_builder: &mut FunctionBuilder,
    blocks: &AHashMap<usize, Block>,
    stack: &mut OperandStack,
    program_counter: usize,
    address: i32,
) -> Result<()> {
    let address = usize::try_from(i64::try_from(program_counter)? + i64::from(address))?;
    let block = blocks
        .get(&address)
        .ok_or_else(|| InvalidBlockAddress(address))?;
    let block_arguments = stack.as_block_arguments();
    function_builder.ins().jump(*block, &block_arguments);
    stack.reset(function_builder)?;
    Ok(())
}

/// # References
///
/// - [JVMS §6.5.jsr](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.jsr)
pub(crate) fn jsr(
    function_builder: &mut FunctionBuilder,
    blocks: &AHashMap<usize, Block>,
    stack: &mut OperandStack,
    program_counter: usize,
    address: i16,
) -> Result<()> {
    jsr_at(
        function_builder,
        blocks,
        stack,
        program_counter,
        i32::from(address),
        SHORT_BRANCH_BYTES,
    )
}

/// # References
///
/// - [JVMS §6.5.jsr](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.jsr)
/// - [JVMS §6.5.wide](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.wide)
pub(crate) fn jsr_w(
    function_builder: &mut FunctionBuilder,
    blocks: &AHashMap<usize, Block>,
    stack: &mut OperandStack,
    program_counter: usize,
    address: i32,
) -> Result<()> {
    jsr_at(
        function_builder,
        blocks,
        stack,
        program_counter,
        address,
        WIDE_BRANCH_BYTES,
    )
}

fn jsr_at(
    function_builder: &mut FunctionBuilder,
    blocks: &AHashMap<usize, Block>,
    stack: &mut OperandStack,
    program_counter: usize,
    address: i32,
    length: usize,
) -> Result<()> {
    let next_address = program_counter
        .checked_add(length)
        .ok_or_else(|| InvalidBlockAddress(program_counter))?;
    let value = function_builder
        .ins()
        .iconst(types::I32, i64::try_from(next_address)?);
    stack.push(value)?;

    let address = usize::try_from(i64::try_from(program_counter)? + i64::from(address))?;
    let block = blocks
        .get(&address)
        .ok_or_else(|| InvalidBlockAddress(address))?;
    let block_arguments = stack.as_block_arguments();
    function_builder.ins().jump(*block, &block_arguments);
    Ok(())
}

/// # References
///
/// - [JVMS §6.5.ret](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.ret)
pub(crate) fn ret(
    function_builder: &mut FunctionBuilder,
    blocks: &AHashMap<usize, Block>,
    locals: &LocalVariables,
    stack: &mut OperandStack,
    index: u8,
) -> Result<()> {
    let index = u16::from(index);
    ret_w(function_builder, blocks, locals, stack, index)
}

/// # References
///
/// - [JVMS §6.5.ret](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.ret)
/// - [JVMS §6.5.wide](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.wide)
pub(crate) fn ret_w(
    function_builder: &mut FunctionBuilder,
    blocks: &AHashMap<usize, Block>,
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
            let trap_code = TrapCode::unwrap_user(TRAP_INTERNAL_ERROR);
            function_builder.ins().trap(trap_code);
        }
    }

    Ok(())
}

/// # References
///
/// - [JVMS §6.5.tableswitch](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.tableswitch)
pub(crate) fn tableswitch(
    function_builder: &mut FunctionBuilder,
    blocks: &AHashMap<usize, Block>,
    stack: &mut OperandStack,
    program_counter: usize,
    table_switch: &TableSwitch,
) -> Result<()> {
    let cases = table_switch
        .offsets
        .iter()
        .enumerate()
        .map(|(index, &offset)| {
            let key = i32::try_from(i64::from(table_switch.low) + i64::try_from(index)?)?;
            Ok((key, offset))
        })
        .collect::<Result<Vec<_>>>()?;
    emit_switch(
        function_builder,
        blocks,
        stack,
        program_counter,
        table_switch.default,
        cases,
    )
}

/// # References
///
/// - [JVMS §6.5.lookupswitch](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.lookupswitch)
pub(crate) fn lookupswitch(
    function_builder: &mut FunctionBuilder,
    blocks: &AHashMap<usize, Block>,
    stack: &mut OperandStack,
    program_counter: usize,
    lookup_switch: &LookupSwitch,
) -> Result<()> {
    emit_switch(
        function_builder,
        blocks,
        stack,
        program_counter,
        lookup_switch.default,
        lookup_switch
            .pairs
            .iter()
            .map(|(&key, &offset)| (key, offset)),
    )
}

/// Cranelift's Switch has no block arguments. Forward through one block per
/// distinct destination to preserve all operands below the consumed key. Those
/// SSA values dominate the forwarding blocks; no new reference kinds are needed.
fn emit_switch(
    function_builder: &mut FunctionBuilder,
    blocks: &AHashMap<usize, Block>,
    stack: &mut OperandStack,
    program_counter: usize,
    default: i32,
    cases: impl IntoIterator<Item = (i32, i32)>,
) -> Result<()> {
    let index = stack.pop_int(function_builder)?;
    let arguments = stack.as_block_arguments();
    let mut forwarding = std::collections::BTreeMap::new();
    let mut target = |offset: i32| -> Result<Block> {
        let address = usize::try_from(i64::try_from(program_counter)? + i64::from(offset))?;
        let block = *blocks.get(&address).ok_or(InvalidBlockAddress(address))?;
        if arguments.is_empty() {
            Ok(block)
        } else {
            Ok(*forwarding
                .entry(address)
                .or_insert_with(|| function_builder.create_block()))
        }
    };
    let mut switch = Switch::new();
    for (key, offset) in cases {
        // Switch matches unsigned register bits; preserve negative Java int keys
        // as their exact 32-bit representation rather than converting their sign.
        let key = u128::from(u32::from_ne_bytes(key.to_ne_bytes()));
        switch.set_entry(key, target(offset)?);
    }
    let default_block = target(default)?;
    switch.emit(function_builder, index, default_block);
    for (address, block) in forwarding {
        let target = *blocks.get(&address).ok_or(InvalidBlockAddress(address))?;
        function_builder.switch_to_block(block);
        function_builder.ins().jump(target, &arguments);
    }
    Ok(())
}

/// # References
///
/// - [JVMS §6.5.return](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.return)
pub(crate) fn r#return(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
    _return_pointer: Value,
) {
    // The caller initializes the result to JitValue::new() for a void return.
    let stack_values = stack.as_slice();
    function_builder.ins().return_(stack_values);
}
