use crate::local_variables::LocalVariables;
use crate::operand_stack::OperandStack;
use crate::{Result, jit_value};
use cranelift::frontend::FunctionBuilder;
use cranelift::prelude::{InstBuilder, IntCC, MemFlags, Value, types};

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dconst_d>
pub(crate) fn dconst_0(function_builder: &mut FunctionBuilder, stack: &mut OperandStack) {
    let constant = function_builder.ins().f64const(0.0);
    stack.push_double(function_builder, constant);
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dconst_d>
pub(crate) fn dconst_1(function_builder: &mut FunctionBuilder, stack: &mut OperandStack) {
    let constant = function_builder.ins().f64const(1.0);
    stack.push_double(function_builder, constant);
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dload>
pub(crate) fn dload(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
    index: u8,
) -> Result<()> {
    let index = usize::from(index);
    let value = locals.get_double(function_builder, index)?;
    stack.push_double(function_builder, value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dload>
/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.wide>
pub(crate) fn dload_w(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
    index: u16,
) -> Result<()> {
    let index = usize::from(index);
    let value = locals.get_double(function_builder, index)?;
    stack.push_double(function_builder, value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dload_n>
pub(crate) fn dload_0(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<()> {
    let value = locals.get_double(function_builder, 0)?;
    stack.push_double(function_builder, value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dload_n>
pub(crate) fn dload_1(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<()> {
    let value = locals.get_double(function_builder, 1)?;
    stack.push_double(function_builder, value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dload_n>
pub(crate) fn dload_2(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<()> {
    let value = locals.get_double(function_builder, 2)?;
    stack.push_double(function_builder, value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dload_n>
pub(crate) fn dload_3(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<()> {
    let value = locals.get_double(function_builder, 3)?;
    stack.push_double(function_builder, value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dstore>
pub(crate) fn dstore(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
    index: u8,
) -> Result<()> {
    let value = stack.pop_double(function_builder);
    let index = usize::from(index);
    locals.set_double(function_builder, index, value)?;
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dstore>
/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.wide>
pub(crate) fn dstore_w(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
    index: u16,
) -> Result<()> {
    let value = stack.pop_double(function_builder);
    let index = usize::from(index);
    locals.set_double(function_builder, index, value)?;
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dstore_n>
pub(crate) fn dstore_0(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<()> {
    let value = stack.pop_double(function_builder);
    locals.set_double(function_builder, 0, value)?;
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dstore_n>
pub(crate) fn dstore_1(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<()> {
    let value = stack.pop_double(function_builder);
    locals.set_double(function_builder, 1, value)?;
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dstore_n>
pub(crate) fn dstore_2(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<()> {
    let value = stack.pop_double(function_builder);
    locals.set_double(function_builder, 2, value)?;
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dstore_n>
pub(crate) fn dstore_3(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<()> {
    let value = stack.pop_double(function_builder);
    locals.set_double(function_builder, 3, value)?;
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dadd>
pub(crate) fn dadd(function_builder: &mut FunctionBuilder, stack: &mut OperandStack) {
    let value2 = stack.pop_double(function_builder);
    let value1 = stack.pop_double(function_builder);
    let value = function_builder.ins().fadd(value1, value2);
    stack.push_double(function_builder, value);
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dsub>
pub(crate) fn dsub(function_builder: &mut FunctionBuilder, stack: &mut OperandStack) {
    let value2 = stack.pop_double(function_builder);
    let value1 = stack.pop_double(function_builder);
    let value = function_builder.ins().fsub(value1, value2);
    stack.push_double(function_builder, value);
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dmul>
pub(crate) fn dmul(function_builder: &mut FunctionBuilder, stack: &mut OperandStack) {
    let value2 = stack.pop_double(function_builder);
    let value1 = stack.pop_double(function_builder);
    let value = function_builder.ins().fmul(value1, value2);
    stack.push_double(function_builder, value);
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.ddiv>
pub(crate) fn ddiv(function_builder: &mut FunctionBuilder, stack: &mut OperandStack) {
    let value2 = stack.pop_double(function_builder);
    let value1 = stack.pop_double(function_builder);
    let value = function_builder.ins().fdiv(value1, value2);
    // TODO: Handle division by zero
    // if value2 == 0.0 {
    //     return Err(ArithmeticException("/ by zero".to_string()).into());
    // };
    stack.push_double(function_builder, value);
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.drem>
pub(crate) fn drem(function_builder: &mut FunctionBuilder, stack: &mut OperandStack) {
    let value2 = stack.pop_double(function_builder);
    let value1 = stack.pop_double(function_builder);
    // TODO: optimize this if/when cranelift supports frem
    let div = function_builder.ins().fdiv(value1, value2);
    let trunc = function_builder.ins().trunc(div);
    let mul = function_builder.ins().fmul(value2, trunc);
    let value = function_builder.ins().fsub(value1, mul);
    stack.push_double(function_builder, value);
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dneg>
pub(crate) fn dneg(function_builder: &mut FunctionBuilder, stack: &mut OperandStack) {
    let value = stack.pop_double(function_builder);
    let value = function_builder.ins().fneg(value);
    stack.push_double(function_builder, value);
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dcmpl>
pub(crate) fn dcmpl(function_builder: &mut FunctionBuilder, stack: &mut OperandStack) {
    let value2 = stack.pop_double(function_builder);
    let value1 = stack.pop_double(function_builder);

    let equal_block = function_builder.create_block();
    let else_block = function_builder.create_block();
    let greater_than_block = function_builder.create_block();
    let less_than_block = function_builder.create_block();
    let merge_block = function_builder.create_block();

    function_builder.append_block_param(merge_block, types::I32);

    // TODO: Handle f64::is_nan(value1) || f64::is_nan(value2)

    let condition_value = function_builder.ins().icmp(IntCC::Equal, value1, value2);
    function_builder
        .ins()
        .brif(condition_value, equal_block, &[], else_block, &[]);

    function_builder.switch_to_block(equal_block);
    function_builder.seal_block(equal_block);
    let equal_return = function_builder.ins().iconst(types::I32, 0);
    function_builder.ins().jump(merge_block, &[equal_return]);

    function_builder.switch_to_block(else_block);
    function_builder.seal_block(else_block);
    let condition_value = function_builder
        .ins()
        .icmp(IntCC::SignedGreaterThan, value1, value2);
    function_builder.ins().brif(
        condition_value,
        greater_than_block,
        &[],
        less_than_block,
        &[],
    );

    function_builder.switch_to_block(greater_than_block);
    function_builder.seal_block(greater_than_block);
    let greater_than_return = function_builder.ins().iconst(types::I32, 1);
    function_builder
        .ins()
        .jump(merge_block, &[greater_than_return]);

    function_builder.switch_to_block(less_than_block);
    function_builder.seal_block(less_than_block);
    let less_than_return = function_builder.ins().iconst(types::I32, -1);
    function_builder
        .ins()
        .jump(merge_block, &[less_than_return]);

    function_builder.switch_to_block(merge_block);
    function_builder.seal_block(merge_block);
    let value = function_builder.block_params(merge_block)[0];
    stack.push_int(function_builder, value);
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dcmpg>
pub(crate) fn dcmpg(function_builder: &mut FunctionBuilder, stack: &mut OperandStack) {
    let value2 = stack.pop_double(function_builder);
    let value1 = stack.pop_double(function_builder);

    let equal_block = function_builder.create_block();
    let else_block = function_builder.create_block();
    let greater_than_block = function_builder.create_block();
    let less_than_block = function_builder.create_block();
    let merge_block = function_builder.create_block();

    function_builder.append_block_param(merge_block, types::I32);

    let condition_value = function_builder.ins().icmp(IntCC::Equal, value1, value2);
    function_builder
        .ins()
        .brif(condition_value, equal_block, &[], else_block, &[]);

    function_builder.switch_to_block(equal_block);
    function_builder.seal_block(equal_block);
    let equal_return = function_builder.ins().iconst(types::I32, 0);
    function_builder.ins().jump(merge_block, &[equal_return]);

    function_builder.switch_to_block(else_block);
    function_builder.seal_block(else_block);
    let condition_value = function_builder
        .ins()
        .icmp(IntCC::SignedGreaterThan, value1, value2);
    function_builder.ins().brif(
        condition_value,
        greater_than_block,
        &[],
        less_than_block,
        &[],
    );

    function_builder.switch_to_block(greater_than_block);
    function_builder.seal_block(greater_than_block);
    let greater_than_return = function_builder.ins().iconst(types::I32, 1);
    function_builder
        .ins()
        .jump(merge_block, &[greater_than_return]);

    function_builder.switch_to_block(less_than_block);
    function_builder.seal_block(less_than_block);
    let less_than_return = function_builder.ins().iconst(types::I32, -1);
    function_builder
        .ins()
        .jump(merge_block, &[less_than_return]);

    function_builder.switch_to_block(merge_block);
    function_builder.seal_block(merge_block);
    let value = function_builder.block_params(merge_block)[0];
    stack.push_int(function_builder, value);
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dreturn>
pub(crate) fn dreturn(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
    return_pointer: Value,
) {
    let value = stack.pop_double(function_builder);
    let value = function_builder.ins().fcvt_to_sint(types::I64, value);
    let discriminate = i64::from(jit_value::F64);
    let discriminate = function_builder.ins().iconst(types::I8, discriminate);
    function_builder
        .ins()
        .store(MemFlags::new(), discriminate, return_pointer, 0);
    function_builder
        .ins()
        .store(MemFlags::new(), value, return_pointer, 8);
    function_builder.ins().return_(&[]);
}
