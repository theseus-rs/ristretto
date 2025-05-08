use crate::operand_stack::OperandStack;
use cranelift::frontend::FunctionBuilder;

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.monitorenter>
pub(crate) fn monitorenter(function_builder: &mut FunctionBuilder, stack: &mut OperandStack) {
    // The monitorenter instruction is not currently used by this implementation.
    let _ = stack.pop(function_builder);
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.monitorexit>
pub(crate) fn monitorexit(function_builder: &mut FunctionBuilder, stack: &mut OperandStack) {
    // The monitorexit instruction is not currently used by this implementation.
    let _ = stack.pop(function_builder);
}
