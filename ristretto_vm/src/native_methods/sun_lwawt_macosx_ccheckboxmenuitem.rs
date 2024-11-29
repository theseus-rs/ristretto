use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.lwawt.macosx.CCheckboxMenuItem`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/lwawt/macosx/CCheckboxMenuItem";
    registry.register(
        class_name,
        "nativeSetIsCheckbox",
        "(J)V",
        native_set_is_checkbox,
    );
    registry.register(class_name, "nativeSetState", "(JZ)V", native_set_state);
}

#[async_recursion(?Send)]
async fn native_set_is_checkbox(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn native_set_state(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
