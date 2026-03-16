use ristretto_classfile::VersionSpecification::{Any, GreaterThanOrEqual};
use ristretto_classfile::{JAVA_11, JAVA_17};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/lwawt/macosx/CPlatformWindow._toggleFullScreenMode(J)V", Any)]
#[async_method]
pub async fn toggle_full_screen_mode<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CPlatformWindow._toggleFullScreenMode(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CPlatformWindow.nativeCreateNSWindow(JJJDDDD)J", Any)]
#[async_method]
pub async fn native_create_ns_window<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CPlatformWindow.nativeCreateNSWindow(JJJDDDD)J".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CPlatformWindow.nativeDispose(J)V", Any)]
#[async_method]
pub async fn native_dispose<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CPlatformWindow.nativeDispose(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CPlatformWindow.nativeEnterFullScreenMode(J)V", Any)]
#[async_method]
pub async fn native_enter_full_screen_mode<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CPlatformWindow.nativeEnterFullScreenMode(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CPlatformWindow.nativeExitFullScreenMode(J)V", Any)]
#[async_method]
pub async fn native_exit_full_screen_mode<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CPlatformWindow.nativeExitFullScreenMode(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/lwawt/macosx/CPlatformWindow.nativeGetNSWindowInsets(J)Ljava/awt/Insets;",
    Any
)]
#[async_method]
pub async fn native_get_ns_window_insets<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CPlatformWindow.nativeGetNSWindowInsets(J)Ljava/awt/Insets;".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/lwawt/macosx/CPlatformWindow.nativeGetTopmostPlatformWindowUnderMouse()Lsun/lwawt/macosx/CPlatformWindow;",
    Any
)]
#[async_method]
pub async fn native_get_topmost_platform_window_under_mouse<
    T: ristretto_types::Thread + 'static,
>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.lwawt.macosx.CPlatformWindow.nativeGetTopmostPlatformWindowUnderMouse()Lsun/lwawt/macosx/CPlatformWindow;".to_string()).into())
}

#[intrinsic_method("sun/lwawt/macosx/CPlatformWindow.nativePushNSWindowToBack(J)V", Any)]
#[async_method]
pub async fn native_push_ns_window_to_back<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CPlatformWindow.nativePushNSWindowToBack(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CPlatformWindow.nativePushNSWindowToFront(J)V", Any)]
#[async_method]
pub async fn native_push_ns_window_to_front<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CPlatformWindow.nativePushNSWindowToFront(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/lwawt/macosx/CPlatformWindow.nativeRevalidateNSWindowShadow(J)V",
    Any
)]
#[async_method]
pub async fn native_revalidate_ns_window_shadow<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CPlatformWindow.nativeRevalidateNSWindowShadow(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/lwawt/macosx/CPlatformWindow.nativeSetAllowAutomaticTabbingProperty(Z)V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn native_set_allow_automatic_tabbing_property<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CPlatformWindow.nativeSetAllowAutomaticTabbingProperty(Z)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CPlatformWindow.nativeSetEnabled(JZ)V", Any)]
#[async_method]
pub async fn native_set_enabled<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CPlatformWindow.nativeSetEnabled(JZ)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/lwawt/macosx/CPlatformWindow.nativeSetNSWindowBounds(JDDDD)V",
    Any
)]
#[async_method]
pub async fn native_set_ns_window_bounds<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CPlatformWindow.nativeSetNSWindowBounds(JDDDD)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/lwawt/macosx/CPlatformWindow.nativeSetNSWindowLocationByPlatform(J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn native_set_ns_window_location_by_platform<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CPlatformWindow.nativeSetNSWindowLocationByPlatform(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CPlatformWindow.nativeSetNSWindowMenuBar(JJ)V", Any)]
#[async_method]
pub async fn native_set_ns_window_menu_bar<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CPlatformWindow.nativeSetNSWindowMenuBar(JJ)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/lwawt/macosx/CPlatformWindow.nativeSetNSWindowMinMax(JDDDD)V",
    Any
)]
#[async_method]
pub async fn native_set_ns_window_min_max<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CPlatformWindow.nativeSetNSWindowMinMax(JDDDD)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/lwawt/macosx/CPlatformWindow.nativeSetNSWindowMinimizedIcon(JJ)V",
    Any
)]
#[async_method]
pub async fn native_set_ns_window_minimized_icon<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CPlatformWindow.nativeSetNSWindowMinimizedIcon(JJ)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/lwawt/macosx/CPlatformWindow.nativeSetNSWindowRepresentedFilename(JLjava/lang/String;)V",
    Any
)]
#[async_method]
pub async fn native_set_ns_window_represented_filename<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.lwawt.macosx.CPlatformWindow.nativeSetNSWindowRepresentedFilename(JLjava/lang/String;)V".to_string()).into())
}

