use crate::instruction::object::{aload, astore};
use crate::operand_stack::OperandStack;
use cranelift::frontend::FunctionBuilder;
use cranelift::prelude::types;

/// # References
///
/// - [JVMS §6.5.baload](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.baload)
pub(crate) fn baload(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
) -> crate::Result<()> {
    // byte/boolean is 1 byte, sign extended to int
    aload(function_builder, stack, types::I8, 1, true, false)
}

/// # References
///
/// - [JVMS §6.5.bastore](https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.bastore)
pub(crate) fn bastore(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
) -> crate::Result<()> {
    astore(function_builder, stack, types::I8, 1)
}
