use crate::instruction::object::{aload, astore};
use crate::operand_stack::OperandStack;
use cranelift::frontend::FunctionBuilder;
use cranelift::prelude::types;

/// # References
/// - [JVMS §6.5.saload](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.saload)
pub(crate) fn saload(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
) -> crate::Result<()> {
    // short is 2 bytes, sign extended to int
    aload(function_builder, stack, types::I16, 2, true, false)
}

/// # References
/// - [JVMS §6.5.sastore](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.sastore)
pub(crate) fn sastore(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
) -> crate::Result<()> {
    astore(function_builder, stack, types::I16, 2)
}
