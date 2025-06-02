use crate::Result;
use crate::intrinsic_methods::registry::{JAVA_11, JAVA_17, MethodRegistry};
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/lwawt/macosx/CPlatformWindow";

/// Register all intrinsic methods for `sun.lwawt.macosx.CPlatformWindow`.
#[expect(clippy::too_many_lines)]
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() >= JAVA_11 {
        registry.register(
            CLASS_NAME,
            "nativeSetNSWindowLocationByPlatform",
            "(J)V",
            native_set_ns_window_location_by_platform,
        );
        registry.register(
            CLASS_NAME,
            "nativeSetNSWindowStandardFrame",
            "(JDDDD)V",
            native_set_ns_window_standard_frame,
        );
    }

    if registry.java_major_version() >= JAVA_17 {
        registry.register(
            CLASS_NAME,
            "nativeSetAllowAutomaticTabbingProperty",
            "(Z)V",
            native_set_allow_automatic_tabbing_property,
        );
        registry.register(
            CLASS_NAME,
            "nativeSetNSWindowLocationByPlatform",
            "(J)V",
            native_set_ns_window_location_by_platform,
        );
    }

    registry.register(
        CLASS_NAME,
        "_toggleFullScreenMode",
        "(J)V",
        toggle_full_screen_mode,
    );
    registry.register(
        CLASS_NAME,
        "nativeCreateNSWindow",
        "(JJJDDDD)J",
        native_create_ns_window,
    );
    registry.register(CLASS_NAME, "nativeDispose", "(J)V", native_dispose);
    registry.register(
        CLASS_NAME,
        "nativeEnterFullScreenMode",
        "(J)V",
        native_enter_full_screen_mode,
    );
    registry.register(
        CLASS_NAME,
        "nativeExitFullScreenMode",
        "(J)V",
        native_exit_full_screen_mode,
    );
    registry.register(
        CLASS_NAME,
        "nativeGetNSWindowInsets",
        "(J)Ljava/awt/Insets;",
        native_get_ns_window_insets,
    );
    registry.register(
        CLASS_NAME,
        "nativeGetTopmostPlatformWindowUnderMouse",
        "()Lsun/lwawt/macosx/CPlatformWindow;",
        native_get_topmost_platform_window_under_mouse,
    );
    registry.register(
        CLASS_NAME,
        "nativePushNSWindowToBack",
        "(J)V",
        native_push_ns_window_to_back,
    );
    registry.register(
        CLASS_NAME,
        "nativePushNSWindowToFront",
        "(J)V",
        native_push_ns_window_to_front,
    );
    registry.register(
        CLASS_NAME,
        "nativeRevalidateNSWindowShadow",
        "(J)V",
        native_revalidate_ns_window_shadow,
    );
    registry.register(CLASS_NAME, "nativeSetEnabled", "(JZ)V", native_set_enabled);
    registry.register(
        CLASS_NAME,
        "nativeSetNSWindowBounds",
        "(JDDDD)V",
        native_set_ns_window_bounds,
    );
    registry.register(
        CLASS_NAME,
        "nativeSetNSWindowMenuBar",
        "(JJ)V",
        native_set_ns_window_menu_bar,
    );
    registry.register(
        CLASS_NAME,
        "nativeSetNSWindowMinMax",
        "(JDDDD)V",
        native_set_ns_window_min_max,
    );
    registry.register(
        CLASS_NAME,
        "nativeSetNSWindowMinimizedIcon",
        "(JJ)V",
        native_set_ns_window_minimized_icon,
    );
    registry.register(
        CLASS_NAME,
        "nativeSetNSWindowRepresentedFilename",
        "(JLjava/lang/String;)V",
        native_set_ns_window_represented_filename,
    );
    registry.register(
        CLASS_NAME,
        "nativeSetNSWindowStyleBits",
        "(JII)V",
        native_set_ns_window_style_bits,
    );
    registry.register(
        CLASS_NAME,
        "nativeSetNSWindowTitle",
        "(JLjava/lang/String;)V",
        native_set_ns_window_title,
    );
    registry.register(
        CLASS_NAME,
        "nativeSynthesizeMouseEnteredExitedEvents",
        "()V",
        native_synthesize_mouse_entered_exited_events_1,
    );
    registry.register(
        CLASS_NAME,
        "nativeSynthesizeMouseEnteredExitedEvents",
        "(JI)V",
        native_synthesize_mouse_entered_exited_events_2,
    );
}

