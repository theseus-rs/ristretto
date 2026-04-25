use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/lwawt/macosx/CWrapper$NSWindow.addChildWindow(JJI)V", Any)]
#[async_method]
pub async fn add_child_window<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _order = parameters.pop_int()?;
    let _child_ptr = parameters.pop_long()?;
    let _parent_ptr = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CWrapper$NSWindow.addChildWindow(JJI)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CWrapper$NSWindow.canBecomeMainWindow(J)Z", Any)]
#[async_method]
pub async fn can_become_main_window<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _window_ptr = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CWrapper$NSWindow.canBecomeMainWindow(J)Z".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CWrapper$NSWindow.close(J)V", Any)]
#[async_method]
pub async fn close<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _window_ptr = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun.lwawt.macosx.CWrapper$NSWindow.close(J)V".to_string())
            .into(),
    )
}

#[intrinsic_method("sun/lwawt/macosx/CWrapper$NSWindow.deminiaturize(J)V", Any)]
#[async_method]
pub async fn deminiaturize<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _window_ptr = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CWrapper$NSWindow.deminiaturize(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CWrapper$NSWindow.isKeyWindow(J)Z", Any)]
#[async_method]
pub async fn is_key_window<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _window_ptr = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CWrapper$NSWindow.isKeyWindow(J)Z".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CWrapper$NSWindow.isZoomed(J)Z", Any)]
#[async_method]
pub async fn is_zoomed<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _window_ptr = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CWrapper$NSWindow.isZoomed(J)Z".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CWrapper$NSWindow.makeFirstResponder(JJ)V", Any)]
#[async_method]
pub async fn make_first_responder<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _responder_ptr = parameters.pop_long()?;
    let _window_ptr = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CWrapper$NSWindow.makeFirstResponder(JJ)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CWrapper$NSWindow.makeKeyAndOrderFront(J)V", Any)]
