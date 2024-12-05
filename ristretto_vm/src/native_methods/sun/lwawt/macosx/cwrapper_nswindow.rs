use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.lwawt.macosx.CWrapper$NSWindow`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/lwawt/macosx/CWrapper$NSWindow";
    registry.register(class_name, "addChildWindow", "(JJI)V", add_child_window);
    registry.register(
        class_name,
        "canBecomeMainWindow",
        "(J)Z",
        can_become_main_window,
    );
    registry.register(class_name, "close", "(J)V", close);
    registry.register(class_name, "deminiaturize", "(J)V", deminiaturize);
    registry.register(class_name, "isKeyWindow", "(J)Z", is_key_window);
    registry.register(class_name, "isZoomed", "(J)Z", is_zoomed);
    registry.register(
        class_name,
        "makeFirstResponder",
        "(JJ)V",
        make_first_responder,
    );
    registry.register(
        class_name,
        "makeKeyAndOrderFront",
        "(J)V",
        make_key_and_order_front,
    );
    registry.register(class_name, "makeKeyWindow", "(J)V", make_key_window);
    registry.register(class_name, "makeMainWindow", "(J)V", make_main_window);
    registry.register(class_name, "miniaturize", "(J)V", miniaturize);
    registry.register(class_name, "orderFront", "(J)V", order_front);
    registry.register(
        class_name,
        "orderFrontRegardless",
        "(J)V",
        order_front_regardless,
    );
    registry.register(class_name, "orderOut", "(J)V", order_out);
    registry.register(class_name, "orderWindow", "(JIJ)V", order_window);
    registry.register(
        class_name,
        "removeChildWindow",
        "(JJ)V",
        remove_child_window,
    );
    registry.register(class_name, "setAlphaValue", "(JF)V", set_alpha_value);
    registry.register(
        class_name,
        "setBackgroundColor",
        "(JI)V",
        set_background_color,
    );
    registry.register(class_name, "setLevel", "(JI)V", set_level);
    registry.register(class_name, "setOpaque", "(JZ)V", set_opaque);
    registry.register(class_name, "zoom", "(J)V", zoom);
}

#[async_recursion(?Send)]
async fn add_child_window(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSWindow.addChildWindow(JJI)V")
}

#[async_recursion(?Send)]
async fn can_become_main_window(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSWindow.canBecomeMainWindow(J)Z")
}

#[async_recursion(?Send)]
async fn close(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSWindow.close(J)V")
}

#[async_recursion(?Send)]
async fn deminiaturize(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSWindow.deminiaturize(J)V")
}

#[async_recursion(?Send)]
async fn is_key_window(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSWindow.isKeyWindow(J)Z")
}

#[async_recursion(?Send)]
async fn is_zoomed(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSWindow.isZoomed(J)Z")
}

#[async_recursion(?Send)]
async fn make_first_responder(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSWindow.makeFirstResponder(JJ)V")
}

#[async_recursion(?Send)]
async fn make_key_and_order_front(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSWindow.makeKeyAndOrderFront(J)V")
}

#[async_recursion(?Send)]
async fn make_key_window(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSWindow.makeKeyWindow(J)V")
}

#[async_recursion(?Send)]
async fn make_main_window(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSWindow.makeMainWindow(J)V")
}

#[async_recursion(?Send)]
async fn miniaturize(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSWindow.miniaturize(J)V")
}

#[async_recursion(?Send)]
async fn order_front(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSWindow.orderFront(J)V")
}

#[async_recursion(?Send)]
async fn order_front_regardless(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSWindow.orderFrontRegardless(J)V")
}

#[async_recursion(?Send)]
async fn order_out(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSWindow.orderOut(J)V")
}

#[async_recursion(?Send)]
async fn order_window(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSWindow.orderWindow(JIJ)V")
}

#[async_recursion(?Send)]
async fn remove_child_window(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSWindow.removeChildWindow(JJ)V")
}

#[async_recursion(?Send)]
async fn set_alpha_value(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSWindow.setAlphaValue(JF)V")
}

#[async_recursion(?Send)]
async fn set_background_color(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSWindow.setBackgroundColor(JI)V")
}

#[async_recursion(?Send)]
async fn set_level(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSWindow.setLevel(JI)V")
}

#[async_recursion(?Send)]
async fn set_opaque(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSWindow.setOpaque(JZ)V")
}

#[async_recursion(?Send)]
async fn zoom(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSWindow.zoom(J)V")
}
