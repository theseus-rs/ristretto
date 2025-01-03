use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `com.apple.laf.ScreenMenu`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "com/apple/laf/ScreenMenu";
    registry.register(
        class_name,
        "addMenuListeners",
        "(Lcom/apple/laf/ScreenMenu;J)J",
        add_menu_listeners,
    );
    registry.register(
        class_name,
        "removeMenuListeners",
        "(J)V",
        remove_menu_listeners,
    );
}

#[async_recursion(?Send)]
async fn add_menu_listeners(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.apple.laf.ScreenMenu.addMenuListeners(Lcom/apple/laf/ScreenMenu;J)J")
}

#[async_recursion(?Send)]
async fn remove_menu_listeners(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.apple.laf.ScreenMenu.removeMenuListeners(J)V")
}
