use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.lwawt.macosx.CWrapper$NSView`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/lwawt/macosx/CWrapper$NSView";
    registry.register(class_name, "addSubview", "(JJ)V", add_subview);
    registry.register(
        class_name,
        "removeFromSuperview",
        "(J)V",
        remove_from_superview,
    );
    registry.register(class_name, "setFrame", "(JIIII)V", set_frame);
    registry.register(class_name, "setHidden", "(JZ)V", set_hidden);
    registry.register(
        class_name,
        "setToolTip",
        "(JLjava/lang/String;)V",
        set_tool_tip,
    );
    registry.register(class_name, "window", "(J)J", window);
}

#[async_recursion(?Send)]
async fn add_subview(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSView.addSubview(JJ)V")
}

#[async_recursion(?Send)]
async fn remove_from_superview(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSView.removeFromSuperview(J)V")
}

#[async_recursion(?Send)]
async fn set_frame(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSView.setFrame(JIIII)V")
}

#[async_recursion(?Send)]
async fn set_hidden(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSView.setHidden(JZ)V")
}

#[async_recursion(?Send)]
async fn set_tool_tip(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSView.setToolTip(JLjava/lang/String;)V")
}

#[async_recursion(?Send)]
async fn window(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSView.window(J)J")
}
