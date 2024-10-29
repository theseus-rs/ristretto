use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for sun.io.Win32ErrorMode.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/io/Win32ErrorMode";
    registry.register(class_name, "setErrorMode", "(J)J", set_error_mode);
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn set_error_mode(_thread: Arc<Thread>, mut arguments: Arguments) -> Result<Option<Value>> {
    let _error_mode = arguments.pop_long()?;
    Ok(Some(Value::Long(0)))
}
