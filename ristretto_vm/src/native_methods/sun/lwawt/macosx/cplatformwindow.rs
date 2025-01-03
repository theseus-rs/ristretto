use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_11: Version = Version::Java11 { minor: 0 };
const JAVA_17: Version = Version::Java17 { minor: 0 };

/// Register all native methods for `sun.lwawt.macosx.CPlatformWindow`.
#[expect(clippy::too_many_lines)]
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/lwawt/macosx/CPlatformWindow";
    let java_version = registry.java_version().clone();

    if java_version >= JAVA_11 {
        registry.register(
            class_name,
            "nativeSetNSWindowLocationByPlatform",
            "(J)V",
            native_set_ns_window_location_by_platform,
        );
        registry.register(
            class_name,
            "nativeSetNSWindowStandardFrame",
            "(JDDDD)V",
            native_set_ns_window_standard_frame,
        );
    }

    if java_version >= JAVA_17 {
        registry.register(
            class_name,
            "nativeSetAllowAutomaticTabbingProperty",
            "(Z)V",
            native_set_allow_automatic_tabbing_property,
        );
        registry.register(
            class_name,
            "nativeSetNSWindowLocationByPlatform",
            "(J)V",
            native_set_ns_window_location_by_platform,
        );
    }

    registry.register(
        class_name,
        "_toggleFullScreenMode",
        "(J)V",
        toggle_full_screen_mode,
    );
    registry.register(
        class_name,
        "nativeCreateNSWindow",
        "(JJJDDDD)J",
        native_create_ns_window,
    );
    registry.register(class_name, "nativeDispose", "(J)V", native_dispose);
    registry.register(
        class_name,
        "nativeEnterFullScreenMode",
        "(J)V",
        native_enter_full_screen_mode,
    );
    registry.register(
        class_name,
        "nativeExitFullScreenMode",
        "(J)V",
        native_exit_full_screen_mode,
    );
    registry.register(
        class_name,
        "nativeGetNSWindowInsets",
        "(J)Ljava/awt/Insets;",
        native_get_ns_window_insets,
    );
    registry.register(
        class_name,
        "nativeGetTopmostPlatformWindowUnderMouse",
        "()Lsun/lwawt/macosx/CPlatformWindow;",
        native_get_topmost_platform_window_under_mouse,
    );
    registry.register(
        class_name,
        "nativePushNSWindowToBack",
        "(J)V",
        native_push_ns_window_to_back,
    );
    registry.register(
        class_name,
        "nativePushNSWindowToFront",
        "(J)V",
        native_push_ns_window_to_front,
    );
    registry.register(
        class_name,
        "nativeRevalidateNSWindowShadow",
        "(J)V",
        native_revalidate_ns_window_shadow,
    );
    registry.register(class_name, "nativeSetEnabled", "(JZ)V", native_set_enabled);
    registry.register(
        class_name,
        "nativeSetNSWindowBounds",
        "(JDDDD)V",
        native_set_ns_window_bounds,
    );
    registry.register(
        class_name,
        "nativeSetNSWindowMenuBar",
        "(JJ)V",
        native_set_ns_window_menu_bar,
    );
    registry.register(
        class_name,
        "nativeSetNSWindowMinMax",
        "(JDDDD)V",
        native_set_ns_window_min_max,
    );
    registry.register(
        class_name,
        "nativeSetNSWindowMinimizedIcon",
        "(JJ)V",
        native_set_ns_window_minimized_icon,
    );
    registry.register(
        class_name,
        "nativeSetNSWindowRepresentedFilename",
        "(JLjava/lang/String;)V",
        native_set_ns_window_represented_filename,
    );
    registry.register(
        class_name,
        "nativeSetNSWindowStyleBits",
        "(JII)V",
        native_set_ns_window_style_bits,
    );
    registry.register(
        class_name,
        "nativeSetNSWindowTitle",
        "(JLjava/lang/String;)V",
        native_set_ns_window_title,
    );
    registry.register(
        class_name,
        "nativeSynthesizeMouseEnteredExitedEvents",
        "()V",
        native_synthesize_mouse_entered_exited_events_1,
    );
    registry.register(
        class_name,
        "nativeSynthesizeMouseEnteredExitedEvents",
        "(JI)V",
        native_synthesize_mouse_entered_exited_events_2,
    );
}

