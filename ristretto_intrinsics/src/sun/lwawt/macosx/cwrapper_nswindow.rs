use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/lwawt/macosx/CWrapper$NSWindow.addChildWindow(JJI)V", Any)]
#[async_method]
pub async fn add_child_window<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSWindow.addChildWindow(JJI)V")
}

#[intrinsic_method("sun/lwawt/macosx/CWrapper$NSWindow.canBecomeMainWindow(J)Z", Any)]
#[async_method]
pub async fn can_become_main_window<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSWindow.canBecomeMainWindow(J)Z")
}

#[intrinsic_method("sun/lwawt/macosx/CWrapper$NSWindow.close(J)V", Any)]
#[async_method]
pub async fn close<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSWindow.close(J)V")
}

#[intrinsic_method("sun/lwawt/macosx/CWrapper$NSWindow.deminiaturize(J)V", Any)]
#[async_method]
pub async fn deminiaturize<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSWindow.deminiaturize(J)V")
}

#[intrinsic_method("sun/lwawt/macosx/CWrapper$NSWindow.isKeyWindow(J)Z", Any)]
#[async_method]
pub async fn is_key_window<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSWindow.isKeyWindow(J)Z")
}

#[intrinsic_method("sun/lwawt/macosx/CWrapper$NSWindow.isZoomed(J)Z", Any)]
#[async_method]
pub async fn is_zoomed<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSWindow.isZoomed(J)Z")
}

#[intrinsic_method("sun/lwawt/macosx/CWrapper$NSWindow.makeFirstResponder(JJ)V", Any)]
#[async_method]
pub async fn make_first_responder<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSWindow.makeFirstResponder(JJ)V")
}

#[intrinsic_method("sun/lwawt/macosx/CWrapper$NSWindow.makeKeyAndOrderFront(J)V", Any)]
#[async_method]
pub async fn make_key_and_order_front<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSWindow.makeKeyAndOrderFront(J)V")
}

#[intrinsic_method("sun/lwawt/macosx/CWrapper$NSWindow.makeKeyWindow(J)V", Any)]
#[async_method]
pub async fn make_key_window<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSWindow.makeKeyWindow(J)V")
}

#[intrinsic_method("sun/lwawt/macosx/CWrapper$NSWindow.makeMainWindow(J)V", Any)]
#[async_method]
pub async fn make_main_window<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSWindow.makeMainWindow(J)V")
}

#[intrinsic_method("sun/lwawt/macosx/CWrapper$NSWindow.miniaturize(J)V", Any)]
#[async_method]
pub async fn miniaturize<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSWindow.miniaturize(J)V")
}

#[intrinsic_method("sun/lwawt/macosx/CWrapper$NSWindow.orderFront(J)V", Any)]
#[async_method]
pub async fn order_front<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSWindow.orderFront(J)V")
}

#[intrinsic_method("sun/lwawt/macosx/CWrapper$NSWindow.orderFrontRegardless(J)V", Any)]
#[async_method]
pub async fn order_front_regardless<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSWindow.orderFrontRegardless(J)V")
}

#[intrinsic_method("sun/lwawt/macosx/CWrapper$NSWindow.orderOut(J)V", Any)]
#[async_method]
pub async fn order_out<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSWindow.orderOut(J)V")
}

#[intrinsic_method("sun/lwawt/macosx/CWrapper$NSWindow.orderWindow(JIJ)V", Any)]
#[async_method]
pub async fn order_window<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSWindow.orderWindow(JIJ)V")
}

#[intrinsic_method("sun/lwawt/macosx/CWrapper$NSWindow.removeChildWindow(JJ)V", Any)]
#[async_method]
pub async fn remove_child_window<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSWindow.removeChildWindow(JJ)V")
}

#[intrinsic_method("sun/lwawt/macosx/CWrapper$NSWindow.setAlphaValue(JF)V", Any)]
#[async_method]
pub async fn set_alpha_value<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSWindow.setAlphaValue(JF)V")
}

#[intrinsic_method("sun/lwawt/macosx/CWrapper$NSWindow.setBackgroundColor(JI)V", Any)]
#[async_method]
pub async fn set_background_color<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSWindow.setBackgroundColor(JI)V")
}

#[intrinsic_method("sun/lwawt/macosx/CWrapper$NSWindow.setLevel(JI)V", Any)]
#[async_method]
pub async fn set_level<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSWindow.setLevel(JI)V")
}

#[intrinsic_method("sun/lwawt/macosx/CWrapper$NSWindow.setOpaque(JZ)V", Any)]
#[async_method]
pub async fn set_opaque<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSWindow.setOpaque(JZ)V")
}

#[intrinsic_method("sun/lwawt/macosx/CWrapper$NSWindow.zoom(J)V", Any)]
#[async_method]
pub async fn zoom<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
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
