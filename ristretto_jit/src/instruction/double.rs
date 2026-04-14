use crate::instruction::object::{array_load, array_store};
use crate::local_variables::LocalVariables;
use crate::operand_stack::OperandStack;
use crate::runtime_helpers::RuntimeHelpers;
use crate::{Result, jit_value};
use cranelift::frontend::FunctionBuilder;
use cranelift::prelude::{FloatCC, InstBuilder, MemFlags, Value, types};

/// # References
///
/// - [JVMS §6.5.dconst_d](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.dconst_d)
pub(crate) fn dconst_0(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
) -> Result<()> {
    let constant = function_builder.ins().f64const(0.0);
    stack.push_double(function_builder, constant)?;
    Ok(())
}

/// # References
///
/// - [JVMS §6.5.dconst_d](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.dconst_d)
pub(crate) fn dconst_1(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
) -> Result<()> {
    let constant = function_builder.ins().f64const(1.0);
    stack.push_double(function_builder, constant)?;
    Ok(())
}

/// # References
///
/// - [JVMS §6.5.dload](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.dload)
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

/// # References
///
/// - [JVMS §6.5.dload](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.dload)
/// - [JVMS §6.5.wide](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.wide)
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

/// # References
///
/// - [JVMS §6.5.dload_n](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.dload_n)
pub(crate) fn dload_0(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<()> {
    let value = locals.get_double(function_builder, 0)?;
    stack.push_double(function_builder, value)?;
    Ok(())
}

/// # References
///
/// - [JVMS §6.5.dload_n](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.dload_n)
pub(crate) fn dload_1(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<()> {
    let value = locals.get_double(function_builder, 1)?;
    stack.push_double(function_builder, value)?;
    Ok(())
}

/// # References
///
/// - [JVMS §6.5.dload_n](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.dload_n)
pub(crate) fn dload_2(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<()> {
    let value = locals.get_double(function_builder, 2)?;
    stack.push_double(function_builder, value)?;
    Ok(())
}

/// # References
///
/// - [JVMS §6.5.dload_n](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.dload_n)
pub(crate) fn dload_3(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<()> {
    let value = locals.get_double(function_builder, 3)?;
    stack.push_double(function_builder, value)?;
    Ok(())
}

/// # References
///
/// - [JVMS §6.5.dstore](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.dstore)
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

/// # References
///
/// - [JVMS §6.5.dstore](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.dstore)
/// - [JVMS §6.5.wide](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.wide)
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

/// # References
///
/// - [JVMS §6.5.dstore_n](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.dstore_n)
pub(crate) fn dstore_0(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<()> {
    let value = stack.pop_double(function_builder)?;
    locals.set_double(function_builder, 0, value)?;
    Ok(())
}

/// # References
///
/// - [JVMS §6.5.dstore_n](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.dstore_n)
pub(crate) fn dstore_1(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<()> {
    let value = stack.pop_double(function_builder)?;
    locals.set_double(function_builder, 1, value)?;
    Ok(())
}

/// # References
///
/// - [JVMS §6.5.dstore_n](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.dstore_n)
pub(crate) fn dstore_2(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<()> {
    let value = stack.pop_double(function_builder)?;
    locals.set_double(function_builder, 2, value)?;
    Ok(())
}

/// # References
///
/// - [JVMS §6.5.dstore_n](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.dstore_n)
pub(crate) fn dstore_3(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<()> {
    let value = stack.pop_double(function_builder)?;
    locals.set_double(function_builder, 3, value)?;
    Ok(())
}

/// # References
///
/// - [JVMS §6.5.daload](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.daload)
pub(crate) fn daload(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
    helpers: &RuntimeHelpers,
) -> Result<()> {
    let value = array_load(function_builder, stack, helpers.daload)?;
    stack.push_double(function_builder, value)?;
    Ok(())
}

/// # References
///
/// - [JVMS §6.5.dastore](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.dastore)
pub(crate) fn dastore(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
    helpers: &RuntimeHelpers,
) -> Result<()> {
    let value = stack.pop_double(function_builder)?;
    array_store(function_builder, stack, helpers.dastore, value)
}

/// # References
///
/// - [JVMS §6.5.dadd](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.dadd)
pub(crate) fn dadd(function_builder: &mut FunctionBuilder, stack: &mut OperandStack) -> Result<()> {
    let value2 = stack.pop_double(function_builder)?;
    let value1 = stack.pop_double(function_builder)?;
    let value = function_builder.ins().fadd(value1, value2);
    stack.push_double(function_builder, value)?;
    Ok(())
}

/// # References
///
/// - [JVMS §6.5.dsub](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.dsub)
pub(crate) fn dsub(function_builder: &mut FunctionBuilder, stack: &mut OperandStack) -> Result<()> {
    let value2 = stack.pop_double(function_builder)?;
    let value1 = stack.pop_double(function_builder)?;
    let value = function_builder.ins().fsub(value1, value2);
    stack.push_double(function_builder, value)?;
    Ok(())
}

/// # References
///
/// - [JVMS §6.5.dmul](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.dmul)
pub(crate) fn dmul(function_builder: &mut FunctionBuilder, stack: &mut OperandStack) -> Result<()> {
    let value2 = stack.pop_double(function_builder)?;
    let value1 = stack.pop_double(function_builder)?;
    let value = function_builder.ins().fmul(value1, value2);
    stack.push_double(function_builder, value)?;
    Ok(())
}

/// # References
///
/// - [JVMS §6.5.ddiv](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.ddiv)
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

/// # References
///
/// - [JVMS §6.5.drem](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.drem)
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

/// # References
///
/// - [JVMS §6.5.dneg](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.dneg)
pub(crate) fn dneg(function_builder: &mut FunctionBuilder, stack: &mut OperandStack) -> Result<()> {
    let value = stack.pop_double(function_builder)?;
    let value = function_builder.ins().fneg(value);
    stack.push_double(function_builder, value)?;
    Ok(())
}

/// # References
///
/// - [JVMS §6.5.dcmp_op](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.dcmp_op)
pub(crate) fn dcmpl(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
) -> Result<()> {
    let value2 = stack.pop_double(function_builder)?;
    let value1 = stack.pop_double(function_builder)?;
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
    // NaN -> -1 for dcmpl
    let result = function_builder.ins().select(is_nan, neg_one, zero);
    let result = function_builder.ins().select(is_less, neg_one, result);
    let result = function_builder.ins().select(is_greater, one, result);
    stack.push_int(function_builder, result)?;
    Ok(())
}

/// # References
///
/// - [JVMS §6.5.dcmp_op](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.dcmp_op)
pub(crate) fn dcmpg(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
) -> Result<()> {
    let value2 = stack.pop_double(function_builder)?;
    let value1 = stack.pop_double(function_builder)?;
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
    // NaN -> 1 for dcmpg
    let result = function_builder.ins().select(is_nan, one, zero);
    let result = function_builder.ins().select(is_less, neg_one, result);
    let result = function_builder.ins().select(is_greater, one, result);
    stack.push_int(function_builder, result)?;
    Ok(())
}

/// # References
///
/// - [JVMS §6.5.dreturn](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.dreturn)
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