#[intrinsic_method(
    "sun/lwawt/macosx/CPlatformWindow.nativeSetNSWindowStandardFrame(JDDDD)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn native_set_ns_window_standard_frame<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CPlatformWindow.nativeSetNSWindowStandardFrame(JDDDD)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/lwawt/macosx/CPlatformWindow.nativeSetNSWindowStyleBits(JII)V",
    Any
)]
#[async_method]
pub async fn native_set_ns_window_style_bits<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CPlatformWindow.nativeSetNSWindowStyleBits(JII)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/lwawt/macosx/CPlatformWindow.nativeSetNSWindowTitle(JLjava/lang/String;)V",
    Any
)]
#[async_method]
pub async fn native_set_ns_window_title<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CPlatformWindow.nativeSetNSWindowTitle(JLjava/lang/String;)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/lwawt/macosx/CPlatformWindow.nativeSynthesizeMouseEnteredExitedEvents()V",
    Any
)]
#[async_method]
pub async fn native_synthesize_mouse_entered_exited_events_1<
    T: ristretto_types::Thread + 'static,
>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CPlatformWindow.nativeSynthesizeMouseEnteredExitedEvents()V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/lwawt/macosx/CPlatformWindow.nativeSynthesizeMouseEnteredExitedEvents(JI)V",
    Any
)]
#[async_method]
pub async fn native_synthesize_mouse_entered_exited_events_2<
    T: ristretto_types::Thread + 'static,
>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CPlatformWindow.nativeSynthesizeMouseEnteredExitedEvents(JI)V"
            .to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_toggle_full_screen_mode() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = toggle_full_screen_mode(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_create_ns_window() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_create_ns_window(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_dispose() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_dispose(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_enter_full_screen_mode() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_enter_full_screen_mode(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_exit_full_screen_mode() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_exit_full_screen_mode(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_get_ns_window_insets() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_get_ns_window_insets(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_get_topmost_platform_window_under_mouse() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            native_get_topmost_platform_window_under_mouse(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_push_ns_window_to_back() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_push_ns_window_to_back(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_push_ns_window_to_front() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_push_ns_window_to_front(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_revalidate_ns_window_shadow() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_revalidate_ns_window_shadow(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_set_allow_automatic_tabbing_property() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            native_set_allow_automatic_tabbing_property(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_set_enabled() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_set_enabled(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_set_ns_window_bounds() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_set_ns_window_bounds(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_set_ns_window_location_by_platform() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_set_ns_window_location_by_platform(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_set_ns_window_menu_bar() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_set_ns_window_menu_bar(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_set_ns_window_min_max() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_set_ns_window_min_max(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_set_ns_window_minimized_icon() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_set_ns_window_minimized_icon(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_set_ns_window_represented_filename() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_set_ns_window_represented_filename(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn testnative_set_ns_window_standard_frame() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_set_ns_window_standard_frame(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_set_ns_window_style_bits() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_set_ns_window_style_bits(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_set_ns_window_title() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_set_ns_window_title(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_synthesize_mouse_entered_exited_events_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ =
            native_synthesize_mouse_entered_exited_events_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_native_synthesize_mouse_entered_exited_events_2() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ =
            native_synthesize_mouse_entered_exited_events_2(thread, Parameters::default()).await;
    }
}
