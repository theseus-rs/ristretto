use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/awt/windows/WComponentPeer._dispose()V", Any)]
#[async_method]
pub async fn dispose<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WComponentPeer._dispose()V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/windows/WComponentPeer._setBackground(I)V", Any)]
#[async_method]
pub async fn set_background<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg0 = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WComponentPeer._setBackground(I)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WComponentPeer._setFont(Ljava/awt/Font;)V", Any)]
#[async_method]
pub async fn set_font<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WComponentPeer._setFont(Ljava/awt/Font;)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WComponentPeer._setForeground(I)V", Any)]
#[async_method]
pub async fn set_foreground<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg0 = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WComponentPeer._setForeground(I)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WComponentPeer.addNativeDropTarget()J", Any)]
#[async_method]
pub async fn add_native_drop_target<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WComponentPeer.addNativeDropTarget()J".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WComponentPeer.beginValidate()V", Any)]
#[async_method]
pub async fn begin_validate<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WComponentPeer.beginValidate()V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WComponentPeer.createPrintedPixels(IIIII)[I", Any)]
#[async_method]
pub async fn create_printed_pixels<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _alpha = parameters.pop_int()?;
    let _src_h = parameters.pop_int()?;
    let _src_w = parameters.pop_int()?;
    let _src_y = parameters.pop_int()?;
    let _src_x = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WComponentPeer.createPrintedPixels(IIIII)[I".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WComponentPeer.disable()V", Any)]
#[async_method]
pub async fn disable<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WComponentPeer.disable()V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/windows/WComponentPeer.enable()V", Any)]
#[async_method]
pub async fn enable<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WComponentPeer.enable()V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/windows/WComponentPeer.endValidate()V", Any)]
#[async_method]
pub async fn end_validate<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WComponentPeer.endValidate()V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/windows/WComponentPeer.getLocationOnScreen()Ljava/awt/Point;",
    Any
)]
#[async_method]
pub async fn get_location_on_screen<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WComponentPeer.getLocationOnScreen()Ljava/awt/Point;".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WComponentPeer.hide()V", Any)]
#[async_method]
pub async fn hide<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WComponentPeer.hide()V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/windows/WComponentPeer.isObscured()Z", Any)]
#[async_method]
pub async fn is_obscured<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WComponentPeer.isObscured()Z".to_string())
            .into(),
    )
}
#[intrinsic_method(
    "sun/awt/windows/WComponentPeer.nativeHandleEvent(Ljava/awt/AWTEvent;)V",
    Any
)]
#[async_method]
pub async fn native_handle_event<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _event = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WComponentPeer.nativeHandleEvent(Ljava/awt/AWTEvent;)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WComponentPeer.nativeHandlesWheelScrolling()Z", Any)]
#[async_method]
pub async fn native_handles_wheel_scrolling<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WComponentPeer.nativeHandlesWheelScrolling()Z".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/windows/WComponentPeer.pSetParent(Ljava/awt/peer/ComponentPeer;)V",
    Any
)]
#[async_method]
pub async fn p_set_parent<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _parent = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WComponentPeer.pSetParent(Ljava/awt/peer/ComponentPeer;)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WComponentPeer.pShow()V", Any)]
#[async_method]
pub async fn p_show<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WComponentPeer.pShow()V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/windows/WComponentPeer.removeNativeDropTarget()V", Any)]
#[async_method]
pub async fn remove_native_drop_target<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WComponentPeer.removeNativeDropTarget()V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WComponentPeer.reshape(IIII)V", Any)]
#[async_method]
pub async fn reshape<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WComponentPeer.reshape(IIII)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WComponentPeer.reshapeNoCheck(IIII)V", Any)]
#[async_method]
pub async fn reshape_no_check<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WComponentPeer.reshapeNoCheck(IIII)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WComponentPeer.setFocus(Z)V", Any)]
#[async_method]
pub async fn set_focus<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _do_set_focus = parameters.pop_bool()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WComponentPeer.setFocus(Z)V".to_string())
            .into(),
    )
}
#[intrinsic_method(
    "sun/awt/windows/WComponentPeer.setRectangularShape(IIIILsun/java2d/pipe/Region;)V",
    Any
)]
#[async_method]
pub async fn set_rectangular_shape<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _region = parameters.pop_reference()?;
    let _y2 = parameters.pop_int()?;
    let _x2 = parameters.pop_int()?;
    let _y1 = parameters.pop_int()?;
    let _x1 = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WComponentPeer.setRectangularShape(IIIILsun/java2d/pipe/Region;)V"
            .to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WComponentPeer.setZOrder(J)V", Any)]
#[async_method]
pub async fn set_zorder<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _above = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WComponentPeer.setZOrder(J)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/windows/WComponentPeer.start()V", Any)]
#[async_method]
pub async fn start<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WComponentPeer.start()V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/windows/WComponentPeer.updateWindow()V", Any)]
#[async_method]
pub async fn update_window<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WComponentPeer.updateWindow()V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_dispose() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = dispose(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WComponentPeer._dispose()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_background() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_background(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/windows/WComponentPeer._setBackground(I)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_font() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_font(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/awt/windows/WComponentPeer._setFont(Ljava/awt/Font;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_foreground() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_foreground(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/windows/WComponentPeer._setForeground(I)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_add_native_drop_target() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = add_native_drop_target(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WComponentPeer.addNativeDropTarget()J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_begin_validate() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = begin_validate(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WComponentPeer.beginValidate()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_create_printed_pixels() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create_printed_pixels(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/WComponentPeer.createPrintedPixels(IIIII)[I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_disable() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = disable(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WComponentPeer.disable()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_enable() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = enable(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WComponentPeer.enable()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_end_validate() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = end_validate(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WComponentPeer.endValidate()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_location_on_screen() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_location_on_screen(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WComponentPeer.getLocationOnScreen()Ljava/awt/Point;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_hide() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = hide(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WComponentPeer.hide()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_is_obscured() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = is_obscured(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WComponentPeer.isObscured()Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_native_handle_event() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_handle_event(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/awt/windows/WComponentPeer.nativeHandleEvent(Ljava/awt/AWTEvent;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_native_handles_wheel_scrolling() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_handles_wheel_scrolling(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WComponentPeer.nativeHandlesWheelScrolling()Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_p_set_parent() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = p_set_parent(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/awt/windows/WComponentPeer.pSetParent(Ljava/awt/peer/ComponentPeer;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_p_show() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = p_show(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WComponentPeer.pShow()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_remove_native_drop_target() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = remove_native_drop_target(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WComponentPeer.removeNativeDropTarget()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_reshape() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = reshape(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/WComponentPeer.reshape(IIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_reshape_no_check() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = reshape_no_check(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/WComponentPeer.reshapeNoCheck(IIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_focus() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_focus(thread, Parameters::new(vec![Value::from(false)])).await;
        assert_eq!(
            "sun/awt/windows/WComponentPeer.setFocus(Z)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_rectangular_shape() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_rectangular_shape(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/WComponentPeer.setRectangularShape(IIIILsun/java2d/pipe/Region;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_zorder() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_zorder(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/windows/WComponentPeer.setZOrder(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_start() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = start(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WComponentPeer.start()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_update_window() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = update_window(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WComponentPeer.updateWindow()V",
            result.unwrap_err().to_string()
        );
    }
}
