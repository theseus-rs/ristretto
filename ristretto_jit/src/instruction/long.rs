use crate::instruction::ThrowContext;
use crate::instruction::object::{array_load, array_store};
use crate::local_variables::LocalVariables;
use crate::operand_stack::OperandStack;
use crate::runtime_helpers::RuntimeHelpers;
use crate::{Result, jit_value};
use cranelift::frontend::FunctionBuilder;
use cranelift::prelude::{InstBuilder, IntCC, MemFlags, Value, types};

/// # References
/// - [JVMS §6.5.lconst_l](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.lconst_l)
pub(crate) fn lconst_0(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
) -> Result<()> {
    let constant = function_builder.ins().iconst(types::I64, 0);
    stack.push_long(function_builder, constant)?;
    Ok(())
}

/// # References
/// - [JVMS §6.5.lconst_l](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.lconst_l)
pub(crate) fn lconst_1(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
) -> Result<()> {
    let constant = function_builder.ins().iconst(types::I64, 1);
    stack.push_long(function_builder, constant)?;
    Ok(())
}

/// # References
/// - [JVMS §6.5.lload](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.lload)
pub(crate) fn lload(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
    index: u8,
) -> Result<()> {
    let index = usize::from(index);
    let value = locals.get_long(function_builder, index)?;
    stack.push_long(function_builder, value)?;
    Ok(())
}

/// # References
/// - [JVMS §6.5.lload](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.lload)
/// - [JVMS §6.5.wide](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.wide)
pub(crate) fn lload_w(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
    index: u16,
) -> Result<()> {
    let index = usize::from(index);
    let value = locals.get_long(function_builder, index)?;
    stack.push_long(function_builder, value)?;
    Ok(())
}