#[async_recursion(?Send)]
async fn toggle_full_screen_mode(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow._toggleFullScreenMode(J)V")
}

#[async_recursion(?Send)]
async fn native_create_ns_window(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow.nativeCreateNSWindow(JJJDDDD)J")
}

#[async_recursion(?Send)]
async fn native_dispose(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow.nativeDispose(J)V")
}

#[async_recursion(?Send)]
async fn native_enter_full_screen_mode(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow.nativeEnterFullScreenMode(J)V")
}

#[async_recursion(?Send)]
async fn native_exit_full_screen_mode(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow.nativeExitFullScreenMode(J)V")
}

#[async_recursion(?Send)]
async fn native_get_ns_window_insets(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow.nativeGetNSWindowInsets(J)Ljava/awt/Insets;")
}

#[async_recursion(?Send)]
async fn native_get_topmost_platform_window_under_mouse(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.lwawt.macosx.CPlatformWindow.nativeGetTopmostPlatformWindowUnderMouse()Lsun/lwawt/macosx/CPlatformWindow;"
    )
}

#[async_recursion(?Send)]
async fn native_push_ns_window_to_back(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow.nativePushNSWindowToBack(J)V")
}

#[async_recursion(?Send)]
async fn native_push_ns_window_to_front(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow.nativePushNSWindowToFront(J)V")
}

#[async_recursion(?Send)]
async fn native_revalidate_ns_window_shadow(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow.nativeRevalidateNSWindowShadow(J)V")
}

#[async_recursion(?Send)]
async fn native_set_allow_automatic_tabbing_property(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow.nativeSetAllowAutomaticTabbingProperty(Z)V")
}

#[async_recursion(?Send)]
async fn native_set_enabled(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow.nativeSetEnabled(JZ)V")
}

#[async_recursion(?Send)]
async fn native_set_ns_window_bounds(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow.nativeSetNSWindowBounds(JDDDD)V")
}

#[async_recursion(?Send)]
async fn native_set_ns_window_location_by_platform(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow.nativeSetNSWindowLocationByPlatform(J)V")
}

#[async_recursion(?Send)]
async fn native_set_ns_window_menu_bar(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow.nativeSetNSWindowMenuBar(JJ)V")
}

#[async_recursion(?Send)]
async fn native_set_ns_window_min_max(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow.nativeSetNSWindowMinMax(JDDDD)V")
}

#[async_recursion(?Send)]
async fn native_set_ns_window_minimized_icon(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow.nativeSetNSWindowMinimizedIcon(JJ)V")
}

#[async_recursion(?Send)]
async fn native_set_ns_window_represented_filename(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.lwawt.macosx.CPlatformWindow.nativeSetNSWindowRepresentedFilename(JLjava/lang/String;)V"
    )
}

#[async_recursion(?Send)]
async fn native_set_ns_window_standard_frame(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow.nativeSetNSWindowStandardFrame(JDDDD)V")
}

#[async_recursion(?Send)]
async fn native_set_ns_window_style_bits(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow.nativeSetNSWindowStyleBits(JII)V")
}

#[async_recursion(?Send)]
async fn native_set_ns_window_title(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow.nativeSetNSWindowTitle(JLjava/lang/String;)V")
}

#[async_recursion(?Send)]
async fn native_synthesize_mouse_entered_exited_events_1(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow.nativeSynthesizeMouseEnteredExitedEvents()V")
}

#[async_recursion(?Send)]
async fn native_synthesize_mouse_entered_exited_events_2(
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
