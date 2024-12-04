use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.awt.X11InputMethod`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/awt/X11InputMethod";
    registry.register(class_name, "disposeXIC", "()V", dispose_xic);
    registry.register(class_name, "initIDs", "()V", init_ids);
    registry.register(
        class_name,
        "isCompositionEnabledNative",
        "()Z",
        is_composition_enabled_native,
    );
    registry.register(class_name, "resetXIC", "()Ljava/lang/String;", reset_xic);
    registry.register(
        class_name,
        "setCompositionEnabledNative",
        "(Z)Z",
        set_composition_enabled_native,
    );
    registry.register(
        class_name,
        "turnoffStatusWindow",
        "()V",
        turnoff_status_window,
    );
}

#[async_recursion(?Send)]
async fn dispose_xic(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn is_composition_enabled_native(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn reset_xic(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn set_composition_enabled_native(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn turnoff_status_window(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}
