use ristretto_classfile::VersionSpecification::{Any, GreaterThanOrEqual, LessThanOrEqual};
use ristretto_classfile::{JAVA_11, JAVA_21};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/awt/windows/WWindowPeer._setResizable(Z)V", Any)]
#[async_method]
pub async fn set_resizable<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg0 = parameters.pop_bool()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WWindowPeer._setResizable(Z)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WWindowPeer._setTitle(Ljava/lang/String;)V", Any)]
#[async_method]
pub async fn set_title<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WWindowPeer._setTitle(Ljava/lang/String;)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WWindowPeer._toFront()V", Any)]
#[async_method]
pub async fn to_front<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WWindowPeer._toFront()V".to_string())
            .into(),
    )
}
#[intrinsic_method(
    "sun/awt/windows/WWindowPeer.createAwtWindow(Lsun/awt/windows/WComponentPeer;)V",
    Any
)]
#[async_method]
pub async fn create_awt_window<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _parent = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WWindowPeer.createAwtWindow(Lsun/awt/windows/WComponentPeer;)V"
            .to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/windows/WWindowPeer.getNativeWindowSize()Ljava/awt/Dimension;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_native_window_size<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WWindowPeer.getNativeWindowSize()Ljava/awt/Dimension;".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WWindowPeer.getScreenImOn()I", Any)]
#[async_method]
pub async fn get_screen_im_on<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WWindowPeer.getScreenImOn()I".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/windows/WWindowPeer.getSysIconHeight()I", Any)]
#[async_method]
pub async fn get_sys_icon_height<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WWindowPeer.getSysIconHeight()I".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WWindowPeer.getSysIconWidth()I", Any)]
#[async_method]
pub async fn get_sys_icon_width<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WWindowPeer.getSysIconWidth()I".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WWindowPeer.getSysMinHeight()I", Any)]
#[async_method]
pub async fn get_sys_min_height<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WWindowPeer.getSysMinHeight()I".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WWindowPeer.getSysMinWidth()I", Any)]
#[async_method]
pub async fn get_sys_min_width<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WWindowPeer.getSysMinWidth()I".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WWindowPeer.getSysSmIconHeight()I", Any)]
#[async_method]
pub async fn get_sys_sm_icon_height<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WWindowPeer.getSysSmIconHeight()I".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WWindowPeer.getSysSmIconWidth()I", Any)]
#[async_method]
pub async fn get_sys_sm_icon_width<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WWindowPeer.getSysSmIconWidth()I".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WWindowPeer.initIDs()V", Any)]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WWindowPeer.initIDs()V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/windows/WWindowPeer.modalDisable(Ljava/awt/Dialog;J)V", Any)]
#[async_method]
pub async fn modal_disable<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _blocker_h_wnd = parameters.pop_long()?;
    let _blocker = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WWindowPeer.modalDisable(Ljava/awt/Dialog;J)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WWindowPeer.modalEnable(Ljava/awt/Dialog;)V", Any)]
#[async_method]
pub async fn modal_enable<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _blocker = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WWindowPeer.modalEnable(Ljava/awt/Dialog;)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WWindowPeer.nativeGrab()V", Any)]
#[async_method]
pub async fn native_grab<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WWindowPeer.nativeGrab()V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/windows/WWindowPeer.nativeUngrab()V", Any)]
#[async_method]
pub async fn native_ungrab<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WWindowPeer.nativeUngrab()V".to_string())
            .into(),
    )
}
#[intrinsic_method(
    "sun/awt/windows/WWindowPeer.repositionSecurityWarning()V",
    LessThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn reposition_security_warning<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WWindowPeer.repositionSecurityWarning()V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WWindowPeer.requestWindowFocus(Z)Z", Any)]
