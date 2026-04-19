use crate::instruction::ThrowContext;
use crate::instruction::object::{array_load, array_store};
use crate::local_variables::LocalVariables;
use crate::operand_stack::OperandStack;
use crate::runtime_helpers::RuntimeHelpers;
use crate::{Result, jit_value};
use cranelift::frontend::FunctionBuilder;
use cranelift::prelude::{FloatCC, InstBuilder, MemFlags, Value, types};

/// # References
///
/// - [JVMS §6.5.fconst_f](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.fconst_f)
pub(crate) fn fconst_0(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
) -> Result<()> {
    let constant = function_builder.ins().f32const(0.0);
    stack.push_float(function_builder, constant)?;
    Ok(())
}

/// # References
///
/// - [JVMS §6.5.fconst_f](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.fconst_f)
pub(crate) fn fconst_1(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
) -> Result<()> {
    let constant = function_builder.ins().f32const(1.0);
    stack.push_float(function_builder, constant)?;
    Ok(())
}

/// # References
///
/// - [JVMS §6.5.fconst_f](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.fconst_f)
pub(crate) fn fconst_2(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
) -> Result<()> {
    let constant = function_builder.ins().f32const(2.0);
    stack.push_float(function_builder, constant)?;
    Ok(())
}

/// # References
///
/// - [JVMS §6.5.fload](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.fload)
pub(crate) fn fload(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
    index: u8,
) -> Result<()> {
    let index = usize::from(index);
    let value = locals.get_float(function_builder, index)?;
    stack.push_float(function_builder, value)?;
    Ok(())
}

/// # References
///
/// - [JVMS §6.5.fload](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.fload)
/// - [JVMS §6.5.wide](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.wide)
pub(crate) fn fload_w(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
    index: u16,
) -> Result<()> {
    let index = usize::from(index);
    let value = locals.get_float(function_builder, index)?;
    stack.push_float(function_builder, value)?;
    Ok(())
}

