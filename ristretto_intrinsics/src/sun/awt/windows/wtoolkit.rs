use ristretto_classfile::VersionSpecification::{Any, Equal, LessThanOrEqual};
use ristretto_classfile::{JAVA_8, JAVA_11, JAVA_21};
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/awt/windows/WToolkit.beep()V", Any)]
pub fn beep<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun/awt/windows/WToolkit.beep()V".to_string()).into())
}
#[intrinsic_method(
    "sun/awt/windows/WToolkit.disableCustomPalette()V",
    LessThanOrEqual(JAVA_11)
)]
pub fn disable_custom_palette<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WToolkit.disableCustomPalette()V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/windows/WToolkit.embeddedDispose()Z",
    LessThanOrEqual(JAVA_21)
)]
pub fn embedded_dispose<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WToolkit.embeddedDispose()Z".to_string())
            .into(),
    )
}
#[intrinsic_method(
    "sun/awt/windows/WToolkit.embeddedEventLoopIdleProcessing()V",
    LessThanOrEqual(JAVA_21)
)]
pub fn embedded_event_loop_idle_processing<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WToolkit.embeddedEventLoopIdleProcessing()V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WToolkit.embeddedInit()Z", LessThanOrEqual(JAVA_21))]
pub fn embedded_init<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WToolkit.embeddedInit()Z".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/windows/WToolkit.eventLoop()V", Any)]
pub fn event_loop<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun/awt/windows/WToolkit.eventLoop()V".to_string()).into())
}
#[intrinsic_method("sun/awt/windows/WToolkit.getLockingKeyStateNative(I)Z", Any)]
pub fn get_locking_key_state_native<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _java_key = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WToolkit.getLockingKeyStateNative(I)Z".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WToolkit.getMaximumCursorColors()I", Any)]
pub fn get_maximum_cursor_colors<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WToolkit.getMaximumCursorColors()I".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WToolkit.getNumberOfButtonsImpl()I", Any)]
pub fn get_number_of_buttons_impl<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WToolkit.getNumberOfButtonsImpl()I".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WToolkit.getScreenHeight()I", Equal(JAVA_8))]
pub fn get_screen_height<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WToolkit.getScreenHeight()I".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/windows/WToolkit.getScreenInsets(I)Ljava/awt/Insets;", Any)]
pub fn get_screen_insets<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _screen = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WToolkit.getScreenInsets(I)Ljava/awt/Insets;".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WToolkit.getScreenWidth()I", Equal(JAVA_8))]
pub fn get_screen_width<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WToolkit.getScreenWidth()I".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/windows/WToolkit.getWindowsVersion()Ljava/lang/String;", Any)]
pub fn get_windows_version<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WToolkit.getWindowsVersion()Ljava/lang/String;".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WToolkit.hideTouchKeyboard()V", Any)]
pub fn hide_touch_keyboard<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WToolkit.hideTouchKeyboard()V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WToolkit.init()Z", Any)]
pub fn init<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun/awt/windows/WToolkit.init()Z".to_string()).into())
}
#[intrinsic_method("sun/awt/windows/WToolkit.initIDs()V", Any)]
pub fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun/awt/windows/WToolkit.initIDs()V".to_string()).into())
}
#[intrinsic_method("sun/awt/windows/WToolkit.isDynamicLayoutSupportedNative()Z", Any)]
pub fn is_dynamic_layout_supported_native<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WToolkit.isDynamicLayoutSupportedNative()Z".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WToolkit.loadSystemColors([I)V", Any)]
pub fn load_system_colors<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _colors = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WToolkit.loadSystemColors([I)V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/windows/WToolkit.makeColorModel()Ljava/awt/image/ColorModel;",
    Any
)]
pub fn make_color_model<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WToolkit.makeColorModel()Ljava/awt/image/ColorModel;".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WToolkit.nativeSync()V", Any)]
pub fn native_sync<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WToolkit.nativeSync()V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/windows/WToolkit.postDispose()V", Any)]
pub fn post_dispose<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WToolkit.postDispose()V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/windows/WToolkit.quitSecondaryEventLoop()V", Any)]
pub fn quit_secondary_event_loop<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WToolkit.quitSecondaryEventLoop()V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WToolkit.setDynamicLayoutNative(Z)V", Any)]
pub fn set_dynamic_layout_native<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _dynamic = parameters.pop_bool()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WToolkit.setDynamicLayoutNative(Z)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WToolkit.setExtraMouseButtonsEnabledNative(Z)V", Any)]
pub fn set_extra_mouse_buttons_enabled_native<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _enable = parameters.pop_bool()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WToolkit.setExtraMouseButtonsEnabledNative(Z)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WToolkit.setLockingKeyStateNative(IZ)V", Any)]
pub fn set_locking_key_state_native<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _state = parameters.pop_bool()?;
    let _java_key = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WToolkit.setLockingKeyStateNative(IZ)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WToolkit.showTouchKeyboard(Z)V", Any)]
pub fn show_touch_keyboard<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _caused_by_touch_event = parameters.pop_bool()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WToolkit.showTouchKeyboard(Z)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WToolkit.shutdown()V", Any)]
pub fn shutdown<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun/awt/windows/WToolkit.shutdown()V".to_string()).into())
}
#[intrinsic_method("sun/awt/windows/WToolkit.startSecondaryEventLoop()V", Any)]
pub fn start_secondary_event_loop<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WToolkit.startSecondaryEventLoop()V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/windows/WToolkit.startToolkitThread(Ljava/lang/Runnable;Ljava/lang/ThreadGroup;)Z",
    Any
)]
pub fn start_toolkit_thread<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _thread_group = parameters.pop_reference()?;
    let _thread = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WToolkit.startToolkitThread(Ljava/lang/Runnable;Ljava/lang/ThreadGroup;)Z"
            .to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WToolkit.syncNativeQueue(J)Z", Any)]
