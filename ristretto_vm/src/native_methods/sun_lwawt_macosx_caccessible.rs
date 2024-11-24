use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.lwawt.macosx.CAccessible`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/lwawt/macosx/CAccessible";
    registry.register(class_name, "menuClosed", "(J)V", menu_closed);
    registry.register(class_name, "menuItemSelected", "(J)V", menu_item_selected);
    registry.register(class_name, "menuOpened", "(J)V", menu_opened);
    registry.register(
        class_name,
        "selectedTextChanged",
        "(J)V",
        selected_text_changed,
    );
    registry.register(class_name, "selectionChanged", "(J)V", selection_changed);
    registry.register(
        class_name,
        "unregisterFromCocoaAXSystem",
        "(J)V",
        unregister_from_cocoa_ax_system,
    );
    registry.register(class_name, "valueChanged", "(J)V", value_changed);
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn menu_closed(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn menu_item_selected(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn menu_opened(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn selected_text_changed(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn selection_changed(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn unregister_from_cocoa_ax_system(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn value_changed(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
