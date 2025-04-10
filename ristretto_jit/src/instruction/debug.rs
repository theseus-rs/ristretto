use cranelift::frontend::FunctionBuilder;
use cranelift::prelude::Value;

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.2>
pub(crate) fn breakpoint(_function_builder: &mut FunctionBuilder, _stack: &mut [Value]) {
    // Breakpoint instruction is reserved for debugging and implementation dependent operations.
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.2>
pub(crate) fn impdep1(_function_builder: &mut FunctionBuilder, _stack: &mut [Value]) {
    // Impdep1 instruction is reserved for debugging and implementation dependent operations.
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.2>
pub(crate) fn impdep2(_function_builder: &mut FunctionBuilder, _stack: &mut [Value]) {
    // Impdep2 instruction is reserved for debugging and implementation dependent operations.
}
