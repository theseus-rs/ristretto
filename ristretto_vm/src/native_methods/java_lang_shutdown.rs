use crate::arguments::Arguments;
use crate::call_stack::CallStack;
use crate::native_methods::registry::MethodRegistry;
use crate::{Result, VM};
use ristretto_classloader::Value;

/// Register all native methods for java.lang.Shutdown.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/Shutdown";
    registry.register(class_name, "halt0", "(I)V", halt0);
}

fn halt0(_vm: &VM, _call_stack: &mut CallStack, mut arguments: Arguments) -> Result<Option<Value>> {
    let code = arguments.pop_int()?;
    std::process::exit(code);
}
