use crate::Error::{InvalidLocalVariableIndex, OperandStackUnderflow};
use crate::{Result, jit_value};
use cranelift::frontend::FunctionBuilder;
use cranelift::prelude::{InstBuilder, MemFlags, Value, types};

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.fconst_f>
pub(crate) fn fconst_0(function_builder: &mut FunctionBuilder, stack: &mut Vec<Value>) {
    let constant = function_builder.ins().f32const(0.0);
    stack.push(constant);
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.fconst_f>
pub(crate) fn fconst_1(function_builder: &mut FunctionBuilder, stack: &mut Vec<Value>) {
    let constant = function_builder.ins().f32const(1.0);
    stack.push(constant);
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.fconst_f>
pub(crate) fn fconst_2(function_builder: &mut FunctionBuilder, stack: &mut Vec<Value>) {
    let constant = function_builder.ins().f32const(2.0);
    stack.push(constant);
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.fload>
pub(crate) fn fload(locals: &mut [Value], stack: &mut Vec<Value>, index: u8) -> Result<()> {
    let index = usize::from(index);
    let value = locals.get(index).ok_or(InvalidLocalVariableIndex(index))?;
    stack.push(*value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.fload>
/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.wide>
pub(crate) fn fload_w(locals: &mut [Value], stack: &mut Vec<Value>, index: u16) -> Result<()> {
    let index = usize::from(index);
    let value = locals.get(index).ok_or(InvalidLocalVariableIndex(index))?;
    stack.push(*value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.fload_n>
pub(crate) fn fload_0(locals: &mut [Value], stack: &mut Vec<Value>) -> Result<()> {
    let index = 0;
    let value = locals.get(index).ok_or(InvalidLocalVariableIndex(index))?;
    stack.push(*value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.fload_n>
pub(crate) fn fload_1(locals: &mut [Value], stack: &mut Vec<Value>) -> Result<()> {
    let index = 1;
    let value = locals.get(index).ok_or(InvalidLocalVariableIndex(index))?;
    stack.push(*value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.fload_n>
pub(crate) fn fload_2(locals: &mut [Value], stack: &mut Vec<Value>) -> Result<()> {
    let index = 2;
    let value = locals.get(index).ok_or(InvalidLocalVariableIndex(index))?;
    stack.push(*value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.fload_n>
pub(crate) fn fload_3(locals: &mut [Value], stack: &mut Vec<Value>) -> Result<()> {
    let index = 3;
    let value = locals.get(index).ok_or(InvalidLocalVariableIndex(index))?;
    stack.push(*value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.fstore>
pub(crate) fn fstore(locals: &mut [Value], stack: &mut Vec<Value>, index: u8) -> Result<()> {
    let value = stack.pop().ok_or(OperandStackUnderflow)?;
    let index = usize::from(index);
    if index >= locals.len() {
        return Err(InvalidLocalVariableIndex(index));
    }
    locals[index] = value;
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.fstore>
/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.wide>
pub(crate) fn fstore_w(locals: &mut [Value], stack: &mut Vec<Value>, index: u16) -> Result<()> {
    let value = stack.pop().ok_or(OperandStackUnderflow)?;
    let index = usize::from(index);
    if index >= locals.len() {
        return Err(InvalidLocalVariableIndex(index));
    }
    locals[index] = value;
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.fstore_n>
pub(crate) fn fstore_0(locals: &mut [Value], stack: &mut Vec<Value>) -> Result<()> {
    let value = stack.pop().ok_or(OperandStackUnderflow)?;
    let index = 0;
    if index >= locals.len() {
        return Err(InvalidLocalVariableIndex(index));
    }
    locals[index] = value;
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.fstore_n>
pub(crate) fn fstore_1(locals: &mut [Value], stack: &mut Vec<Value>) -> Result<()> {
    let value = stack.pop().ok_or(OperandStackUnderflow)?;
    let index = 1;
    if index >= locals.len() {
        return Err(InvalidLocalVariableIndex(index));
    }
    locals[index] = value;
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.fstore_n>
pub(crate) fn fstore_2(locals: &mut [Value], stack: &mut Vec<Value>) -> Result<()> {
    let value = stack.pop().ok_or(OperandStackUnderflow)?;
    let index = 2;
    if index >= locals.len() {
        return Err(InvalidLocalVariableIndex(index));
    }
    locals[index] = value;
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.fstore_n>
pub(crate) fn fstore_3(locals: &mut [Value], stack: &mut Vec<Value>) -> Result<()> {
    let value = stack.pop().ok_or(OperandStackUnderflow)?;
    let index = 3;
    if index >= locals.len() {
        return Err(InvalidLocalVariableIndex(index));
    }
    locals[index] = value;
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.fadd>
pub(crate) fn fadd(function_builder: &mut FunctionBuilder, stack: &mut Vec<Value>) -> Result<()> {
    let value2 = stack.pop().ok_or(OperandStackUnderflow)?;
    let value1 = stack.pop().ok_or(OperandStackUnderflow)?;
    let value = function_builder.ins().fadd(value1, value2);
    stack.push(value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.fsub>
pub(crate) fn fsub(function_builder: &mut FunctionBuilder, stack: &mut Vec<Value>) -> Result<()> {
    let value2 = stack.pop().ok_or(OperandStackUnderflow)?;
    let value1 = stack.pop().ok_or(OperandStackUnderflow)?;
    let value = function_builder.ins().fsub(value1, value2);
    stack.push(value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.fmul>
pub(crate) fn fmul(function_builder: &mut FunctionBuilder, stack: &mut Vec<Value>) -> Result<()> {
    let value2 = stack.pop().ok_or(OperandStackUnderflow)?;
    let value1 = stack.pop().ok_or(OperandStackUnderflow)?;
    let value = function_builder.ins().fmul(value1, value2);
    stack.push(value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.fdiv>
pub(crate) fn fdiv(function_builder: &mut FunctionBuilder, stack: &mut Vec<Value>) -> Result<()> {
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

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.frem>
pub(crate) fn frem(function_builder: &mut FunctionBuilder, stack: &mut Vec<Value>) -> Result<()> {
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

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.fneg>
pub(crate) fn fneg(function_builder: &mut FunctionBuilder, stack: &mut Vec<Value>) -> Result<()> {
    let value = stack.pop().ok_or(OperandStackUnderflow)?;
    let value = function_builder.ins().fneg(value);
    stack.push(value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.freturn>
pub(crate) fn freturn(
    function_builder: &mut FunctionBuilder,
    stack: &mut Vec<Value>,
    return_pointer: Value,
) -> Result<()> {
    let value = stack.pop().ok_or(OperandStackUnderflow)?;
    let value = function_builder.ins().fcvt_to_sint(types::I64, value);
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