#[async_method]
pub async fn request_window_focus<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _is_mouse_event_cause = parameters.pop_bool()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WWindowPeer.requestWindowFocus(Z)Z".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WWindowPeer.reshapeFrame(IIII)V", Any)]
#[async_method]
pub async fn reshape_frame<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WWindowPeer.reshapeFrame(IIII)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WWindowPeer.setAlwaysOnTopNative(Z)V", Any)]
#[async_method]
pub async fn set_always_on_top_native<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _value = parameters.pop_bool()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WWindowPeer.setAlwaysOnTopNative(Z)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WWindowPeer.setFocusableWindow(Z)V", Any)]
#[async_method]
pub async fn set_focusable_window<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _is_focusable_window = parameters.pop_bool()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WWindowPeer.setFocusableWindow(Z)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WWindowPeer.setFullScreenExclusiveModeState(Z)V", Any)]
#[async_method]
pub async fn set_full_screen_exclusive_mode_state<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _state = parameters.pop_bool()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WWindowPeer.setFullScreenExclusiveModeState(Z)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WWindowPeer.setIconImagesData([III[III)V", Any)]
#[async_method]
pub async fn set_icon_images_data<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _smh = parameters.pop_int()?;
    let _smw = parameters.pop_int()?;
    let _small_icon_raster = parameters.pop_reference()?;
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _icon_raster = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WWindowPeer.setIconImagesData([III[III)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WWindowPeer.setMinSize(II)V", Any)]
#[async_method]
pub async fn set_min_size<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WWindowPeer.setMinSize(II)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/windows/WWindowPeer.setOpacity(I)V", Any)]
#[async_method]
pub async fn set_opacity<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _i_opacity = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WWindowPeer.setOpacity(I)V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/windows/WWindowPeer.setOpaqueImpl(Z)V", Any)]
#[async_method]
pub async fn set_opaque_impl<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _is_opaque = parameters.pop_bool()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WWindowPeer.setOpaqueImpl(Z)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WWindowPeer.toBack()V", Any)]
#[async_method]
pub async fn to_back<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun/awt/windows/WWindowPeer.toBack()V".to_string()).into())
}
#[intrinsic_method("sun/awt/windows/WWindowPeer.updateInsets(Ljava/awt/Insets;)V", Any)]
#[async_method]
pub async fn update_insets<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _insets = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WWindowPeer.updateInsets(Ljava/awt/Insets;)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WWindowPeer.updateWindowImpl([III)V", Any)]
#[async_method]
pub async fn update_window_impl<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _height = parameters.pop_int()?;
    let _width = parameters.pop_int()?;
    let _data = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WWindowPeer.updateWindowImpl([III)V".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_resizable() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_resizable(thread, Parameters::new(vec![Value::from(false)])).await;
        assert_eq!(
            "sun/awt/windows/WWindowPeer._setResizable(Z)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_title() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_title(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/awt/windows/WWindowPeer._setTitle(Ljava/lang/String;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_to_front() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = to_front(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WWindowPeer._toFront()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_create_awt_window() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create_awt_window(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/awt/windows/WWindowPeer.createAwtWindow(Lsun/awt/windows/WComponentPeer;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_native_window_size() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_native_window_size(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WWindowPeer.getNativeWindowSize()Ljava/awt/Dimension;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_screen_im_on() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_screen_im_on(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WWindowPeer.getScreenImOn()I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_sys_icon_height() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_sys_icon_height(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WWindowPeer.getSysIconHeight()I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_sys_icon_width() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_sys_icon_width(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WWindowPeer.getSysIconWidth()I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_sys_min_height() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_sys_min_height(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WWindowPeer.getSysMinHeight()I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_sys_min_width() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_sys_min_width(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WWindowPeer.getSysMinWidth()I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_sys_sm_icon_height() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_sys_sm_icon_height(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WWindowPeer.getSysSmIconHeight()I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_sys_sm_icon_width() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_sys_sm_icon_width(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WWindowPeer.getSysSmIconWidth()I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_init_ids() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ids(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WWindowPeer.initIDs()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_modal_disable() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = modal_disable(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/WWindowPeer.modalDisable(Ljava/awt/Dialog;J)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_modal_enable() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = modal_enable(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/awt/windows/WWindowPeer.modalEnable(Ljava/awt/Dialog;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_native_grab() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_grab(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WWindowPeer.nativeGrab()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_native_ungrab() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_ungrab(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WWindowPeer.nativeUngrab()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_reposition_security_warning() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = reposition_security_warning(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WWindowPeer.repositionSecurityWarning()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_request_window_focus() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = request_window_focus(thread, Parameters::new(vec![Value::from(false)])).await;
        assert_eq!(
            "sun/awt/windows/WWindowPeer.requestWindowFocus(Z)Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_reshape_frame() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = reshape_frame(
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
            "sun/awt/windows/WWindowPeer.reshapeFrame(IIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_always_on_top_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            set_always_on_top_native(thread, Parameters::new(vec![Value::from(false)])).await;
        assert_eq!(
            "sun/awt/windows/WWindowPeer.setAlwaysOnTopNative(Z)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_focusable_window() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_focusable_window(thread, Parameters::new(vec![Value::from(false)])).await;
        assert_eq!(
            "sun/awt/windows/WWindowPeer.setFocusableWindow(Z)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_full_screen_exclusive_mode_state() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            set_full_screen_exclusive_mode_state(thread, Parameters::new(vec![Value::from(false)]))
                .await;
        assert_eq!(
            "sun/awt/windows/WWindowPeer.setFullScreenExclusiveModeState(Z)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_icon_images_data() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_icon_images_data(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/WWindowPeer.setIconImagesData([III[III)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_min_size() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            set_min_size(thread, Parameters::new(vec![Value::Int(0), Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/windows/WWindowPeer.setMinSize(II)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_opacity() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_opacity(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/awt/windows/WWindowPeer.setOpacity(I)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_opaque_impl() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_opaque_impl(thread, Parameters::new(vec![Value::from(false)])).await;
        assert_eq!(
            "sun/awt/windows/WWindowPeer.setOpaqueImpl(Z)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_to_back() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = to_back(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WWindowPeer.toBack()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_update_insets() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = update_insets(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/awt/windows/WWindowPeer.updateInsets(Ljava/awt/Insets;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_update_window_impl() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = update_window_impl(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Int(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/WWindowPeer.updateWindowImpl([III)V",
            result.unwrap_err().to_string()
        );
    }
}
