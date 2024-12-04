use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.lwawt.macosx.CMenuBar`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/lwawt/macosx/CMenuBar";
    registry.register(
        class_name,
        "nativeCreateMenuBar",
        "()J",
        native_create_menu_bar,
    );
    registry.register(class_name, "nativeDelMenu", "(JI)V", native_del_menu);
    registry.register(
        class_name,
        "nativeSetHelpMenu",
        "(JJ)V",
        native_set_help_menu,
    );
}

#[async_recursion(?Send)]
async fn native_create_menu_bar(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn native_del_menu(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn native_set_help_menu(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}
