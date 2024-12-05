use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.lwawt.macosx.CInputMethodDescriptor`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/lwawt/macosx/CInputMethodDescriptor";
    registry.register(
        class_name,
        "nativeGetAvailableLocales",
        "()Ljava/util/List;",
        native_get_available_locales,
    );
    registry.register(class_name, "nativeInit", "()V", native_init);
}

#[async_recursion(?Send)]
async fn native_get_available_locales(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CInputMethodDescriptor.nativeGetAvailableLocales()Ljava/util/List;")
}

#[async_recursion(?Send)]
async fn native_init(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CInputMethodDescriptor.nativeInit()V")
}
