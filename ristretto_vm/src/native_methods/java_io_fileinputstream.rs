use crate::arguments::Arguments;
use crate::call_stack::CallStack;
use crate::native_methods::registry::MethodRegistry;
use crate::{Result, VM};
use ristretto_classloader::Value;

/// Register all native methods for java.io.FileInputStream.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/io/FileInputStream";
    registry.register(class_name, "initIDs", "()V", init_ids);
}

#[expect(clippy::needless_pass_by_value)]
#[expect(clippy::unnecessary_wraps)]
fn init_ids(_vm: &VM, _call_stack: &CallStack, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}
