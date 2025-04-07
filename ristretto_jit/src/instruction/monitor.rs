use cranelift::frontend::FunctionBuilder;
use cranelift::prelude::Value;

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.monitorenter>
pub(crate) fn monitorenter(_function_builder: &mut FunctionBuilder, stack: &mut Vec<Value>) {
    // The monitorenter instruction is not currently used by this implementation.
    let _ = stack.pop();
}

/// See: <https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-6.html#jvms-6.5.monitorexit>
pub(crate) fn monitorexit(_function_builder: &mut FunctionBuilder, stack: &mut Vec<Value>) {
    // The monitorexit instruction is not currently used by this implementation.
    let _ = stack.pop();
}
