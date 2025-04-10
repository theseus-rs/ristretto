use crate::Error::{InvalidLocalVariableIndex, OperandStackUnderflow};
use crate::{Result, jit_value};
use cranelift::frontend::FunctionBuilder;
use cranelift::prelude::{InstBuilder, MemFlags, Value, types};

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dconst_d>
pub(crate) fn dconst_0(function_builder: &mut FunctionBuilder, stack: &mut Vec<Value>) {
    let constant = function_builder.ins().f64const(0.0);
    stack.push(constant);
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dconst_d>
pub(crate) fn dconst_1(function_builder: &mut FunctionBuilder, stack: &mut Vec<Value>) {
    let constant = function_builder.ins().f64const(1.0);
    stack.push(constant);
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dload>
pub(crate) fn dload(locals: &mut [Value], stack: &mut Vec<Value>, index: u8) -> Result<()> {
    let index = usize::from(index);
    let value = locals.get(index).ok_or(InvalidLocalVariableIndex(index))?;
    stack.push(*value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dload>
/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.wide>
pub(crate) fn dload_w(locals: &mut [Value], stack: &mut Vec<Value>, index: u16) -> Result<()> {
    let index = usize::from(index);
    let value = locals.get(index).ok_or(InvalidLocalVariableIndex(index))?;
    stack.push(*value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dload_n>
pub(crate) fn dload_0(locals: &mut [Value], stack: &mut Vec<Value>) -> Result<()> {
    let index = 0;
    let value = locals.get(index).ok_or(InvalidLocalVariableIndex(index))?;
    stack.push(*value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dload_n>
pub(crate) fn dload_1(locals: &mut [Value], stack: &mut Vec<Value>) -> Result<()> {
    let index = 1;
    let value = locals.get(index).ok_or(InvalidLocalVariableIndex(index))?;
    stack.push(*value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dload_n>
pub(crate) fn dload_2(locals: &mut [Value], stack: &mut Vec<Value>) -> Result<()> {
    let index = 2;
    let value = locals.get(index).ok_or(InvalidLocalVariableIndex(index))?;
    stack.push(*value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dload_n>
pub(crate) fn dload_3(locals: &mut [Value], stack: &mut Vec<Value>) -> Result<()> {
    let index = 3;
    let value = locals.get(index).ok_or(InvalidLocalVariableIndex(index))?;
    stack.push(*value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dstore>
pub(crate) fn dstore(locals: &mut [Value], stack: &mut Vec<Value>, index: u8) -> Result<()> {
    let value = stack.pop().ok_or(OperandStackUnderflow)?;
    let index = usize::from(index);
    if index >= locals.len() {
        return Err(InvalidLocalVariableIndex(index));
    }
    locals[index] = value;
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dstore>
/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.wide>
pub(crate) fn dstore_w(locals: &mut [Value], stack: &mut Vec<Value>, index: u16) -> Result<()> {
    let value = stack.pop().ok_or(OperandStackUnderflow)?;
    let index = usize::from(index);
    if index >= locals.len() {
        return Err(InvalidLocalVariableIndex(index));
    }
    locals[index] = value;
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dstore_n>
pub(crate) fn dstore_0(locals: &mut [Value], stack: &mut Vec<Value>) -> Result<()> {
    let value = stack.pop().ok_or(OperandStackUnderflow)?;
    let index = 0;
    if index >= locals.len() {
        return Err(InvalidLocalVariableIndex(index));
    }
    locals[index] = value;
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dstore_n>
pub(crate) fn dstore_1(locals: &mut [Value], stack: &mut Vec<Value>) -> Result<()> {
    let value = stack.pop().ok_or(OperandStackUnderflow)?;
    let index = 1;
    if index >= locals.len() {
        return Err(InvalidLocalVariableIndex(index));
    }
    locals[index] = value;
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dstore_n>
pub(crate) fn dstore_2(locals: &mut [Value], stack: &mut Vec<Value>) -> Result<()> {
    let value = stack.pop().ok_or(OperandStackUnderflow)?;
    let index = 2;
    if index >= locals.len() {
        return Err(InvalidLocalVariableIndex(index));
    }
    locals[index] = value;
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dstore_n>
pub(crate) fn dstore_3(locals: &mut [Value], stack: &mut Vec<Value>) -> Result<()> {
    let value = stack.pop().ok_or(OperandStackUnderflow)?;
    let index = 3;
    if index >= locals.len() {
        return Err(InvalidLocalVariableIndex(index));
    }
    locals[index] = value;
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dadd>
pub(crate) fn dadd(function_builder: &mut FunctionBuilder, stack: &mut Vec<Value>) -> Result<()> {
    let value2 = stack.pop().ok_or(OperandStackUnderflow)?;
    let value1 = stack.pop().ok_or(OperandStackUnderflow)?;
    let value = function_builder.ins().fadd(value1, value2);
    stack.push(value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dsub>
pub(crate) fn dsub(function_builder: &mut FunctionBuilder, stack: &mut Vec<Value>) -> Result<()> {
    let value2 = stack.pop().ok_or(OperandStackUnderflow)?;
    let value1 = stack.pop().ok_or(OperandStackUnderflow)?;
    let value = function_builder.ins().fsub(value1, value2);
    stack.push(value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dmul>
pub(crate) fn dmul(function_builder: &mut FunctionBuilder, stack: &mut Vec<Value>) -> Result<()> {
    let value2 = stack.pop().ok_or(OperandStackUnderflow)?;
    let value1 = stack.pop().ok_or(OperandStackUnderflow)?;
    let value = function_builder.ins().fmul(value1, value2);
    stack.push(value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.ddiv>
pub(crate) fn ddiv(function_builder: &mut FunctionBuilder, stack: &mut Vec<Value>) -> Result<()> {
    let value2 = stack.pop().ok_or(OperandStackUnderflow)?;
    let value1 = stack.pop().ok_or(OperandStackUnderflow)?;
    let value = function_builder.ins().fdiv(value1, value2);
    // TODO: Handle division by zero
    // if value2 == 0.0 {
    //     return Err(ArithmeticException("/ by zero".to_string()).into());
    // };
    stack.push(value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.drem>
pub(crate) fn drem(function_builder: &mut FunctionBuilder, stack: &mut Vec<Value>) -> Result<()> {
    let value2 = stack.pop().ok_or(OperandStackUnderflow)?;
    let value1 = stack.pop().ok_or(OperandStackUnderflow)?;
    // TODO: optimize this if/when cranelift supports frem
    let div = function_builder.ins().fdiv(value1, value2);
    let trunc = function_builder.ins().trunc(div);
    let mul = function_builder.ins().fmul(value2, trunc);
    let value = function_builder.ins().fsub(value1, mul);
    stack.push(value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dneg>
pub(crate) fn dneg(function_builder: &mut FunctionBuilder, stack: &mut Vec<Value>) -> Result<()> {
    let value = stack.pop().ok_or(OperandStackUnderflow)?;
    let value = function_builder.ins().fneg(value);
    stack.push(value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.dreturn>
pub(crate) fn dreturn(
    function_builder: &mut FunctionBuilder,
    stack: &mut Vec<Value>,
    return_pointer: Value,
) -> Result<()> {
    let value = stack.pop().ok_or(OperandStackUnderflow)?;
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
    Ok(())
}