#[async_method]
pub async fn make_key_and_order_front<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _window_ptr = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CWrapper$NSWindow.makeKeyAndOrderFront(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CWrapper$NSWindow.makeKeyWindow(J)V", Any)]
#[async_method]
pub async fn make_key_window<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _window_ptr = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CWrapper$NSWindow.makeKeyWindow(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CWrapper$NSWindow.makeMainWindow(J)V", Any)]
#[async_method]
pub async fn make_main_window<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _window_ptr = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CWrapper$NSWindow.makeMainWindow(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CWrapper$NSWindow.miniaturize(J)V", Any)]
#[async_method]
pub async fn miniaturize<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _window_ptr = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CWrapper$NSWindow.miniaturize(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CWrapper$NSWindow.orderFront(J)V", Any)]
#[async_method]
pub async fn order_front<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _window_ptr = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CWrapper$NSWindow.orderFront(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CWrapper$NSWindow.orderFrontRegardless(J)V", Any)]
#[async_method]
pub async fn order_front_regardless<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _window_ptr = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CWrapper$NSWindow.orderFrontRegardless(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CWrapper$NSWindow.orderOut(J)V", Any)]
#[async_method]
pub async fn order_out<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _window_ptr = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CWrapper$NSWindow.orderOut(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CWrapper$NSWindow.orderWindow(JIJ)V", Any)]
#[async_method]
pub async fn order_window<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _relative_to_ptr = parameters.pop_long()?;
    let _order = parameters.pop_int()?;
    let _window_ptr = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CWrapper$NSWindow.orderWindow(JIJ)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CWrapper$NSWindow.removeChildWindow(JJ)V", Any)]
#[async_method]
pub async fn remove_child_window<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _child_ptr = parameters.pop_long()?;
    let _parent_ptr = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CWrapper$NSWindow.removeChildWindow(JJ)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CWrapper$NSWindow.setAlphaValue(JF)V", Any)]
#[async_method]
pub async fn set_alpha_value<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _alpha = parameters.pop_float()?;
    let _window_ptr = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CWrapper$NSWindow.setAlphaValue(JF)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CWrapper$NSWindow.setBackgroundColor(JI)V", Any)]
#[async_method]
pub async fn set_background_color<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _rgb = parameters.pop_int()?;
    let _window_ptr = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CWrapper$NSWindow.setBackgroundColor(JI)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CWrapper$NSWindow.setLevel(JI)V", Any)]
#[async_method]
pub async fn set_level<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _level = parameters.pop_int()?;
    let _window_ptr = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CWrapper$NSWindow.setLevel(JI)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CWrapper$NSWindow.setOpaque(JZ)V", Any)]
#[async_method]
pub async fn set_opaque<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _opaque = parameters.pop_bool()?;
    let _window_ptr = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CWrapper$NSWindow.setOpaque(JZ)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CWrapper$NSWindow.zoom(J)V", Any)]
#[async_method]
pub async fn zoom<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _window_ptr = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun.lwawt.macosx.CWrapper$NSWindow.zoom(J)V".to_string())
            .into(),
    )
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_child_window() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = add_child_window(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun.lwawt.macosx.CWrapper$NSWindow.addChildWindow(JJI)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_can_become_main_window() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = can_become_main_window(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.lwawt.macosx.CWrapper$NSWindow.canBecomeMainWindow(J)Z",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_close() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = close(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.lwawt.macosx.CWrapper$NSWindow.close(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_deminiaturize() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = deminiaturize(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.lwawt.macosx.CWrapper$NSWindow.deminiaturize(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_is_key_window() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = is_key_window(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.lwawt.macosx.CWrapper$NSWindow.isKeyWindow(J)Z",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_is_zoomed() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = is_zoomed(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.lwawt.macosx.CWrapper$NSWindow.isZoomed(J)Z",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_make_first_responder() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = make_first_responder(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun.lwawt.macosx.CWrapper$NSWindow.makeFirstResponder(JJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_make_key_and_order_front() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = make_key_and_order_front(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.lwawt.macosx.CWrapper$NSWindow.makeKeyAndOrderFront(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_make_key_window() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = make_key_window(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.lwawt.macosx.CWrapper$NSWindow.makeKeyWindow(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_make_main_window() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = make_main_window(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.lwawt.macosx.CWrapper$NSWindow.makeMainWindow(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_miniaturize() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = miniaturize(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.lwawt.macosx.CWrapper$NSWindow.miniaturize(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_order_front() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = order_front(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.lwawt.macosx.CWrapper$NSWindow.orderFront(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_order_front_regardless() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = order_front_regardless(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.lwawt.macosx.CWrapper$NSWindow.orderFrontRegardless(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_order_out() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = order_out(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.lwawt.macosx.CWrapper$NSWindow.orderOut(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_order_window() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = order_window(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Int(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun.lwawt.macosx.CWrapper$NSWindow.orderWindow(JIJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_remove_child_window() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = remove_child_window(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun.lwawt.macosx.CWrapper$NSWindow.removeChildWindow(JJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_set_alpha_value() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_alpha_value(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Float(0.0)]),
        )
        .await;
        assert_eq!(
            "sun.lwawt.macosx.CWrapper$NSWindow.setAlphaValue(JF)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_set_background_color() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            set_background_color(thread, Parameters::new(vec![Value::Long(0), Value::Int(0)]))
                .await;
        assert_eq!(
            "sun.lwawt.macosx.CWrapper$NSWindow.setBackgroundColor(JI)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_set_level() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_level(thread, Parameters::new(vec![Value::Long(0), Value::Int(0)])).await;
        assert_eq!(
            "sun.lwawt.macosx.CWrapper$NSWindow.setLevel(JI)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_set_opaque() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_opaque(
            thread,
            Parameters::new(vec![Value::Long(0), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "sun.lwawt.macosx.CWrapper$NSWindow.setOpaque(JZ)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_zoom() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = zoom(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.lwawt.macosx.CWrapper$NSWindow.zoom(J)V",
            result.unwrap_err().to_string()
        );
    }
}