pub fn sync_native_queue<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _timeout = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WToolkit.syncNativeQueue(J)Z".to_string())
            .into(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_beep() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = beep(thread, Parameters::default());
        assert_eq!(
            "sun/awt/windows/WToolkit.beep()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_disable_custom_palette() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = disable_custom_palette(thread, Parameters::default());
        assert_eq!(
            "sun/awt/windows/WToolkit.disableCustomPalette()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_embedded_dispose() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = embedded_dispose(thread, Parameters::default());
        assert_eq!(
            "sun/awt/windows/WToolkit.embeddedDispose()Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_embedded_event_loop_idle_processing() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = embedded_event_loop_idle_processing(thread, Parameters::default());
        assert_eq!(
            "sun/awt/windows/WToolkit.embeddedEventLoopIdleProcessing()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_embedded_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = embedded_init(thread, Parameters::default());
        assert_eq!(
            "sun/awt/windows/WToolkit.embeddedInit()Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_event_loop() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = event_loop(thread, Parameters::default());
        assert_eq!(
            "sun/awt/windows/WToolkit.eventLoop()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_locking_key_state_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_locking_key_state_native(thread, Parameters::new(vec![Value::Int(0)]));
        assert_eq!(
            "sun/awt/windows/WToolkit.getLockingKeyStateNative(I)Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_maximum_cursor_colors() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_maximum_cursor_colors(thread, Parameters::default());
        assert_eq!(
            "sun/awt/windows/WToolkit.getMaximumCursorColors()I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_number_of_buttons_impl() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_number_of_buttons_impl(thread, Parameters::default());
        assert_eq!(
            "sun/awt/windows/WToolkit.getNumberOfButtonsImpl()I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_screen_height() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_screen_height(thread, Parameters::default());
        assert_eq!(
            "sun/awt/windows/WToolkit.getScreenHeight()I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_screen_insets() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_screen_insets(thread, Parameters::new(vec![Value::Int(0)]));
        assert_eq!(
            "sun/awt/windows/WToolkit.getScreenInsets(I)Ljava/awt/Insets;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_screen_width() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_screen_width(thread, Parameters::default());
        assert_eq!(
            "sun/awt/windows/WToolkit.getScreenWidth()I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_windows_version() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_windows_version(thread, Parameters::default());
        assert_eq!(
            "sun/awt/windows/WToolkit.getWindowsVersion()Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_hide_touch_keyboard() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = hide_touch_keyboard(thread, Parameters::default());
        assert_eq!(
            "sun/awt/windows/WToolkit.hideTouchKeyboard()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init(thread, Parameters::default());
        assert_eq!(
            "sun/awt/windows/WToolkit.init()Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_init_ids() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ids(thread, Parameters::default());
        assert_eq!(
            "sun/awt/windows/WToolkit.initIDs()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_is_dynamic_layout_supported_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = is_dynamic_layout_supported_native(thread, Parameters::default());
        assert_eq!(
            "sun/awt/windows/WToolkit.isDynamicLayoutSupportedNative()Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_load_system_colors() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = load_system_colors(thread, Parameters::new(vec![Value::Object(None)]));
        assert_eq!(
            "sun/awt/windows/WToolkit.loadSystemColors([I)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_make_color_model() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = make_color_model(thread, Parameters::default());
        assert_eq!(
            "sun/awt/windows/WToolkit.makeColorModel()Ljava/awt/image/ColorModel;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_native_sync() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_sync(thread, Parameters::default());
        assert_eq!(
            "sun/awt/windows/WToolkit.nativeSync()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_post_dispose() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = post_dispose(thread, Parameters::default());
        assert_eq!(
            "sun/awt/windows/WToolkit.postDispose()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_quit_secondary_event_loop() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = quit_secondary_event_loop(thread, Parameters::default());
        assert_eq!(
            "sun/awt/windows/WToolkit.quitSecondaryEventLoop()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_dynamic_layout_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_dynamic_layout_native(thread, Parameters::new(vec![Value::from(false)]));
        assert_eq!(
            "sun/awt/windows/WToolkit.setDynamicLayoutNative(Z)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_extra_mouse_buttons_enabled_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_extra_mouse_buttons_enabled_native(
            thread,
            Parameters::new(vec![Value::from(false)]),
        );
        assert_eq!(
            "sun/awt/windows/WToolkit.setExtraMouseButtonsEnabledNative(Z)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_locking_key_state_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_locking_key_state_native(
            thread,
            Parameters::new(vec![Value::Int(0), Value::from(false)]),
        );
        assert_eq!(
            "sun/awt/windows/WToolkit.setLockingKeyStateNative(IZ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_show_touch_keyboard() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = show_touch_keyboard(thread, Parameters::new(vec![Value::from(false)]));
        assert_eq!(
            "sun/awt/windows/WToolkit.showTouchKeyboard(Z)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_shutdown() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = shutdown(thread, Parameters::default());
        assert_eq!(
            "sun/awt/windows/WToolkit.shutdown()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_start_secondary_event_loop() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = start_secondary_event_loop(thread, Parameters::default());
        assert_eq!(
            "sun/awt/windows/WToolkit.startSecondaryEventLoop()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_start_toolkit_thread() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = start_toolkit_thread(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        );
        assert_eq!(
            "sun/awt/windows/WToolkit.startToolkitThread(Ljava/lang/Runnable;Ljava/lang/ThreadGroup;)Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_sync_native_queue() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = sync_native_queue(thread, Parameters::new(vec![Value::Long(0)]));
        assert_eq!(
            "sun/awt/windows/WToolkit.syncNativeQueue(J)Z",
            result.unwrap_err().to_string()
        );
    }
}
