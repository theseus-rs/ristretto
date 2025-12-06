use crate::operand_stack::OperandStack;
use cranelift::frontend::FunctionBuilder;

/// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.2>
pub(crate) fn breakpoint(_function_builder: &mut FunctionBuilder, _stack: &mut OperandStack) {
    // Breakpoint instruction is reserved for debugging and implementation dependent operations.
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.2>
pub(crate) fn impdep1(_function_builder: &mut FunctionBuilder, _stack: &mut OperandStack) {
    // Impdep1 instruction is reserved for debugging and implementation dependent operations.
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se25/html/jvms-6.html#jvms-6.2>
pub(crate) fn impdep2(_function_builder: &mut FunctionBuilder, _stack: &mut OperandStack) {
    // Impdep2 instruction is reserved for debugging and implementation dependent operations.
}
