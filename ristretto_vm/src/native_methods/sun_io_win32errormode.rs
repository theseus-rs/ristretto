use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use ristretto_classloader::Value;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Register all native methods for sun.io.Win32ErrorMode.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/io/Win32ErrorMode";
    registry.register(class_name, "setErrorMode", "(J)J", set_error_mode);
}

#[expect(clippy::needless_pass_by_value)]
fn set_error_mode(
    _thread: Arc<Thread>,
    mut arguments: Arguments,
) -> Pin<Box<dyn Future<Output = Result<Option<Value>>>>> {
    Box::pin(async move {
        let _error_mode = arguments.pop_long()?;
        Ok(Some(Value::Long(0)))
    })
}
