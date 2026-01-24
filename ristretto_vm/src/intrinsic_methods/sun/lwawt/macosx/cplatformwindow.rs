use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::VersionSpecification::{Any, GreaterThanOrEqual};
use ristretto_classfile::{JAVA_11, JAVA_17};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("sun/lwawt/macosx/CPlatformWindow._toggleFullScreenMode(J)V", Any)]
#[async_method]
pub(crate) async fn toggle_full_screen_mode(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow._toggleFullScreenMode(J)V")
}

#[intrinsic_method("sun/lwawt/macosx/CPlatformWindow.nativeCreateNSWindow(JJJDDDD)J", Any)]
#[async_method]
pub(crate) async fn native_create_ns_window(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow.nativeCreateNSWindow(JJJDDDD)J")
}

#[intrinsic_method("sun/lwawt/macosx/CPlatformWindow.nativeDispose(J)V", Any)]
#[async_method]
pub(crate) async fn native_dispose(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow.nativeDispose(J)V")
}

#[intrinsic_method("sun/lwawt/macosx/CPlatformWindow.nativeEnterFullScreenMode(J)V", Any)]
#[async_method]
pub(crate) async fn native_enter_full_screen_mode(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow.nativeEnterFullScreenMode(J)V")
}

#[intrinsic_method("sun/lwawt/macosx/CPlatformWindow.nativeExitFullScreenMode(J)V", Any)]
#[async_method]
pub(crate) async fn native_exit_full_screen_mode(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow.nativeExitFullScreenMode(J)V")
}

#[intrinsic_method(
    "sun/lwawt/macosx/CPlatformWindow.nativeGetNSWindowInsets(J)Ljava/awt/Insets;",
    Any
)]
#[async_method]
pub(crate) async fn native_get_ns_window_insets(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow.nativeGetNSWindowInsets(J)Ljava/awt/Insets;")
}

#[intrinsic_method(
    "sun/lwawt/macosx/CPlatformWindow.nativeGetTopmostPlatformWindowUnderMouse()Lsun/lwawt/macosx/CPlatformWindow;",
    Any
)]
#[async_method]
pub(crate) async fn native_get_topmost_platform_window_under_mouse(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.lwawt.macosx.CPlatformWindow.nativeGetTopmostPlatformWindowUnderMouse()Lsun/lwawt/macosx/CPlatformWindow;"
    )
}

#[intrinsic_method("sun/lwawt/macosx/CPlatformWindow.nativePushNSWindowToBack(J)V", Any)]
#[async_method]
pub(crate) async fn native_push_ns_window_to_back(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow.nativePushNSWindowToBack(J)V")
}

#[intrinsic_method("sun/lwawt/macosx/CPlatformWindow.nativePushNSWindowToFront(J)V", Any)]
#[async_method]
pub(crate) async fn native_push_ns_window_to_front(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow.nativePushNSWindowToFront(J)V")
}

#[intrinsic_method(
    "sun/lwawt/macosx/CPlatformWindow.nativeRevalidateNSWindowShadow(J)V",
    Any
)]
#[async_method]
pub(crate) async fn native_revalidate_ns_window_shadow(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow.nativeRevalidateNSWindowShadow(J)V")
}

#[intrinsic_method(
    "sun/lwawt/macosx/CPlatformWindow.nativeSetAllowAutomaticTabbingProperty(Z)V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub(crate) async fn native_set_allow_automatic_tabbing_property(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow.nativeSetAllowAutomaticTabbingProperty(Z)V")
}

#[intrinsic_method("sun/lwawt/macosx/CPlatformWindow.nativeSetEnabled(JZ)V", Any)]
#[async_method]
pub(crate) async fn native_set_enabled(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow.nativeSetEnabled(JZ)V")
}

#[intrinsic_method(
    "sun/lwawt/macosx/CPlatformWindow.nativeSetNSWindowBounds(JDDDD)V",
    Any
)]
#[async_method]
pub(crate) async fn native_set_ns_window_bounds(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow.nativeSetNSWindowBounds(JDDDD)V")
}

#[intrinsic_method(
    "sun/lwawt/macosx/CPlatformWindow.nativeSetNSWindowLocationByPlatform(J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn native_set_ns_window_location_by_platform(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow.nativeSetNSWindowLocationByPlatform(J)V")
}

#[intrinsic_method("sun/lwawt/macosx/CPlatformWindow.nativeSetNSWindowMenuBar(JJ)V", Any)]
#[async_method]
pub(crate) async fn native_set_ns_window_menu_bar(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow.nativeSetNSWindowMenuBar(JJ)V")
}

#[intrinsic_method(
    "sun/lwawt/macosx/CPlatformWindow.nativeSetNSWindowMinMax(JDDDD)V",
    Any
)]
#[async_method]
pub(crate) async fn native_set_ns_window_min_max(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow.nativeSetNSWindowMinMax(JDDDD)V")
}

#[intrinsic_method(
    "sun/lwawt/macosx/CPlatformWindow.nativeSetNSWindowMinimizedIcon(JJ)V",
    Any
)]
#[async_method]
pub(crate) async fn native_set_ns_window_minimized_icon(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow.nativeSetNSWindowMinimizedIcon(JJ)V")
}

#[intrinsic_method(
    "sun/lwawt/macosx/CPlatformWindow.nativeSetNSWindowRepresentedFilename(JLjava/lang/String;)V",
    Any
)]
#[async_method]
pub(crate) async fn native_set_ns_window_represented_filename(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.lwawt.macosx.CPlatformWindow.nativeSetNSWindowRepresentedFilename(JLjava/lang/String;)V"
    )
}

