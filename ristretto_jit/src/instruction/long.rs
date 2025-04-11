use crate::Error::{InvalidLocalVariableIndex, OperandStackUnderflow};
use crate::{Result, jit_value};
use cranelift::frontend::FunctionBuilder;
use cranelift::prelude::{InstBuilder, IntCC, MemFlags, Value, types};

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.lconst_l>
pub(crate) fn lconst_0(function_builder: &mut FunctionBuilder, stack: &mut Vec<Value>) {
    let constant = function_builder.ins().iconst(types::I64, 0);
    stack.push(constant);
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.lconst_l>
pub(crate) fn lconst_1(function_builder: &mut FunctionBuilder, stack: &mut Vec<Value>) {
    let constant = function_builder.ins().iconst(types::I64, 1);
    stack.push(constant);
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.lload>
pub(crate) fn lload(locals: &mut [Value], stack: &mut Vec<Value>, index: u8) -> Result<()> {
    let index = usize::from(index);
    let value = locals.get(index).ok_or(InvalidLocalVariableIndex(index))?;
    stack.push(*value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.lload>
/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.wide>
pub(crate) fn lload_w(locals: &mut [Value], stack: &mut Vec<Value>, index: u16) -> Result<()> {
    let index = usize::from(index);
    let value = locals.get(index).ok_or(InvalidLocalVariableIndex(index))?;
    stack.push(*value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.lload_n>
pub(crate) fn lload_0(locals: &mut [Value], stack: &mut Vec<Value>) -> Result<()> {
    let index = 0;
    let value = locals.get(index).ok_or(InvalidLocalVariableIndex(index))?;
    stack.push(*value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.lload_n>
pub(crate) fn lload_1(locals: &mut [Value], stack: &mut Vec<Value>) -> Result<()> {
    let index = 1;
    let value = locals.get(index).ok_or(InvalidLocalVariableIndex(index))?;
    stack.push(*value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.lload_n>
pub(crate) fn lload_2(locals: &mut [Value], stack: &mut Vec<Value>) -> Result<()> {
    let index = 2;
    let value = locals.get(index).ok_or(InvalidLocalVariableIndex(index))?;
    stack.push(*value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.lload_n>
pub(crate) fn lload_3(locals: &mut [Value], stack: &mut Vec<Value>) -> Result<()> {
    let index = 3;
    let value = locals.get(index).ok_or(InvalidLocalVariableIndex(index))?;
    stack.push(*value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.lstore>
pub(crate) fn lstore(locals: &mut [Value], stack: &mut Vec<Value>, index: u8) -> Result<()> {
    let value = stack.pop().ok_or(OperandStackUnderflow)?;
    let index = usize::from(index);
    if index >= locals.len() {
        return Err(InvalidLocalVariableIndex(index));
    }
    locals[index] = value;
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.lstore>
/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.wide>
pub(crate) fn lstore_w(locals: &mut [Value], stack: &mut Vec<Value>, index: u16) -> Result<()> {
    let value = stack.pop().ok_or(OperandStackUnderflow)?;
    let index = usize::from(index);
    if index >= locals.len() {
        return Err(InvalidLocalVariableIndex(index));
    }
    locals[index] = value;
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.lstore_n>
pub(crate) fn lstore_0(locals: &mut [Value], stack: &mut Vec<Value>) -> Result<()> {
    let value = stack.pop().ok_or(OperandStackUnderflow)?;
    let index = 0;
    if index >= locals.len() {
        return Err(InvalidLocalVariableIndex(index));
    }
    locals[index] = value;
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.lstore_n>
pub(crate) fn lstore_1(locals: &mut [Value], stack: &mut Vec<Value>) -> Result<()> {
    let value = stack.pop().ok_or(OperandStackUnderflow)?;
    let index = 1;
    if index >= locals.len() {
        return Err(InvalidLocalVariableIndex(index));
    }
    locals[index] = value;
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.lstore_n>
pub(crate) fn lstore_2(locals: &mut [Value], stack: &mut Vec<Value>) -> Result<()> {
    let value = stack.pop().ok_or(OperandStackUnderflow)?;
    let index = 2;
    if index >= locals.len() {
        return Err(InvalidLocalVariableIndex(index));
    }
    locals[index] = value;
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.lstore_n>
pub(crate) fn lstore_3(locals: &mut [Value], stack: &mut Vec<Value>) -> Result<()> {
    let value = stack.pop().ok_or(OperandStackUnderflow)?;
    let index = 3;
    if index >= locals.len() {
        return Err(InvalidLocalVariableIndex(index));
    }
    locals[index] = value;
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.ladd>
pub(crate) fn ladd(function_builder: &mut FunctionBuilder, stack: &mut Vec<Value>) -> Result<()> {
    let value2 = stack.pop().ok_or(OperandStackUnderflow)?;
    let value1 = stack.pop().ok_or(OperandStackUnderflow)?;
    let value = function_builder.ins().iadd(value1, value2);
    stack.push(value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.lsub>
pub(crate) fn lsub(function_builder: &mut FunctionBuilder, stack: &mut Vec<Value>) -> Result<()> {
    let value2 = stack.pop().ok_or(OperandStackUnderflow)?;
    let value1 = stack.pop().ok_or(OperandStackUnderflow)?;
    let value = function_builder.ins().isub(value1, value2);
    stack.push(value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.lmul>
pub(crate) fn lmul(function_builder: &mut FunctionBuilder, stack: &mut Vec<Value>) -> Result<()> {
    let value2 = stack.pop().ok_or(OperandStackUnderflow)?;
    let value1 = stack.pop().ok_or(OperandStackUnderflow)?;
    let value = function_builder.ins().imul(value1, value2);
    stack.push(value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.ldiv>
pub(crate) fn ldiv(function_builder: &mut FunctionBuilder, stack: &mut Vec<Value>) -> Result<()> {
    let value2 = stack.pop().ok_or(OperandStackUnderflow)?;
    let value1 = stack.pop().ok_or(OperandStackUnderflow)?;
    let value = function_builder.ins().sdiv(value1, value2);
    // TODO: Handle division by zero
    // stack.push_long(
    //     value1
    //         .checked_div(value2)
    //         .ok_or(ArithmeticException("/ by zero".to_string()))?,
    // )?;
    stack.push(value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.lrem>
pub(crate) fn lrem(function_builder: &mut FunctionBuilder, stack: &mut Vec<Value>) -> Result<()> {
    let value2 = stack.pop().ok_or(OperandStackUnderflow)?;
    let value1 = stack.pop().ok_or(OperandStackUnderflow)?;
    let value = function_builder.ins().srem(value1, value2);
    // TODO: Handle division by zero
    // stack.push_long(
    //     value1
    //         .checked_rem(value2)
    //         .ok_or(ArithmeticException("/ by zero".to_string()))?,
    // )?;
    stack.push(value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.lneg>
pub(crate) fn lneg(function_builder: &mut FunctionBuilder, stack: &mut Vec<Value>) -> Result<()> {
    let value = stack.pop().ok_or(OperandStackUnderflow)?;
    let value = function_builder.ins().ineg(value);
    stack.push(value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.lshl>
pub(crate) fn lshl(function_builder: &mut FunctionBuilder, stack: &mut Vec<Value>) -> Result<()> {
    let value2 = stack.pop().ok_or(OperandStackUnderflow)?;
    let value1 = stack.pop().ok_or(OperandStackUnderflow)?;
    let value2 = function_builder.ins().sextend(types::I64, value2);
    let mask = function_builder.ins().iconst(types::I64, 0x3f);
    let value2 = function_builder.ins().band(value2, mask);
    let value = function_builder.ins().ishl(value1, value2);
    stack.push(value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.lshr>
pub(crate) fn lshr(function_builder: &mut FunctionBuilder, stack: &mut Vec<Value>) -> Result<()> {
    let value2 = stack.pop().ok_or(OperandStackUnderflow)?;
    let value1 = stack.pop().ok_or(OperandStackUnderflow)?;
    let value2 = function_builder.ins().sextend(types::I64, value2);
    let mask = function_builder.ins().iconst(types::I64, 0x3f);
    let value2 = function_builder.ins().band(value2, mask);
    let value = function_builder.ins().sshr(value1, value2);
    stack.push(value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.lushr>
pub(crate) fn lushr(function_builder: &mut FunctionBuilder, stack: &mut Vec<Value>) -> Result<()> {
    let value2 = stack.pop().ok_or(OperandStackUnderflow)?;
    let value1 = stack.pop().ok_or(OperandStackUnderflow)?;
    let value2 = function_builder.ins().sextend(types::I64, value2);
    let mask = function_builder.ins().iconst(types::I64, 0x3f);
    let value2 = function_builder.ins().band(value2, mask);
    let value = function_builder.ins().ushr(value1, value2);
    stack.push(value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.land>
pub(crate) fn land(function_builder: &mut FunctionBuilder, stack: &mut Vec<Value>) -> Result<()> {
    let value2 = stack.pop().ok_or(OperandStackUnderflow)?;
    let value1 = stack.pop().ok_or(OperandStackUnderflow)?;
    let value = function_builder.ins().band(value1, value2);
    stack.push(value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.lor>
pub(crate) fn lor(function_builder: &mut FunctionBuilder, stack: &mut Vec<Value>) -> Result<()> {
    let value2 = stack.pop().ok_or(OperandStackUnderflow)?;
    let value1 = stack.pop().ok_or(OperandStackUnderflow)?;
    let value = function_builder.ins().bor(value1, value2);
    stack.push(value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.lxor>
pub(crate) fn lxor(function_builder: &mut FunctionBuilder, stack: &mut Vec<Value>) -> Result<()> {
    let value2 = stack.pop().ok_or(OperandStackUnderflow)?;
    let value1 = stack.pop().ok_or(OperandStackUnderflow)?;
    let value = function_builder.ins().bxor(value1, value2);
    stack.push(value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.lcmp>
pub(crate) fn lcmp(function_builder: &mut FunctionBuilder, stack: &mut Vec<Value>) -> Result<()> {
    let value2 = stack.pop().ok_or(OperandStackUnderflow)?;
    let value1 = stack.pop().ok_or(OperandStackUnderflow)?;

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
    stack.push(value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.lreturn>
pub(crate) fn lreturn(
    function_builder: &mut FunctionBuilder,
    stack: &mut Vec<Value>,
    return_pointer: Value,
) -> Result<()> {
    let value = stack.pop().ok_or(OperandStackUnderflow)?;
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
