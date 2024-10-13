use crate::arguments::Arguments;
use crate::call_stack::CallStack;
use crate::native_methods::registry::MethodRegistry;
use crate::Result;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for sun.io.Win32ErrorMode.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/io/Win32ErrorMode";
    registry.register(class_name, "setErrorMode", "(J)J", set_error_mode);
}

fn set_error_mode(_call_stack: &Arc<CallStack>, mut arguments: Arguments) -> Result<Option<Value>> {
    let _error_mode = arguments.pop_long()?;
    Ok(Some(Value::Long(0)))
}
