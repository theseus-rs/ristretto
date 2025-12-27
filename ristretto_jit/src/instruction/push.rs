use crate::Result;
use crate::operand_stack::OperandStack;
use cranelift::frontend::FunctionBuilder;
use cranelift::prelude::{InstBuilder, types};

/// # References
/// - [JVMS §6.5.bipush](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.bipush)
pub(crate) fn bipush(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
    value: i8,
) -> Result<()> {
    let value = i64::from(value);
    let value = function_builder.ins().iconst(types::I32, value);
    stack.push_int(function_builder, value)?;
    Ok(())
}

/// # References
/// - [JVMS §6.5.sipush](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.sipush)
pub(crate) fn sipush(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
    value: i16,
) -> Result<()> {
    let value = i64::from(value);
    let value = function_builder.ins().iconst(types::I32, value);
    stack.push_int(function_builder, value)?;
    Ok(())
}
