use crate::Error::{InvalidLocalVariableIndex, OperandStackUnderflow};
use crate::{Result, jit_value};
use cranelift::frontend::{FunctionBuilder, Variable};
use cranelift::prelude::{EntityRef, InstBuilder, IntCC, MemFlags, Value, types};

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
pub(crate) fn fload(
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

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.fload>
/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.wide>
pub(crate) fn fload_w(
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

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.fload_n>
pub(crate) fn fload_0(
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

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.fload_n>
pub(crate) fn fload_1(
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

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.fload_n>
pub(crate) fn fload_2(
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

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.fload_n>
pub(crate) fn fload_3(
    function_builder: &mut FunctionBuilder,
    stack: &mut Vec<Value>,
) -> Result<()> {
    let index = 3;
    let variable = Variable::new(3);
    let Ok(value) = function_builder.try_use_var(variable) else {
        return Err(InvalidLocalVariableIndex(index));
    };
    stack.push(value);
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.fstore>
pub(crate) fn fstore(
    function_builder: &mut FunctionBuilder,
    stack: &mut Vec<Value>,
    index: u8,
) -> Result<()> {
    let value = stack.pop().ok_or(OperandStackUnderflow)?;
    let index = usize::from(index);
    let variable = Variable::new(index);
    if function_builder.try_def_var(variable, value).is_err() {
        function_builder.declare_var(variable, types::F32);
        function_builder.def_var(variable, value);
    }
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.fstore>
/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.wide>
pub(crate) fn fstore_w(
    function_builder: &mut FunctionBuilder,
    stack: &mut Vec<Value>,
    index: u16,
) -> Result<()> {
    let value = stack.pop().ok_or(OperandStackUnderflow)?;
    let index = usize::from(index);
    let variable = Variable::new(index);
    if function_builder.try_def_var(variable, value).is_err() {
        function_builder.declare_var(variable, types::F32);
        function_builder.def_var(variable, value);
    }
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.fstore_n>
pub(crate) fn fstore_0(
    function_builder: &mut FunctionBuilder,
    stack: &mut Vec<Value>,
) -> Result<()> {
    let value = stack.pop().ok_or(OperandStackUnderflow)?;
    let variable = Variable::new(0);
    if function_builder.try_def_var(variable, value).is_err() {
        function_builder.declare_var(variable, types::F32);
        function_builder.def_var(variable, value);
    }
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.fstore_n>
pub(crate) fn fstore_1(
    function_builder: &mut FunctionBuilder,
    stack: &mut Vec<Value>,
) -> Result<()> {
    let value = stack.pop().ok_or(OperandStackUnderflow)?;
    let variable = Variable::new(1);
    if function_builder.try_def_var(variable, value).is_err() {
        function_builder.declare_var(variable, types::F32);
        function_builder.def_var(variable, value);
    }
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.fstore_n>
pub(crate) fn fstore_2(
    function_builder: &mut FunctionBuilder,
    stack: &mut Vec<Value>,
) -> Result<()> {
    let value = stack.pop().ok_or(OperandStackUnderflow)?;
    let variable = Variable::new(2);
    if function_builder.try_def_var(variable, value).is_err() {
        function_builder.declare_var(variable, types::F32);
        function_builder.def_var(variable, value);
    }
    Ok(())
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.fstore_n>
pub(crate) fn fstore_3(
    function_builder: &mut FunctionBuilder,
    stack: &mut Vec<Value>,
) -> Result<()> {
    let value = stack.pop().ok_or(OperandStackUnderflow)?;
    let variable = Variable::new(3);
    if function_builder.try_def_var(variable, value).is_err() {
        function_builder.declare_var(variable, types::F32);
        function_builder.def_var(variable, value);
    }
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

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.fcmpl>
pub(crate) fn fcmpl(function_builder: &mut FunctionBuilder, stack: &mut Vec<Value>) -> Result<()> {
    let value2 = stack.pop().ok_or(OperandStackUnderflow)?;
    let value1 = stack.pop().ok_or(OperandStackUnderflow)?;

    let equal_block = function_builder.create_block();
    let else_block = function_builder.create_block();
    let greater_than_block = function_builder.create_block();
    let less_than_block = function_builder.create_block();
    let merge_block = function_builder.create_block();

    function_builder.append_block_param(merge_block, types::I32);

    // TODO: Handle f32::is_nan(value1) || f32::is_nan(value2)

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

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.fcmpg>
pub(crate) fn fcmpg(function_builder: &mut FunctionBuilder, stack: &mut Vec<Value>) -> Result<()> {
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
