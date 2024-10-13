use crate::call_stack::CallStack;
use crate::native::registry::NativeRegistry;
use crate::{Result, VM};
use ristretto_classloader::Value;

/// Register all native methods for the system class.
pub(crate) fn register(registry: &mut NativeRegistry) {
    let class_name = "java/lang/Class";
    registry.register(class_name, "registerNatives", "()V", register_natives);
}

#[expect(clippy::needless_pass_by_value)]
#[expect(clippy::unnecessary_wraps)]
fn register_natives(
    _vm: &VM,
    _call_stack: &CallStack,
    _arguments: Vec<Value>,
) -> Result<Option<Value>> {
    Ok(None)
}
