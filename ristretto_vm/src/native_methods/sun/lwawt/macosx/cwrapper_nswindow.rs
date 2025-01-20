use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/lwawt/macosx/CWrapper$NSWindow";

/// Register all native methods for `sun.lwawt.macosx.CWrapper$NSWindow`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "addChildWindow", "(JJI)V", add_child_window);
    registry.register(
        CLASS_NAME,
        "canBecomeMainWindow",
        "(J)Z",
        can_become_main_window,
    );
    registry.register(CLASS_NAME, "close", "(J)V", close);
    registry.register(CLASS_NAME, "deminiaturize", "(J)V", deminiaturize);
    registry.register(CLASS_NAME, "isKeyWindow", "(J)Z", is_key_window);
    registry.register(CLASS_NAME, "isZoomed", "(J)Z", is_zoomed);
    registry.register(
        CLASS_NAME,
        "makeFirstResponder",
        "(JJ)V",
        make_first_responder,
    );
    registry.register(
        CLASS_NAME,
        "makeKeyAndOrderFront",
        "(J)V",
        make_key_and_order_front,
    );
    registry.register(CLASS_NAME, "makeKeyWindow", "(J)V", make_key_window);
    registry.register(CLASS_NAME, "makeMainWindow", "(J)V", make_main_window);
    registry.register(CLASS_NAME, "miniaturize", "(J)V", miniaturize);
    registry.register(CLASS_NAME, "orderFront", "(J)V", order_front);
    registry.register(
        CLASS_NAME,
        "orderFrontRegardless",
        "(J)V",
        order_front_regardless,
    );
    registry.register(CLASS_NAME, "orderOut", "(J)V", order_out);
    registry.register(CLASS_NAME, "orderWindow", "(JIJ)V", order_window);
    registry.register(
        CLASS_NAME,
        "removeChildWindow",
        "(JJ)V",
        remove_child_window,
    );
    registry.register(CLASS_NAME, "setAlphaValue", "(JF)V", set_alpha_value);
    registry.register(
        CLASS_NAME,
        "setBackgroundColor",
        "(JI)V",
        set_background_color,
    );
    registry.register(CLASS_NAME, "setLevel", "(JI)V", set_level);
    registry.register(CLASS_NAME, "setOpaque", "(JZ)V", set_opaque);
    registry.register(CLASS_NAME, "zoom", "(J)V", zoom);
}

#[async_recursion(?Send)]
async fn add_child_window(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSWindow.addChildWindow(JJI)V")
}

#[async_recursion(?Send)]
async fn can_become_main_window(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSWindow.canBecomeMainWindow(J)Z")
}

#[async_recursion(?Send)]
async fn close(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSWindow.close(J)V")
}

#[async_recursion(?Send)]
async fn deminiaturize(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSWindow.deminiaturize(J)V")
}

#[async_recursion(?Send)]
async fn is_key_window(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSWindow.isKeyWindow(J)Z")
}

#[async_recursion(?Send)]
async fn is_zoomed(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSWindow.isZoomed(J)Z")
}

#[async_recursion(?Send)]
async fn make_first_responder(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSWindow.makeFirstResponder(JJ)V")
}

#[async_recursion(?Send)]
async fn make_key_and_order_front(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSWindow.makeKeyAndOrderFront(J)V")
}

#[async_recursion(?Send)]
async fn make_key_window(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSWindow.makeKeyWindow(J)V")
}

#[async_recursion(?Send)]
async fn make_main_window(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSWindow.makeMainWindow(J)V")
}

#[async_recursion(?Send)]
async fn miniaturize(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSWindow.miniaturize(J)V")
}

#[async_recursion(?Send)]
async fn order_front(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSWindow.orderFront(J)V")
}

#[async_recursion(?Send)]
async fn order_front_regardless(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSWindow.orderFrontRegardless(J)V")
}

#[async_recursion(?Send)]
async fn order_out(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSWindow.orderOut(J)V")
}

#[async_recursion(?Send)]
async fn order_window(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSWindow.orderWindow(JIJ)V")
}

#[async_recursion(?Send)]
async fn remove_child_window(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSWindow.removeChildWindow(JJ)V")
}

#[async_recursion(?Send)]
async fn set_alpha_value(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSWindow.setAlphaValue(JF)V")
}

#[async_recursion(?Send)]
async fn set_background_color(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSWindow.setBackgroundColor(JI)V")
}

#[async_recursion(?Send)]
async fn set_level(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSWindow.setLevel(JI)V")
}

#[async_recursion(?Send)]
async fn set_opaque(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSWindow.setOpaque(JZ)V")
}

#[async_recursion(?Send)]
async fn zoom(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSWindow.zoom(J)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CWrapper$NSWindow.addChildWindow(JJI)V"
    )]
    async fn test_add_child_window() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = add_child_window(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CWrapper$NSWindow.canBecomeMainWindow(J)Z"
    )]
    async fn test_can_become_main_window() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = can_become_main_window(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.lwawt.macosx.CWrapper$NSWindow.close(J)V")]
    async fn test_close() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = close(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CWrapper$NSWindow.deminiaturize(J)V"
    )]
    async fn test_deminiaturize() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = deminiaturize(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CWrapper$NSWindow.isKeyWindow(J)Z"
    )]
    async fn test_is_key_window() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_key_window(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CWrapper$NSWindow.isZoomed(J)Z"
    )]
    async fn test_is_zoomed() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_zoomed(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CWrapper$NSWindow.makeFirstResponder(JJ)V"
    )]
    async fn test_make_first_responder() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = make_first_responder(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CWrapper$NSWindow.makeKeyAndOrderFront(J)V"
    )]
    async fn test_make_key_and_order_front() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = make_key_and_order_front(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CWrapper$NSWindow.makeKeyWindow(J)V"
    )]
    async fn test_make_key_window() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = make_key_window(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CWrapper$NSWindow.makeMainWindow(J)V"
    )]
    async fn test_make_main_window() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = make_main_window(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CWrapper$NSWindow.miniaturize(J)V"
    )]
    async fn test_miniaturize() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = miniaturize(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CWrapper$NSWindow.orderFront(J)V"
    )]
    async fn test_order_front() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = order_front(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CWrapper$NSWindow.orderFrontRegardless(J)V"
    )]
    async fn test_order_front_regardless() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = order_front_regardless(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CWrapper$NSWindow.orderOut(J)V"
    )]
    async fn test_order_out() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = order_out(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CWrapper$NSWindow.orderWindow(JIJ)V"
    )]
    async fn test_order_window() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = order_window(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CWrapper$NSWindow.removeChildWindow(JJ)V"
    )]
    async fn test_remove_child_window() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = remove_child_window(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CWrapper$NSWindow.setAlphaValue(JF)V"
    )]
    async fn test_set_alpha_value() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_alpha_value(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CWrapper$NSWindow.setBackgroundColor(JI)V"
    )]
    async fn test_set_background_color() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_background_color(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CWrapper$NSWindow.setLevel(JI)V"
    )]
    async fn test_set_level() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_level(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CWrapper$NSWindow.setOpaque(JZ)V"
    )]
    async fn test_set_opaque() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_opaque(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.lwawt.macosx.CWrapper$NSWindow.zoom(J)V")]
    async fn test_zoom() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = zoom(thread, Parameters::default()).await;
    }
}
