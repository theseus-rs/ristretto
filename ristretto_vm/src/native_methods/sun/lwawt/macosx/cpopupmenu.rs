use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.lwawt.macosx.CPopupMenu`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/lwawt/macosx/CPopupMenu";
    registry.register(
        class_name,
        "nativeCreatePopupMenu",
        "()J",
        native_create_popup_menu,
    );
    registry.register(
        class_name,
        "nativeShowPopupMenu",
        "(JII)J",
        native_show_popup_menu,
    );
}

#[async_recursion(?Send)]
async fn native_create_popup_menu(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn native_show_popup_menu(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}
