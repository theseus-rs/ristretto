use crate::instruction::object::{aload, astore};
use crate::operand_stack::OperandStack;
use cranelift::frontend::FunctionBuilder;
use cranelift::prelude::types;

/// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.caload>
pub(crate) fn caload(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
) -> crate::Result<()> {
    // char is 2 bytes, zero extended to int
    aload(function_builder, stack, types::I16, 2, false, true)
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.5.castore>
pub(crate) fn castore(
    function_builder: &mut FunctionBuilder,
    stack: &mut OperandStack,
) -> crate::Result<()> {
    astore(function_builder, stack, types::I16, 2)
}
