use crate::call_stack::CallStack;
use crate::native::registry::NativeRegistry;
use crate::Error::RuntimeError;
use crate::{Result, VM};
use ristretto_classloader::Value;

/// Register all native methods for the system class.
pub(crate) fn register(registry: &mut NativeRegistry) {
    let class_name = "java/lang/Shutdown";
    registry.register(class_name, "halt0", "(I)V", halt0);
}

#[expect(clippy::needless_pass_by_value)]
fn halt0(_vm: &VM, _call_stack: &mut CallStack, arguments: Vec<Value>) -> Result<Option<Value>> {
    let Some(Value::Int(code)) = arguments.first() else {
        return Err(RuntimeError("exit status must be an integer".to_string()));
    };
    std::process::exit(*code);
}
