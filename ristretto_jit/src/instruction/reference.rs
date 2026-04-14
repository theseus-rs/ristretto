use crate::local_variables::LocalVariables;
use crate::operand_stack::OperandStack;
use crate::{Result, jit_value};
use cranelift::codegen::ir::Value;
use cranelift::prelude::{FunctionBuilder, InstBuilder, MemFlags, types};

/// # References
/// - [JVMS §6.5.aconst_null](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.aconst_null)
pub(crate) fn aconst_null(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
) -> Result<()> {
    let null = function_builder.ins().iconst(types::I64, 0);
    stack.push_object(function_builder, null)?;
    Ok(())
}

/// # References
/// - [JVMS §6.5.aload](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.aload)
pub(crate) fn aload_ref(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
    index: u8,
) -> Result<()> {
    let index = usize::from(index);
    let value = locals.get_object(function_builder, index)?;
    stack.push_object(function_builder, value)?;
    Ok(())
}

/// # References
/// - [JVMS §6.5.aload](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.aload)
/// - [JVMS §6.5.wide](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.wide)
pub(crate) fn aload_ref_w(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
    index: u16,
) -> Result<()> {
    let index = usize::from(index);
    let value = locals.get_object(function_builder, index)?;
    stack.push_object(function_builder, value)?;
    Ok(())
}

/// # References
/// - [JVMS §6.5.aload_n](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.aload_n)
pub(crate) fn aload_ref_0(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<()> {
    let value = locals.get_object(function_builder, 0)?;
    stack.push_object(function_builder, value)?;
    Ok(())
}

/// # References
/// - [JVMS §6.5.aload_n](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.aload_n)
pub(crate) fn aload_ref_1(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<()> {
    let value = locals.get_object(function_builder, 1)?;
    stack.push_object(function_builder, value)?;
    Ok(())
}

/// # References
/// - [JVMS §6.5.aload_n](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.aload_n)
pub(crate) fn aload_ref_2(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<()> {
    let value = locals.get_object(function_builder, 2)?;
    stack.push_object(function_builder, value)?;
    Ok(())
}

/// # References
/// - [JVMS §6.5.aload_n](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.aload_n)
pub(crate) fn aload_ref_3(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<()> {
    let value = locals.get_object(function_builder, 3)?;
    stack.push_object(function_builder, value)?;
    Ok(())
}

/// # References
/// - [JVMS §6.5.astore](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.astore)
pub(crate) fn astore_ref(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
    index: u8,
) -> Result<()> {
    let index = usize::from(index);
    let value = stack.pop_object(function_builder)?;
    locals.set_object(function_builder, index, value)?;
    Ok(())
}

/// # References
/// - [JVMS §6.5.astore](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.astore)
/// - [JVMS §6.5.wide](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.wide)
pub(crate) fn astore_ref_w(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
    index: u16,
) -> Result<()> {
    let index = usize::from(index);
    let value = stack.pop_object(function_builder)?;
    locals.set_object(function_builder, index, value)?;
    Ok(())
}

/// # References
/// - [JVMS §6.5.astore_n](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.astore_n)
pub(crate) fn astore_ref_0(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<()> {
    let value = stack.pop_object(function_builder)?;
    locals.set_object(function_builder, 0, value)?;
    Ok(())
}

/// # References
/// - [JVMS §6.5.astore_n](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.astore_n)
pub(crate) fn astore_ref_1(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<()> {
    let value = stack.pop_object(function_builder)?;
    locals.set_object(function_builder, 1, value)?;
    Ok(())
}

/// # References
/// - [JVMS §6.5.astore_n](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.astore_n)
pub(crate) fn astore_ref_2(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<()> {
    let value = stack.pop_object(function_builder)?;
    locals.set_object(function_builder, 2, value)?;
    Ok(())
}

/// # References
/// - [JVMS §6.5.astore_n](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.astore_n)
pub(crate) fn astore_ref_3(
    function_builder: &mut FunctionBuilder,
    locals: &mut LocalVariables,
    stack: &mut OperandStack,
) -> Result<()> {
    let value = stack.pop_object(function_builder)?;
    locals.set_object(function_builder, 3, value)?;
    Ok(())
}

/// # References
/// - [JVMS §6.5.areturn](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.areturn)
pub(crate) fn areturn(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
    return_pointer: Value,
) -> Result<()> {
    let value = stack.pop_object(function_builder)?;
    let discriminant = i64::from(jit_value::PTR);
    let discriminant = function_builder.ins().iconst(types::I8, discriminant);
    function_builder
        .ins()
        .store(MemFlags::new(), discriminant, return_pointer, 0);
    function_builder
        .ins()
        .store(MemFlags::new(), value, return_pointer, 8);
    function_builder.ins().return_(&[]);
    Ok(())
}