/// # References
///
/// - [JVMS §6.5.fload_n](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.fload_n)
pub(crate) fn fload_0(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<()> {
    let value = locals.get_float(function_builder, 0)?;
    stack.push_float(function_builder, value)?;
    Ok(())
}

/// # References
///
/// - [JVMS §6.5.fload_n](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.fload_n)
pub(crate) fn fload_1(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<()> {
    let value = locals.get_float(function_builder, 1)?;
    stack.push_float(function_builder, value)?;
    Ok(())
}

/// # References
///
/// - [JVMS §6.5.fload_n](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.fload_n)
pub(crate) fn fload_2(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<()> {
    let value = locals.get_float(function_builder, 2)?;
    stack.push_float(function_builder, value)?;
    Ok(())
}

/// # References
///
/// - [JVMS §6.5.fload_n](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.fload_n)
pub(crate) fn fload_3(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<()> {
    let value = locals.get_float(function_builder, 3)?;
    stack.push_float(function_builder, value)?;
    Ok(())
}

/// # References
///
/// - [JVMS §6.5.fstore](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.fstore)
pub(crate) fn fstore(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
    index: u8,
) -> Result<()> {
    let value = stack.pop_float(function_builder)?;
    let index = usize::from(index);
    locals.set_float(function_builder, index, value)?;
    Ok(())
}

/// # References
///
/// - [JVMS §6.5.fstore](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.fstore)
/// - [JVMS §6.5.wide](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.wide)
pub(crate) fn fstore_w(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
    index: u16,
) -> Result<()> {
    let value = stack.pop_float(function_builder)?;
    let index = usize::from(index);
    locals.set_float(function_builder, index, value)?;
    Ok(())
}

/// # References
///
/// - [JVMS §6.5.fstore_n](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.fstore_n)
pub(crate) fn fstore_0(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<()> {
    let value = stack.pop_float(function_builder)?;
    locals.set_float(function_builder, 0, value)?;
    Ok(())
}

/// # References
///
/// - [JVMS §6.5.fstore_n](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.fstore_n)
pub(crate) fn fstore_1(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<()> {
    let value = stack.pop_float(function_builder)?;
    locals.set_float(function_builder, 1, value)?;
    Ok(())
}

/// # References
///
/// - [JVMS §6.5.fstore_n](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.fstore_n)
pub(crate) fn fstore_2(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<()> {
    let value = stack.pop_float(function_builder)?;
    locals.set_float(function_builder, 2, value)?;
    Ok(())
}

/// # References
///
/// - [JVMS §6.5.fstore_n](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.fstore_n)
pub(crate) fn fstore_3(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<()> {
    let value = stack.pop_float(function_builder)?;
    locals.set_float(function_builder, 3, value)?;
    Ok(())
}

/// # References
///
/// - [JVMS §6.5.faload](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.faload)
pub(crate) fn faload(
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
        helpers.faload,
    )?;
    stack.push_float(function_builder, value)?;
    Ok(())
}

/// # References
///
/// - [JVMS §6.5.fastore](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.fastore)
pub(crate) fn fastore(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
    helpers: &RuntimeHelpers,
    context_pointer: Value,
    throw_context: &ThrowContext<'_>,
) -> Result<()> {
    let value = stack.pop_float(function_builder)?;
    array_store(
        function_builder,
        stack,
        helpers,
        context_pointer,
        throw_context,
        helpers.fastore,
        value,
    )
}

/// # References
///
/// - [JVMS §6.5.fadd](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.fadd)
pub(crate) fn fadd(function_builder: &mut FunctionBuilder, stack: &mut OperandStack) -> Result<()> {
    let value2 = stack.pop_float(function_builder)?;
    let value1 = stack.pop_float(function_builder)?;
    let value = function_builder.ins().fadd(value1, value2);
    stack.push_float(function_builder, value)?;
    Ok(())
}

/// # References
///
/// - [JVMS §6.5.fsub](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.fsub)
pub(crate) fn fsub(function_builder: &mut FunctionBuilder, stack: &mut OperandStack) -> Result<()> {
    let value2 = stack.pop_float(function_builder)?;
    let value1 = stack.pop_float(function_builder)?;
    let value = function_builder.ins().fsub(value1, value2);
    stack.push_float(function_builder, value)?;
    Ok(())
}

/// # References
///
/// - [JVMS §6.5.fmul](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.fmul)
pub(crate) fn fmul(function_builder: &mut FunctionBuilder, stack: &mut OperandStack) -> Result<()> {
    let value2 = stack.pop_float(function_builder)?;
    let value1 = stack.pop_float(function_builder)?;
    let value = function_builder.ins().fmul(value1, value2);
    stack.push_float(function_builder, value)?;
    Ok(())
}

/// # References
///
/// - [JVMS §6.5.fdiv](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.fdiv)
pub(crate) fn fdiv(function_builder: &mut FunctionBuilder, stack: &mut OperandStack) -> Result<()> {
    let value2 = stack.pop_float(function_builder)?;
    let value1 = stack.pop_float(function_builder)?;
    let value = function_builder.ins().fdiv(value1, value2);
    // TODO: Handle division by zero
    // if value2 == 0.0 {
    //     return Err(ArithmeticException("/ by zero".to_string()).into());
    // };
    stack.push_float(function_builder, value)?;
    Ok(())
}

/// # References
///
/// - [JVMS §6.5.frem](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.frem)
pub(crate) fn frem(function_builder: &mut FunctionBuilder, stack: &mut OperandStack) -> Result<()> {
    let value2 = stack.pop_float(function_builder)?;
    let value1 = stack.pop_float(function_builder)?;
    // TODO: optimize this if/when cranelift supports frem
    let div = function_builder.ins().fdiv(value1, value2);
    let trunc = function_builder.ins().trunc(div);
    let mul = function_builder.ins().fmul(value2, trunc);
    let value = function_builder.ins().fsub(value1, mul);
    stack.push_float(function_builder, value)?;
    Ok(())
}

/// # References
///
/// - [JVMS §6.5.fneg](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.fneg)
pub(crate) fn fneg(function_builder: &mut FunctionBuilder, stack: &mut OperandStack) -> Result<()> {
    let value = stack.pop_float(function_builder)?;
    let value = function_builder.ins().fneg(value);
    stack.push_float(function_builder, value)?;
    Ok(())
}

/// # References
///
/// - [JVMS §6.5.fcmp_op](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.fcmp_op)
pub(crate) fn fcmpl(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
) -> Result<()> {
    let value2 = stack.pop_float(function_builder)?;
    let value1 = stack.pop_float(function_builder)?;
    let is_nan = function_builder
        .ins()
        .fcmp(FloatCC::Unordered, value1, value2);
    let is_greater = function_builder
        .ins()
        .fcmp(FloatCC::GreaterThan, value1, value2);
    let is_less = function_builder
        .ins()
        .fcmp(FloatCC::LessThan, value1, value2);
    let one = function_builder.ins().iconst(types::I32, 1);
    let neg_one = function_builder.ins().iconst(types::I32, -1);
    let zero = function_builder.ins().iconst(types::I32, 0);
    // NaN -> -1 for fcmpl
    let result = function_builder.ins().select(is_nan, neg_one, zero);
    let result = function_builder.ins().select(is_less, neg_one, result);
    let result = function_builder.ins().select(is_greater, one, result);
    stack.push_int(function_builder, result)?;
    Ok(())
}

/// # References
///
/// - [JVMS §6.5.fcmp_op](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.fcmp_op)
pub(crate) fn fcmpg(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
) -> Result<()> {
    let value2 = stack.pop_float(function_builder)?;
    let value1 = stack.pop_float(function_builder)?;
    let is_nan = function_builder
        .ins()
        .fcmp(FloatCC::Unordered, value1, value2);
    let is_greater = function_builder
        .ins()
        .fcmp(FloatCC::GreaterThan, value1, value2);
    let is_less = function_builder
        .ins()
        .fcmp(FloatCC::LessThan, value1, value2);
    let one = function_builder.ins().iconst(types::I32, 1);
    let neg_one = function_builder.ins().iconst(types::I32, -1);
    let zero = function_builder.ins().iconst(types::I32, 0);
    // NaN -> 1 for fcmpg
    let result = function_builder.ins().select(is_nan, one, zero);
    let result = function_builder.ins().select(is_less, neg_one, result);
    let result = function_builder.ins().select(is_greater, one, result);
    stack.push_int(function_builder, result)?;
    Ok(())
}

/// # References
///
/// - [JVMS §6.5.freturn](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.freturn)
pub(crate) fn freturn(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
    return_pointer: Value,
) -> Result<()> {
    let value = stack.pop_float(function_builder)?;
    let discriminate = i64::from(jit_value::F32);
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