#[intrinsic_method(
    "sun/lwawt/macosx/CPlatformWindow.nativeSetNSWindowStandardFrame(JDDDD)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn native_set_ns_window_standard_frame(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow.nativeSetNSWindowStandardFrame(JDDDD)V")
}

#[intrinsic_method(
    "sun/lwawt/macosx/CPlatformWindow.nativeSetNSWindowStyleBits(JII)V",
    Any
)]
#[async_method]
pub(crate) async fn native_set_ns_window_style_bits(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow.nativeSetNSWindowStyleBits(JII)V")
}

#[intrinsic_method(
    "sun/lwawt/macosx/CPlatformWindow.nativeSetNSWindowTitle(JLjava/lang/String;)V",
    Any
)]
#[async_method]
pub(crate) async fn native_set_ns_window_title(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow.nativeSetNSWindowTitle(JLjava/lang/String;)V")
}

#[intrinsic_method(
    "sun/lwawt/macosx/CPlatformWindow.nativeSynthesizeMouseEnteredExitedEvents()V",
    Any
)]
#[async_method]
pub(crate) async fn native_synthesize_mouse_entered_exited_events_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow.nativeSynthesizeMouseEnteredExitedEvents()V")
}
#[intrinsic_method(
    "sun/lwawt/macosx/CPlatformWindow.nativeSynthesizeMouseEnteredExitedEvents(JI)V",
    Any
)]
#[async_method]
pub(crate) async fn native_synthesize_mouse_entered_exited_events_2(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow.nativeSynthesizeMouseEnteredExitedEvents(JI)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CPlatformWindow._toggleFullScreenMode(J)V"
    )]
    async fn test_toggle_full_screen_mode() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = toggle_full_screen_mode(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CPlatformWindow.nativeCreateNSWindow(JJJDDDD)J"
    )]
    async fn test_native_create_ns_window() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_create_ns_window(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CPlatformWindow.nativeDispose(J)V"
    )]
    async fn test_native_dispose() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_dispose(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CPlatformWindow.nativeEnterFullScreenMode(J)V"
    )]
    async fn test_native_enter_full_screen_mode() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_enter_full_screen_mode(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CPlatformWindow.nativeExitFullScreenMode(J)V"
    )]
    async fn test_native_exit_full_screen_mode() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_exit_full_screen_mode(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CPlatformWindow.nativeGetNSWindowInsets(J)Ljava/awt/Insets;"
    )]
    async fn test_native_get_ns_window_insets() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_get_ns_window_insets(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CPlatformWindow.nativeGetTopmostPlatformWindowUnderMouse()Lsun/lwawt/macosx/CPlatformWindow;"
    )]
    async fn test_native_get_topmost_platform_window_under_mouse() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_get_topmost_platform_window_under_mouse(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CPlatformWindow.nativePushNSWindowToBack(J)V"
    )]
    async fn test_native_push_ns_window_to_back() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_push_ns_window_to_back(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CPlatformWindow.nativePushNSWindowToFront(J)V"
    )]
    async fn test_native_push_ns_window_to_front() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_push_ns_window_to_front(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CPlatformWindow.nativeRevalidateNSWindowShadow(J)V"
    )]
    async fn test_native_revalidate_ns_window_shadow() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_revalidate_ns_window_shadow(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CPlatformWindow.nativeSetAllowAutomaticTabbingProperty(Z)V"
    )]
    async fn test_native_set_allow_automatic_tabbing_property() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_allow_automatic_tabbing_property(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CPlatformWindow.nativeSetEnabled(JZ)V"
    )]
    async fn test_native_set_enabled() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_enabled(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CPlatformWindow.nativeSetNSWindowBounds(JDDDD)V"
    )]
    async fn test_native_set_ns_window_bounds() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_ns_window_bounds(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CPlatformWindow.nativeSetNSWindowLocationByPlatform(J)V"
    )]
    async fn test_native_set_ns_window_location_by_platform() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_ns_window_location_by_platform(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CPlatformWindow.nativeSetNSWindowMenuBar(JJ)V"
    )]
    async fn test_native_set_ns_window_menu_bar() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_ns_window_menu_bar(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CPlatformWindow.nativeSetNSWindowMinMax(JDDDD)V"
    )]
    async fn test_native_set_ns_window_min_max() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_ns_window_min_max(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CPlatformWindow.nativeSetNSWindowMinimizedIcon(JJ)V"
    )]
    async fn test_native_set_ns_window_minimized_icon() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_ns_window_minimized_icon(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CPlatformWindow.nativeSetNSWindowRepresentedFilename(JLjava/lang/String;)V"
    )]
    async fn test_native_set_ns_window_represented_filename() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_ns_window_represented_filename(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CPlatformWindow.nativeSetNSWindowStandardFrame(JDDDD)V"
    )]
    async fn testnative_set_ns_window_standard_frame() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_ns_window_standard_frame(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CPlatformWindow.nativeSetNSWindowStyleBits(JII)V"
    )]
    async fn test_native_set_ns_window_style_bits() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_ns_window_style_bits(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CPlatformWindow.nativeSetNSWindowTitle(JLjava/lang/String;)V"
    )]
    async fn test_native_set_ns_window_title() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_ns_window_title(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CPlatformWindow.nativeSynthesizeMouseEnteredExitedEvents()V"
    )]
    async fn test_native_synthesize_mouse_entered_exited_events_1() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ =
            native_synthesize_mouse_entered_exited_events_1(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CPlatformWindow.nativeSynthesizeMouseEnteredExitedEvents(JI)V"
    )]
    async fn test_native_synthesize_mouse_entered_exited_events_2() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ =
            native_synthesize_mouse_entered_exited_events_2(thread, Parameters::default()).await;
    }
}
