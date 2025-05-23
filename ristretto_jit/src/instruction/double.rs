use crate::control_flow_graph::append_block_params;
use crate::local_variables::LocalVariables;
use crate::operand_stack::OperandStack;
use crate::{Result, jit_value};
use cranelift::frontend::FunctionBuilder;
use cranelift::prelude::{FloatCC, InstBuilder, MemFlags, Value, types};

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dconst_d>
pub(crate) fn dconst_0(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
) -> Result<()> {
    let constant = function_builder.ins().f64const(0.0);
    stack.push_double(function_builder, constant)?;
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dconst_d>
pub(crate) fn dconst_1(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
) -> Result<()> {
    let constant = function_builder.ins().f64const(1.0);
    stack.push_double(function_builder, constant)?;
    Ok(())
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
    stack.push_double(function_builder, value)?;
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
    stack.push_double(function_builder, value)?;
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dload_n>
pub(crate) fn dload_0(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<()> {
    let value = locals.get_double(function_builder, 0)?;
    stack.push_double(function_builder, value)?;
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dload_n>
pub(crate) fn dload_1(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<()> {
    let value = locals.get_double(function_builder, 1)?;
    stack.push_double(function_builder, value)?;
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dload_n>
pub(crate) fn dload_2(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<()> {
    let value = locals.get_double(function_builder, 2)?;
    stack.push_double(function_builder, value)?;
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dload_n>
pub(crate) fn dload_3(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<()> {
    let value = locals.get_double(function_builder, 3)?;
    stack.push_double(function_builder, value)?;
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dstore>
pub(crate) fn dstore(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
    index: u8,
) -> Result<()> {
    let value = stack.pop_double(function_builder)?;
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
    let value = stack.pop_double(function_builder)?;
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
    let value = stack.pop_double(function_builder)?;
    locals.set_double(function_builder, 0, value)?;
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dstore_n>
pub(crate) fn dstore_1(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<()> {
    let value = stack.pop_double(function_builder)?;
    locals.set_double(function_builder, 1, value)?;
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dstore_n>
pub(crate) fn dstore_2(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<()> {
    let value = stack.pop_double(function_builder)?;
    locals.set_double(function_builder, 2, value)?;
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dstore_n>
pub(crate) fn dstore_3(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<()> {
    let value = stack.pop_double(function_builder)?;
    locals.set_double(function_builder, 3, value)?;
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dadd>
pub(crate) fn dadd(function_builder: &mut FunctionBuilder, stack: &mut OperandStack) -> Result<()> {
    let value2 = stack.pop_double(function_builder)?;
    let value1 = stack.pop_double(function_builder)?;
    let value = function_builder.ins().fadd(value1, value2);
    stack.push_double(function_builder, value)?;
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dsub>
pub(crate) fn dsub(function_builder: &mut FunctionBuilder, stack: &mut OperandStack) -> Result<()> {
    let value2 = stack.pop_double(function_builder)?;
    let value1 = stack.pop_double(function_builder)?;
    let value = function_builder.ins().fsub(value1, value2);
    stack.push_double(function_builder, value)?;
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dmul>
pub(crate) fn dmul(function_builder: &mut FunctionBuilder, stack: &mut OperandStack) -> Result<()> {
    let value2 = stack.pop_double(function_builder)?;
    let value1 = stack.pop_double(function_builder)?;
    let value = function_builder.ins().fmul(value1, value2);
    stack.push_double(function_builder, value)?;
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.ddiv>
pub(crate) fn ddiv(function_builder: &mut FunctionBuilder, stack: &mut OperandStack) -> Result<()> {
    let value2 = stack.pop_double(function_builder)?;
    let value1 = stack.pop_double(function_builder)?;
    let value = function_builder.ins().fdiv(value1, value2);
    // TODO: Handle division by zero
    // if value2 == 0.0 {
    //     return Err(ArithmeticException("/ by zero".to_string()).into());
    // };
    stack.push_double(function_builder, value)?;
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.drem>
pub(crate) fn drem(function_builder: &mut FunctionBuilder, stack: &mut OperandStack) -> Result<()> {
    let value2 = stack.pop_double(function_builder)?;
    let value1 = stack.pop_double(function_builder)?;
    // TODO: optimize this if/when cranelift supports frem
    let div = function_builder.ins().fdiv(value1, value2);
    let trunc = function_builder.ins().trunc(div);
    let mul = function_builder.ins().fmul(value2, trunc);
    let value = function_builder.ins().fsub(value1, mul);
    stack.push_double(function_builder, value)?;
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dneg>
pub(crate) fn dneg(function_builder: &mut FunctionBuilder, stack: &mut OperandStack) -> Result<()> {
    let value = stack.pop_double(function_builder)?;
    let value = function_builder.ins().fneg(value);
    stack.push_double(function_builder, value)?;
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dcmp_op>
pub(crate) fn dcmpl(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
) -> Result<()> {
    let value2 = stack.pop_double(function_builder)?;
    let value1 = stack.pop_double(function_builder)?;
    let stack_types = stack.to_type_vec(function_builder);
    let params = stack.as_block_arguments();

    let nan_block = function_builder.create_block();
    append_block_params(function_builder, nan_block, &stack_types);
    let equal_block = function_builder.create_block();
    append_block_params(function_builder, equal_block, &stack_types);
    let else_block = function_builder.create_block();
    append_block_params(function_builder, equal_block, &stack_types);
    let greater_than_block = function_builder.create_block();
    append_block_params(function_builder, equal_block, &stack_types);
    let less_than_block = function_builder.create_block();
    append_block_params(function_builder, equal_block, &stack_types);
    let merge_block = function_builder.create_block();
    append_block_params(function_builder, equal_block, &stack_types);
    function_builder.append_block_param(merge_block, types::I32);

    // NOTE: this could be optimized to use FloatCC::UnorderedOrLessThan once implemented by cranelift

    // Check for NaN: if value1 or value2 is NaN, go to nan_block
    let is_nan = function_builder
        .ins()
        .fcmp(FloatCC::Unordered, value1, value2);
    function_builder
        .ins()
        .brif(is_nan, nan_block, &params, equal_block, &params);

    // nan_block: push -1
    function_builder.switch_to_block(nan_block);
    let mut nan_params = params.clone();
    let nan_return = function_builder.ins().iconst(types::I32, -1);
    nan_params.push(nan_return.into());
    function_builder.ins().jump(merge_block, &nan_params);

    // equal_block: check for equality
    function_builder.switch_to_block(equal_block);
    let is_equal = function_builder.ins().fcmp(FloatCC::Equal, value1, value2);
    let mut equals_params = params.clone();
    let equal_return = function_builder.ins().iconst(types::I32, 0);
    equals_params.push(equal_return.into());
    function_builder
        .ins()
        .brif(is_equal, merge_block, &equals_params, else_block, &params);

    // else_block: check for greater than
    function_builder.switch_to_block(else_block);
    let condition_value = function_builder
        .ins()
        .fcmp(FloatCC::GreaterThan, value1, value2);
    function_builder.ins().brif(
        condition_value,
        greater_than_block,
        &params,
        less_than_block,
        &params,
    );

    // greater_than_block: push 1
    function_builder.switch_to_block(greater_than_block);
    let mut greater_than_params = params.clone();
    let greater_than_return = function_builder.ins().iconst(types::I32, 1);
    greater_than_params.push(greater_than_return.into());
    function_builder
        .ins()
        .jump(merge_block, &greater_than_params);

    // less_than_block: push -1
    function_builder.switch_to_block(less_than_block);
    let mut less_than_params = params.clone();
    let less_than_return = function_builder.ins().iconst(types::I32, -1);
    less_than_params.push(less_than_return.into());
    function_builder.ins().jump(merge_block, &less_than_params);

    // merge_block: push result
    function_builder.switch_to_block(merge_block);
    let value = function_builder.block_params(merge_block)[0];
    stack.push_int(function_builder, value)?;
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dcmp_op>
pub(crate) fn dcmpg(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
) -> Result<()> {
    let value2 = stack.pop_double(function_builder)?;
    let value1 = stack.pop_double(function_builder)?;
    let stack_types = stack.to_type_vec(function_builder);
    let params = stack.as_block_arguments();

    let nan_block = function_builder.create_block();
    append_block_params(function_builder, nan_block, &stack_types);
    let equal_block = function_builder.create_block();
    append_block_params(function_builder, equal_block, &stack_types);
    let else_block = function_builder.create_block();
    append_block_params(function_builder, equal_block, &stack_types);
    let greater_than_block = function_builder.create_block();
    append_block_params(function_builder, equal_block, &stack_types);
    let less_than_block = function_builder.create_block();
    append_block_params(function_builder, equal_block, &stack_types);
    let merge_block = function_builder.create_block();
    append_block_params(function_builder, equal_block, &stack_types);
    function_builder.append_block_param(merge_block, types::I32);

    // NOTE: this could be optimized to use FloatCC::UnorderedOrGreaterThan once implemented by cranelift

    // Check for NaN: if value1 or value2 is NaN, go to nan_block
    let is_nan = function_builder
        .ins()
        .fcmp(FloatCC::Unordered, value1, value2);
    function_builder
        .ins()
        .brif(is_nan, nan_block, &params, equal_block, &params);

    // nan_block: push 1
    function_builder.switch_to_block(nan_block);
    let mut nan_params = params.clone();
    let nan_return = function_builder.ins().iconst(types::I32, 1);
    nan_params.push(nan_return.into());
    function_builder.ins().jump(merge_block, &nan_params);

    // equal_block: check for equality
    function_builder.switch_to_block(equal_block);
    let is_equal = function_builder.ins().fcmp(FloatCC::Equal, value1, value2);
    let mut equals_params = params.clone();
    let equal_return = function_builder.ins().iconst(types::I32, 0);
    equals_params.push(equal_return.into());
    function_builder
        .ins()
        .brif(is_equal, merge_block, &equals_params, else_block, &params);

    // else_block: check for greater than
    function_builder.switch_to_block(else_block);
    let condition_value = function_builder
        .ins()
        .fcmp(FloatCC::GreaterThan, value1, value2);
    function_builder.ins().brif(
        condition_value,
        greater_than_block,
        &params,
        less_than_block,
        &params,
    );

    // greater_than_block: push 1
    function_builder.switch_to_block(greater_than_block);
    let mut greater_than_params = params.clone();
    let greater_than_return = function_builder.ins().iconst(types::I32, 1);
    greater_than_params.push(greater_than_return.into());
    function_builder
        .ins()
        .jump(merge_block, &greater_than_params);

    // less_than_block: push -1
    function_builder.switch_to_block(less_than_block);
    let mut less_than_params = params.clone();
    let less_than_return = function_builder.ins().iconst(types::I32, -1);
    less_than_params.push(less_than_return.into());
    function_builder.ins().jump(merge_block, &less_than_params);

    // merge_block: push result
    function_builder.switch_to_block(merge_block);
    let value = function_builder.block_params(merge_block)[0];
    stack.push_int(function_builder, value)?;
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dreturn>
pub(crate) fn dreturn(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
    return_pointer: Value,
) -> Result<()> {
    let value = stack.pop_double(function_builder)?;
    let discriminate = i64::from(jit_value::F64);
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