#[async_recursion(?Send)]
async fn toggle_full_screen_mode(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow._toggleFullScreenMode(J)V")
}

#[async_recursion(?Send)]
async fn native_create_ns_window(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow.nativeCreateNSWindow(JJJDDDD)J")
}

#[async_recursion(?Send)]
async fn native_dispose(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow.nativeDispose(J)V")
}

#[async_recursion(?Send)]
async fn native_enter_full_screen_mode(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow.nativeEnterFullScreenMode(J)V")
}

#[async_recursion(?Send)]
async fn native_exit_full_screen_mode(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow.nativeExitFullScreenMode(J)V")
}

#[async_recursion(?Send)]
async fn native_get_ns_window_insets(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow.nativeGetNSWindowInsets(J)Ljava/awt/Insets;")
}

#[async_recursion(?Send)]
async fn native_get_topmost_platform_window_under_mouse(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow.nativeGetTopmostPlatformWindowUnderMouse()Lsun/lwawt/macosx/CPlatformWindow;")
}

#[async_recursion(?Send)]
async fn native_push_ns_window_to_back(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow.nativePushNSWindowToBack(J)V")
}

#[async_recursion(?Send)]
async fn native_push_ns_window_to_front(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow.nativePushNSWindowToFront(J)V")
}

#[async_recursion(?Send)]
async fn native_revalidate_ns_window_shadow(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow.nativeRevalidateNSWindowShadow(J)V")
}

#[async_recursion(?Send)]
async fn native_set_allow_automatic_tabbing_property(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow.nativeSetAllowAutomaticTabbingProperty(Z)V")
}

#[async_recursion(?Send)]
async fn native_set_enabled(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow.nativeSetEnabled(JZ)V")
}

#[async_recursion(?Send)]
async fn native_set_ns_window_bounds(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow.nativeSetNSWindowBounds(JDDDD)V")
}

#[async_recursion(?Send)]
async fn native_set_ns_window_location_by_platform(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow.nativeSetNSWindowLocationByPlatform(J)V")
}

#[async_recursion(?Send)]
async fn native_set_ns_window_menu_bar(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow.nativeSetNSWindowMenuBar(JJ)V")
}

#[async_recursion(?Send)]
async fn native_set_ns_window_min_max(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow.nativeSetNSWindowMinMax(JDDDD)V")
}

#[async_recursion(?Send)]
async fn native_set_ns_window_minimized_icon(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow.nativeSetNSWindowMinimizedIcon(JJ)V")
}

#[async_recursion(?Send)]
async fn native_set_ns_window_represented_filename(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow.nativeSetNSWindowRepresentedFilename(JLjava/lang/String;)V")
}

#[async_recursion(?Send)]
async fn native_set_ns_window_standard_frame(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow.nativeSetNSWindowStandardFrame(JDDDD)V")
}

#[async_recursion(?Send)]
async fn native_set_ns_window_style_bits(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow.nativeSetNSWindowStyleBits(JII)V")
}

#[async_recursion(?Send)]
async fn native_set_ns_window_title(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow.nativeSetNSWindowTitle(JLjava/lang/String;)V")
}

#[async_recursion(?Send)]
async fn native_synthesize_mouse_entered_exited_events_1(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow.nativeSynthesizeMouseEnteredExitedEvents()V")
}

#[async_recursion(?Send)]
async fn native_synthesize_mouse_entered_exited_events_2(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPlatformWindow.nativeSynthesizeMouseEnteredExitedEvents(JI)V")
}
