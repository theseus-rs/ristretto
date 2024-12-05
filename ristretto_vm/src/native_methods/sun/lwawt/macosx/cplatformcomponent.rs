use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.lwawt.macosx.CPlatformComponent`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/lwawt/macosx/CPlatformComponent";
    registry.register(
        class_name,
        "nativeCreateComponent",
        "(J)J",
        native_create_component,
    );
    registry.register(class_name, "nativeSetBounds", "(JIIII)V", native_set_bounds);
}

#[async_recursion(?Send)]
async fn native_create_component(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformComponent.nativeCreateComponent(J)J")
}

#[async_recursion(?Send)]
async fn native_set_bounds(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformComponent.nativeSetBounds(JIIII)V")
}
