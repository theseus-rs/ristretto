use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `com.apple.eawt._AppMenuBarHandler`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "com/apple/eawt/_AppMenuBarHandler";
    registry.register(
        class_name,
        "nativeSetDefaultMenuBar",
        "(J)V",
        native_set_default_menu_bar,
    );
    registry.register(
        class_name,
        "nativeSetMenuState",
        "(IZZ)V",
        native_set_menu_state,
    );
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn native_set_default_menu_bar(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn native_set_menu_state(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}
