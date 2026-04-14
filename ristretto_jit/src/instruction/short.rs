use crate::instruction::object::{array_load, array_store};
use crate::operand_stack::OperandStack;
use crate::runtime_helpers::RuntimeHelpers;
use cranelift::frontend::FunctionBuilder;

/// # References
/// - [JVMS §6.5.saload](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.saload)
pub(crate) fn saload(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
    helpers: &RuntimeHelpers,
) -> crate::Result<()> {
    let value = array_load(function_builder, stack, helpers.saload)?;
    stack.push_int(function_builder, value)?;
    Ok(())
}

/// # References
/// - [JVMS §6.5.sastore](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.sastore)
pub(crate) fn sastore(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
    helpers: &RuntimeHelpers,
) -> crate::Result<()> {
    let value = stack.pop_int(function_builder)?;
    array_store(function_builder, stack, helpers.sastore, value)
}
