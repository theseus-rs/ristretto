use crate::Error::{InvalidLocalVariableIndex, OperandStackUnderflow};
use crate::{Result, jit_value};
use cranelift::codegen::ir::Value;
use cranelift::prelude::{EntityRef, FunctionBuilder, InstBuilder, MemFlags, Variable, types};

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.iconst_i>
pub(crate) fn iconst_m1(function_builder: &mut FunctionBuilder, stack: &mut Vec<Value>) {
    let constant = function_builder.ins().iconst(types::I32, -1);
    stack.push(constant);
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.iconst_i>
pub(crate) fn iconst_0(function_builder: &mut FunctionBuilder, stack: &mut Vec<Value>) {
    let constant = function_builder.ins().iconst(types::I32, 0);
    stack.push(constant);
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.iconst_i>
pub(crate) fn iconst_1(function_builder: &mut FunctionBuilder, stack: &mut Vec<Value>) {
    let constant = function_builder.ins().iconst(types::I32, 1);
    stack.push(constant);
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.iconst_i>
pub(crate) fn iconst_2(function_builder: &mut FunctionBuilder, stack: &mut Vec<Value>) {
    let constant = function_builder.ins().iconst(types::I32, 2);
    stack.push(constant);
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.iconst_i>
pub(crate) fn iconst_3(function_builder: &mut FunctionBuilder, stack: &mut Vec<Value>) {
    let constant = function_builder.ins().iconst(types::I32, 3);
    stack.push(constant);
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.iconst_i>
pub(crate) fn iconst_4(function_builder: &mut FunctionBuilder, stack: &mut Vec<Value>) {
    let constant = function_builder.ins().iconst(types::I32, 4);
    stack.push(constant);
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.iconst_i>
pub(crate) fn iconst_5(function_builder: &mut FunctionBuilder, stack: &mut Vec<Value>) {
    let constant = function_builder.ins().iconst(types::I32, 5);
    stack.push(constant);
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.iload>
pub(crate) fn iload(
    function_builder: &mut FunctionBuilder,
    stack: &mut Vec<Value>,
    index: u8,
) -> Result<()> {
    let index = usize::from(index);
    let variable = Variable::new(index);
    let Ok(value) = function_builder.try_use_var(variable) else {
        return Err(InvalidLocalVariableIndex(index));
    };
    stack.push(value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.iload>
/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.wide>
pub(crate) fn iload_w(
    function_builder: &mut FunctionBuilder,
    stack: &mut Vec<Value>,
    index: u16,
) -> Result<()> {
    let index = usize::from(index);
    let variable = Variable::new(index);
    let Ok(value) = function_builder.try_use_var(variable) else {
        return Err(InvalidLocalVariableIndex(index));
    };
    stack.push(value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.iload_n>
pub(crate) fn iload_0(
    function_builder: &mut FunctionBuilder,
    stack: &mut Vec<Value>,
) -> Result<()> {
    let index = 0;
    let variable = Variable::new(index);
    let Ok(value) = function_builder.try_use_var(variable) else {
        return Err(InvalidLocalVariableIndex(index));
    };
    stack.push(value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.iload_n>
pub(crate) fn iload_1(
    function_builder: &mut FunctionBuilder,
    stack: &mut Vec<Value>,
) -> Result<()> {
    let index = 1;
    let variable = Variable::new(index);
    let Ok(value) = function_builder.try_use_var(variable) else {
        return Err(InvalidLocalVariableIndex(index));
    };
    stack.push(value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.iload_n>
pub(crate) fn iload_2(
    function_builder: &mut FunctionBuilder,
    stack: &mut Vec<Value>,
) -> Result<()> {
    let index = 2;
    let variable = Variable::new(index);
    let Ok(value) = function_builder.try_use_var(variable) else {
        return Err(InvalidLocalVariableIndex(index));
    };
    stack.push(value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.iload_n>
pub(crate) fn iload_3(
    function_builder: &mut FunctionBuilder,
    stack: &mut Vec<Value>,
) -> Result<()> {
    let index = 3;
    let variable = Variable::new(index);
    let Ok(value) = function_builder.try_use_var(variable) else {
        return Err(InvalidLocalVariableIndex(index));
    };
    stack.push(value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.istore>
pub(crate) fn istore(
    function_builder: &mut FunctionBuilder,
    stack: &mut Vec<Value>,
    index: u8,
) -> Result<()> {
    let value = stack.pop().ok_or(OperandStackUnderflow)?;
    let index = usize::from(index);
    let variable = Variable::new(index);
    if function_builder.try_def_var(variable, value).is_err() {
        function_builder.declare_var(variable, types::I32);
        function_builder.def_var(variable, value);
    }
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.istore>
/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.wide>
pub(crate) fn istore_w(
    function_builder: &mut FunctionBuilder,
    stack: &mut Vec<Value>,
    index: u16,
) -> Result<()> {
    let value = stack.pop().ok_or(OperandStackUnderflow)?;
    let index = usize::from(index);
    let variable = Variable::new(index);
    if function_builder.try_def_var(variable, value).is_err() {
        function_builder.declare_var(variable, types::I32);
        function_builder.def_var(variable, value);
    }
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.istore_n>
pub(crate) fn istore_0(
    function_builder: &mut FunctionBuilder,
    stack: &mut Vec<Value>,
) -> Result<()> {
    let value = stack.pop().ok_or(OperandStackUnderflow)?;
    let variable = Variable::new(0);
    if function_builder.try_def_var(variable, value).is_err() {
        function_builder.declare_var(variable, types::I32);
        function_builder.def_var(variable, value);
    }
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.istore_n>
pub(crate) fn istore_1(
    function_builder: &mut FunctionBuilder,
    stack: &mut Vec<Value>,
) -> Result<()> {
    let value = stack.pop().ok_or(OperandStackUnderflow)?;
    let variable = Variable::new(1);
    if function_builder.try_def_var(variable, value).is_err() {
        function_builder.declare_var(variable, types::I32);
        function_builder.def_var(variable, value);
    }
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.istore_n>
pub(crate) fn istore_2(
    function_builder: &mut FunctionBuilder,
    stack: &mut Vec<Value>,
) -> Result<()> {
    let value = stack.pop().ok_or(OperandStackUnderflow)?;
    let variable = Variable::new(2);
    if function_builder.try_def_var(variable, value).is_err() {
        function_builder.declare_var(variable, types::I32);
        function_builder.def_var(variable, value);
    }
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.istore_n>
pub(crate) fn istore_3(
    function_builder: &mut FunctionBuilder,
    stack: &mut Vec<Value>,
) -> Result<()> {
    let value = stack.pop().ok_or(OperandStackUnderflow)?;
    let variable = Variable::new(3);
    if function_builder.try_def_var(variable, value).is_err() {
        function_builder.declare_var(variable, types::I32);
        function_builder.def_var(variable, value);
    }
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.iadd>
pub(crate) fn iadd(function_builder: &mut FunctionBuilder, stack: &mut Vec<Value>) -> Result<()> {
    let value2 = stack.pop().ok_or(OperandStackUnderflow)?;
    let value1 = stack.pop().ok_or(OperandStackUnderflow)?;
    let value = function_builder.ins().iadd(value1, value2);
    stack.push(value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.isub>
pub(crate) fn isub(function_builder: &mut FunctionBuilder, stack: &mut Vec<Value>) -> Result<()> {
    let value2 = stack.pop().ok_or(OperandStackUnderflow)?;
    let value1 = stack.pop().ok_or(OperandStackUnderflow)?;
    let value = function_builder.ins().isub(value1, value2);
    stack.push(value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.imul>
pub(crate) fn imul(function_builder: &mut FunctionBuilder, stack: &mut Vec<Value>) -> Result<()> {
    let value2 = stack.pop().ok_or(OperandStackUnderflow)?;
    let value1 = stack.pop().ok_or(OperandStackUnderflow)?;
    let value = function_builder.ins().imul(value1, value2);
    stack.push(value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.idiv>
pub(crate) fn idiv(function_builder: &mut FunctionBuilder, stack: &mut Vec<Value>) -> Result<()> {
    let value2 = stack.pop().ok_or(OperandStackUnderflow)?;
    let value1 = stack.pop().ok_or(OperandStackUnderflow)?;
    let value = function_builder.ins().sdiv(value1, value2);
    // TODO: Handle division by zero
    // stack.push_int(
    //     value1
    //         .checked_div(value2)
    //         .ok_or(ArithmeticException("/ by zero".to_string()))?,
    // )?;
    stack.push(value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.irem>
pub(crate) fn irem(function_builder: &mut FunctionBuilder, stack: &mut Vec<Value>) -> Result<()> {
    let value2 = stack.pop().ok_or(OperandStackUnderflow)?;
    let value1 = stack.pop().ok_or(OperandStackUnderflow)?;
    let value = function_builder.ins().srem(value1, value2);
    // TODO: Handle division by zero
    // stack.push_int(
    //     value1
    //         .checked_rem(value2)
    //         .ok_or(ArithmeticException("/ by zero".to_string()))?,
    // )?;
    stack.push(value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.ineg>
pub(crate) fn ineg(function_builder: &mut FunctionBuilder, stack: &mut Vec<Value>) -> Result<()> {
    let value = stack.pop().ok_or(OperandStackUnderflow)?;
    let value = function_builder.ins().ineg(value);
    stack.push(value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.ishl>
pub(crate) fn ishl(function_builder: &mut FunctionBuilder, stack: &mut Vec<Value>) -> Result<()> {
    let value2 = stack.pop().ok_or(OperandStackUnderflow)?;
    let value1 = stack.pop().ok_or(OperandStackUnderflow)?;
    let mask = function_builder.ins().iconst(types::I32, 0x1f);
    let value2 = function_builder.ins().band(value2, mask);
    let value = function_builder.ins().ishl(value1, value2);
    stack.push(value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.ishr>
pub(crate) fn ishr(function_builder: &mut FunctionBuilder, stack: &mut Vec<Value>) -> Result<()> {
    let value2 = stack.pop().ok_or(OperandStackUnderflow)?;
    let value1 = stack.pop().ok_or(OperandStackUnderflow)?;
    let mask = function_builder.ins().iconst(types::I32, 0x1f);
    let value2 = function_builder.ins().band(value2, mask);
    let value = function_builder.ins().sshr(value1, value2);
    stack.push(value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.iushr>
pub(crate) fn iushr(function_builder: &mut FunctionBuilder, stack: &mut Vec<Value>) -> Result<()> {
    let value2 = stack.pop().ok_or(OperandStackUnderflow)?;
    let value1 = stack.pop().ok_or(OperandStackUnderflow)?;
    let mask = function_builder.ins().iconst(types::I32, 0x1f);
    let value2 = function_builder.ins().band(value2, mask);
    let value = function_builder.ins().ushr(value1, value2);
    stack.push(value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.iand>
pub(crate) fn iand(function_builder: &mut FunctionBuilder, stack: &mut Vec<Value>) -> Result<()> {
    let value2 = stack.pop().ok_or(OperandStackUnderflow)?;
    let value1 = stack.pop().ok_or(OperandStackUnderflow)?;
    let value = function_builder.ins().band(value1, value2);
    stack.push(value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.ior>
pub(crate) fn ior(function_builder: &mut FunctionBuilder, stack: &mut Vec<Value>) -> Result<()> {
    let value2 = stack.pop().ok_or(OperandStackUnderflow)?;
    let value1 = stack.pop().ok_or(OperandStackUnderflow)?;
    let value = function_builder.ins().bor(value1, value2);
    stack.push(value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.ixor>
pub(crate) fn ixor(function_builder: &mut FunctionBuilder, stack: &mut Vec<Value>) -> Result<()> {
    let value2 = stack.pop().ok_or(OperandStackUnderflow)?;
    let value1 = stack.pop().ok_or(OperandStackUnderflow)?;
    let value = function_builder.ins().bxor(value1, value2);
    stack.push(value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.iinc>
pub(crate) fn iinc(function_builder: &mut FunctionBuilder, index: u8, constant: i8) -> Result<()> {
    let index = usize::from(index);
    let constant = i64::from(constant);
    let variable = Variable::new(index);
    let Ok(value) = function_builder.try_use_var(variable) else {
        return Err(InvalidLocalVariableIndex(index));
    };
    let value = function_builder.ins().iadd_imm(value, constant);
    function_builder.def_var(variable, value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.iinc>
/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.wide>
pub(crate) fn iinc_w(
    function_builder: &mut FunctionBuilder,
    index: u16,
    constant: i16,
) -> Result<()> {
    let index = usize::from(index);
    let constant = i64::from(constant);
    let variable = Variable::new(index);
    let Ok(value) = function_builder.try_use_var(variable) else {
        return Err(InvalidLocalVariableIndex(index));
    };
    let value = function_builder.ins().iadd_imm(value, constant);
    function_builder.def_var(variable, value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.ireturn>
pub(crate) fn ireturn(
    function_builder: &mut FunctionBuilder,
    stack: &mut Vec<Value>,
    return_pointer: Value,
) -> Result<()> {
    let value = stack.pop().ok_or(OperandStackUnderflow)?;
    let value = function_builder.ins().sextend(types::I64, value);
    let discriminate = i64::from(jit_value::I32);
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
