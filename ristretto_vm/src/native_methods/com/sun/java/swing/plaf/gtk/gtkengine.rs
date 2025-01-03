use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `com.sun.java.swing.plaf.gtk.GTKEngine`.
#[expect(clippy::too_many_lines)]
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "com/sun/java/swing/plaf/gtk/GTKEngine";
    registry.register(
        class_name,
        "nativeFinishPainting",
        "([III)I",
        native_finish_painting,
    );
    registry.register(
        class_name,
        "nativeSetRangeValue",
        "(IDDDD)V",
        native_set_range_value,
    );
    registry.register(
        class_name,
        "nativeStartPainting",
        "(II)V",
        native_start_painting,
    );
    registry.register(
        class_name,
        "native_get_gtk_setting",
        "(I)Ljava/lang/Object;",
        native_get_gtk_setting,
    );
    registry.register(
        class_name,
        "native_paint_arrow",
        "(IIILjava/lang/String;IIIII)V",
        native_paint_arrow,
    );
    registry.register(
        class_name,
        "native_paint_background",
        "(IIIIII)V",
        native_paint_background,
    );
    registry.register(
        class_name,
        "native_paint_box",
        "(IIILjava/lang/String;IIIIII)V",
        native_paint_box,
    );
    registry.register(
        class_name,
        "native_paint_box_gap",
        "(IIILjava/lang/String;IIIIIII)V",
        native_paint_box_gap,
    );
    registry.register(
        class_name,
        "native_paint_check",
        "(IILjava/lang/String;IIII)V",
        native_paint_check,
    );
    registry.register(
        class_name,
        "native_paint_expander",
        "(IILjava/lang/String;IIIII)V",
        native_paint_expander,
    );
    registry.register(
        class_name,
        "native_paint_extension",
        "(IIILjava/lang/String;IIIII)V",
        native_paint_extension,
    );
    registry.register(
        class_name,
        "native_paint_flat_box",
        "(IIILjava/lang/String;IIIIZ)V",
        native_paint_flat_box,
    );
    registry.register(
        class_name,
        "native_paint_focus",
        "(IILjava/lang/String;IIII)V",
        native_paint_focus,
    );
    registry.register(
        class_name,
        "native_paint_handle",
        "(IIILjava/lang/String;IIIII)V",
        native_paint_handle,
    );
    registry.register(
        class_name,
        "native_paint_hline",
        "(IILjava/lang/String;IIII)V",
        native_paint_hline,
    );
    registry.register(
        class_name,
        "native_paint_option",
        "(IILjava/lang/String;IIII)V",
        native_paint_option,
    );
    registry.register(
        class_name,
        "native_paint_shadow",
        "(IIILjava/lang/String;IIIIII)V",
        native_paint_shadow,
    );
    registry.register(
        class_name,
        "native_paint_slider",
        "(IIILjava/lang/String;IIIIIZ)V",
        native_paint_slider,
    );
    registry.register(
        class_name,
        "native_paint_vline",
        "(IILjava/lang/String;IIII)V",
        native_paint_vline,
    );
    registry.register(
        class_name,
        "native_switch_theme",
        "()V",
        native_switch_theme,
    );
}

#[async_recursion(?Send)]
async fn native_finish_painting(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.sun.java.swing.plaf.gtk.GTKEngine.nativeFinishPainting([III)I")
}

#[async_recursion(?Send)]
async fn native_set_range_value(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.sun.java.swing.plaf.gtk.GTKEngine.nativeSetRangeValue(IDDDD)V")
}

#[async_recursion(?Send)]
async fn native_start_painting(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.sun.java.swing.plaf.gtk.GTKEngine.nativeStartPainting(II)V")
}

#[async_recursion(?Send)]
async fn native_get_gtk_setting(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.sun.java.swing.plaf.gtk.GTKEngine.nativeGetGTKSetting(I)Ljava/lang/Object;")
}

#[async_recursion(?Send)]
async fn native_paint_arrow(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.java.swing.plaf.gtk.GTKEngine.nativePaintArrow(IIILjava/lang/String;IIIII)V")
}

#[async_recursion(?Send)]
async fn native_paint_background(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.sun.java.swing.plaf.gtk.GTKEngine.nativePaintBackground(IIIIII)V")
}

#[async_recursion(?Send)]
async fn native_paint_box(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.java.swing.plaf.gtk.GTKEngine.nativePaintBox(IIILjava/lang/String;IIIIII)V")
}

#[async_recursion(?Send)]
async fn native_paint_box_gap(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.sun.java.swing.plaf.gtk.GTKEngine.nativePaintBoxGap(IIILjava/lang/String;IIIIIII)V")
}

#[async_recursion(?Send)]
async fn native_paint_check(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.java.swing.plaf.gtk.GTKEngine.nativePaintCheck(IILjava/lang/String;IIII)V")
}

#[async_recursion(?Send)]
async fn native_paint_expander(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.sun.java.swing.plaf.gtk.GTKEngine.nativePaintExpander(IILjava/lang/String;IIIII)V")
}

#[async_recursion(?Send)]
async fn native_paint_extension(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.sun.java.swing.plaf.gtk.GTKEngine.nativePaintExtension(IIILjava/lang/String;IIIII)V")
}

#[async_recursion(?Send)]
async fn native_paint_flat_box(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("com.sun.java.swing.plaf.gtk.GTKEngine.nativePaintFlatBox(IIILjava/lang/String;IIIIZ)V")
}

#[async_recursion(?Send)]
async fn native_paint_focus(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.java.swing.plaf.gtk.GTKEngine.nativePaintFocus(IILjava/lang/String;IIII)V")
}

#[async_recursion(?Send)]
async fn native_paint_handle(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.java.swing.plaf.gtk.GTKEngine.nativePaintHandle(IIILjava/lang/String;IIIII)V")
}

#[async_recursion(?Send)]
async fn native_paint_hline(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.java.swing.plaf.gtk.GTKEngine.nativePaintHline(IILjava/lang/String;IIII)V")
}

#[async_recursion(?Send)]
async fn native_paint_option(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.java.swing.plaf.gtk.GTKEngine.nativePaintOption(IILjava/lang/String;IIII)V")
}

#[async_recursion(?Send)]
async fn native_paint_shadow(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.java.swing.plaf.gtk.GTKEngine.nativePaintShadow(IIILjava/lang/String;IIIIII)V")
}

#[async_recursion(?Send)]
async fn native_paint_slider(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.java.swing.plaf.gtk.GTKEngine.nativePaintSlider(IIILjava/lang/String;IIIIIZ)V")
}

#[async_recursion(?Send)]
async fn native_paint_vline(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.java.swing.plaf.gtk.GTKEngine.nativePaintVline(IILjava/lang/String;IIII)V")
}

#[async_recursion(?Send)]
async fn native_switch_theme(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("com.sun.java.swing.plaf.gtk.GTKEngine.nativeSwitchTheme()V")
}