/// # References
/// - [JVMS §6.5.lload_n](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.lload_n)
pub(crate) fn lload_0(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<()> {
    let value = locals.get_long(function_builder, 0)?;
    stack.push_long(function_builder, value)?;
    Ok(())
}

/// # References
/// - [JVMS §6.5.lload_n](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.lload_n)
pub(crate) fn lload_1(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<()> {
    let value = locals.get_long(function_builder, 1)?;
    stack.push_long(function_builder, value)?;
    Ok(())
}

/// # References
/// - [JVMS §6.5.lload_n](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.lload_n)
pub(crate) fn lload_2(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<()> {
    let value = locals.get_long(function_builder, 2)?;
    stack.push_long(function_builder, value)?;
    Ok(())
}

/// # References
/// - [JVMS §6.5.lload_n](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.lload_n)
pub(crate) fn lload_3(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<()> {
    let value = locals.get_long(function_builder, 3)?;
    stack.push_long(function_builder, value)?;
    Ok(())
}

/// # References
/// - [JVMS §6.5.lstore](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.lstore)
pub(crate) fn lstore(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
    index: u8,
) -> Result<()> {
    let value = stack.pop_long(function_builder)?;
    let index = usize::from(index);
    locals.set_long(function_builder, index, value)?;
    Ok(())
}

/// # References
/// - [JVMS §6.5.lstore](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.lstore)
/// - [JVMS §6.5.wide](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.wide)
pub(crate) fn lstore_w(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
    index: u16,
) -> Result<()> {
    let value = stack.pop_long(function_builder)?;
    let index = usize::from(index);
    locals.set_long(function_builder, index, value)?;
    Ok(())
}

/// # References
/// - [JVMS §6.5.lstore_n](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.lstore_n)
pub(crate) fn lstore_0(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<()> {
    let value = stack.pop_long(function_builder)?;
    locals.set_long(function_builder, 0, value)?;
    Ok(())
}

/// # References
/// - [JVMS §6.5.lstore_n](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.lstore_n)
pub(crate) fn lstore_1(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<()> {
    let value = stack.pop_long(function_builder)?;
    locals.set_long(function_builder, 1, value)?;
    Ok(())
}

/// # References
/// - [JVMS §6.5.lstore_n](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.lstore_n)
pub(crate) fn lstore_2(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<()> {
    let value = stack.pop_long(function_builder)?;
    locals.set_long(function_builder, 2, value)?;
    Ok(())
}

/// # References
/// - [JVMS §6.5.lstore_n](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.lstore_n)
pub(crate) fn lstore_3(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<()> {
    let value = stack.pop_long(function_builder)?;
    locals.set_long(function_builder, 3, value)?;
    Ok(())
}

/// # References
/// - [JVMS §6.5.laload](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.laload)
pub(crate) fn laload(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
    helpers: &RuntimeHelpers,
    context_pointer: Value,
    throw_context: &ThrowContext<'_>,
) -> Result<()> {
    let value = array_load(
        function_builder,
        stack,
        helpers,
        context_pointer,
        throw_context,
        helpers.laload,
    )?;
    stack.push_long(function_builder, value)?;
    Ok(())
}

/// # References
/// - [JVMS §6.5.lastore](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.lastore)
pub(crate) fn lastore(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
    helpers: &RuntimeHelpers,
    context_pointer: Value,
    throw_context: &ThrowContext<'_>,
) -> Result<()> {
    let value = stack.pop_long(function_builder)?;
    array_store(
        function_builder,
        stack,
        helpers,
        context_pointer,
        throw_context,
        helpers.lastore,
        value,
    )
}

/// # References
/// - [JVMS §6.5.ladd](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.ladd)
pub(crate) fn ladd(function_builder: &mut FunctionBuilder, stack: &mut OperandStack) -> Result<()> {
    let value2 = stack.pop_long(function_builder)?;
    let value1 = stack.pop_long(function_builder)?;
    let value = function_builder.ins().iadd(value1, value2);
    stack.push_long(function_builder, value)?;
    Ok(())
}

/// # References
/// - [JVMS §6.5.lsub](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.lsub)
pub(crate) fn lsub(function_builder: &mut FunctionBuilder, stack: &mut OperandStack) -> Result<()> {
    let value2 = stack.pop_long(function_builder)?;
    let value1 = stack.pop_long(function_builder)?;
    let value = function_builder.ins().isub(value1, value2);
    stack.push_long(function_builder, value)?;
    Ok(())
}

/// # References
/// - [JVMS §6.5.lmul](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.lmul)
pub(crate) fn lmul(function_builder: &mut FunctionBuilder, stack: &mut OperandStack) -> Result<()> {
    let value2 = stack.pop_long(function_builder)?;
    let value1 = stack.pop_long(function_builder)?;
    let value = function_builder.ins().imul(value1, value2);
    stack.push_long(function_builder, value)?;
    Ok(())
}

/// # References
/// - [JVMS §6.5.ldiv](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.ldiv)
pub(crate) fn ldiv(function_builder: &mut FunctionBuilder, stack: &mut OperandStack) -> Result<()> {
    let value2 = stack.pop_long(function_builder)?;
    let value1 = stack.pop_long(function_builder)?;
    let value = function_builder.ins().sdiv(value1, value2);
    // TODO: Handle division by zero
    // stack.push_long(
    //     value1
    //         .checked_div(value2)
    //         .ok_or(ArithmeticException("/ by zero".to_string()))?,
    // )?;
    stack.push_long(function_builder, value)?;
    Ok(())
}

/// # References
/// - [JVMS §6.5.lrem](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.lrem)
pub(crate) fn lrem(function_builder: &mut FunctionBuilder, stack: &mut OperandStack) -> Result<()> {
    let value2 = stack.pop_long(function_builder)?;
    let value1 = stack.pop_long(function_builder)?;
    let value = function_builder.ins().srem(value1, value2);
    // TODO: Handle division by zero
    // stack.push_long(
    //     value1
    //         .checked_rem(value2)
    //         .ok_or(ArithmeticException("/ by zero".to_string()))?,
    // )?;
    stack.push_long(function_builder, value)?;
    Ok(())
}

/// # References
/// - [JVMS §6.5.lneg](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.lneg)
pub(crate) fn lneg(function_builder: &mut FunctionBuilder, stack: &mut OperandStack) -> Result<()> {
    let value = stack.pop_long(function_builder)?;
    let value = function_builder.ins().ineg(value);
    stack.push_long(function_builder, value)?;
    Ok(())
}

/// # References
/// - [JVMS §6.5.lshl](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.lshl)
pub(crate) fn lshl(function_builder: &mut FunctionBuilder, stack: &mut OperandStack) -> Result<()> {
    let value2 = stack.pop_int(function_builder)?;
    let value1 = stack.pop_long(function_builder)?;
    let value2 = function_builder.ins().sextend(types::I64, value2);
    let mask = function_builder.ins().iconst(types::I64, 0x3f);
    let value2 = function_builder.ins().band(value2, mask);
    let value = function_builder.ins().ishl(value1, value2);
    stack.push_long(function_builder, value)?;
    Ok(())
}

/// # References
/// - [JVMS §6.5.lshr](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.lshr)
pub(crate) fn lshr(function_builder: &mut FunctionBuilder, stack: &mut OperandStack) -> Result<()> {
    let value2 = stack.pop_int(function_builder)?;
    let value1 = stack.pop_long(function_builder)?;
    let value2 = function_builder.ins().sextend(types::I64, value2);
    let mask = function_builder.ins().iconst(types::I64, 0x3f);
    let value2 = function_builder.ins().band(value2, mask);
    let value = function_builder.ins().sshr(value1, value2);
    stack.push_long(function_builder, value)?;
    Ok(())
}

/// # References
/// - [JVMS §6.5.lushr](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.lushr)
pub(crate) fn lushr(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
) -> Result<()> {
    let value2 = stack.pop_int(function_builder)?;
    let value1 = stack.pop_long(function_builder)?;
    let value2 = function_builder.ins().sextend(types::I64, value2);
    let mask = function_builder.ins().iconst(types::I64, 0x3f);
    let value2 = function_builder.ins().band(value2, mask);
    let value = function_builder.ins().ushr(value1, value2);
    stack.push_long(function_builder, value)?;
    Ok(())
}

/// # References
/// - [JVMS §6.5.land](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.land)
pub(crate) fn land(function_builder: &mut FunctionBuilder, stack: &mut OperandStack) -> Result<()> {
    let value2 = stack.pop_long(function_builder)?;
    let value1 = stack.pop_long(function_builder)?;
    let value = function_builder.ins().band(value1, value2);
    stack.push_long(function_builder, value)?;
    Ok(())
}

/// # References
/// - [JVMS §6.5.lor](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.lor)
pub(crate) fn lor(function_builder: &mut FunctionBuilder, stack: &mut OperandStack) -> Result<()> {
    let value2 = stack.pop_long(function_builder)?;
    let value1 = stack.pop_long(function_builder)?;
    let value = function_builder.ins().bor(value1, value2);
    stack.push_long(function_builder, value)?;
    Ok(())
}

/// # References
/// - [JVMS §6.5.lxor](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.lxor)
pub(crate) fn lxor(function_builder: &mut FunctionBuilder, stack: &mut OperandStack) -> Result<()> {
    let value2 = stack.pop_long(function_builder)?;
    let value1 = stack.pop_long(function_builder)?;
    let value = function_builder.ins().bxor(value1, value2);
    stack.push_long(function_builder, value)?;
    Ok(())
}

/// # References
/// - [JVMS §6.5.lcmp](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.lcmp)
pub(crate) fn lcmp(function_builder: &mut FunctionBuilder, stack: &mut OperandStack) -> Result<()> {
    let value2 = stack.pop_long(function_builder)?;
    let value1 = stack.pop_long(function_builder)?;
    let is_greater = function_builder
        .ins()
        .icmp(IntCC::SignedGreaterThan, value1, value2);
    let is_less = function_builder
        .ins()
        .icmp(IntCC::SignedLessThan, value1, value2);
    let one = function_builder.ins().iconst(types::I32, 1);
    let neg_one = function_builder.ins().iconst(types::I32, -1);
    let zero = function_builder.ins().iconst(types::I32, 0);
    let result = function_builder.ins().select(is_less, neg_one, zero);
    let result = function_builder.ins().select(is_greater, one, result);
    stack.push_int(function_builder, result)?;
    Ok(())
}

/// # References
/// - [JVMS §6.5.lreturn](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.lreturn)
pub(crate) fn lreturn(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
    return_pointer: Value,
) -> Result<()> {
    let value = stack.pop_long(function_builder)?;
    let discriminate = i64::from(jit_value::I64);
    let discriminate = function_builder.ins().iconst(types::I8, discriminate);
    function_builder
        .ins()
        .store(MemFlags::new(), discriminate, return_pointer, 0);
    function_builder
        .ins()
        .store(MemFlags::new(), value, return_pointer, 8);
    function_builder.ins().return_(&[]);
    Ok(())
}
