use ristretto_classfile::JAVA_8;
#[cfg(target_os = "linux")]
use ristretto_classfile::JAVA_11;
#[cfg(target_os = "linux")]
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKEngine.nativeFinishPainting([III)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_finish_painting<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _height = parameters.pop_int()?;
    let _width = parameters.pop_int()?;
    let _buffer = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.java.swing.plaf.gtk.GTKEngine.nativeFinishPainting([III)I".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKEngine.nativeSetRangeValue(IDDDD)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_set_range_value<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _visible = parameters.pop_double()?;
    let _max = parameters.pop_double()?;
    let _min = parameters.pop_double()?;
    let _value = parameters.pop_double()?;
    let _widget_type = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.java.swing.plaf.gtk.GTKEngine.nativeSetRangeValue(IDDDD)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKEngine.nativeStartPainting(II)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_start_painting<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.java.swing.plaf.gtk.GTKEngine.nativeStartPainting(II)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKEngine.native_get_gtk_setting(I)Ljava/lang/Object;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_get_gtk_setting<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _property = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.java.swing.plaf.gtk.GTKEngine.nativeGetGTKSetting(I)Ljava/lang/Object;"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_arrow(IIILjava/lang/String;IIIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_paint_arrow<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arrow_type = parameters.pop_int()?;
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _detail = parameters.pop_reference()?;
    let _shadow_type = parameters.pop_int()?;
    let _state = parameters.pop_int()?;
    let _widget_type = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.java.swing.plaf.gtk.GTKEngine.nativePaintArrow(IIILjava/lang/String;IIIII)V"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_background(IIIIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_paint_background<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _height = parameters.pop_int()?;
    let _width = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _state = parameters.pop_int()?;
    let _widget_type = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.java.swing.plaf.gtk.GTKEngine.nativePaintBackground(IIIIII)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_box(IIILjava/lang/String;IIIIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_paint_box<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _dir = parameters.pop_int()?;
    let _synth_state = parameters.pop_int()?;
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _detail = parameters.pop_reference()?;
    let _shadow_type = parameters.pop_int()?;
    let _state = parameters.pop_int()?;
    let _widget_type = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.java.swing.plaf.gtk.GTKEngine.nativePaintBox(IIILjava/lang/String;IIIIII)V"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_box_gap(IIILjava/lang/String;IIIIIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_paint_box_gap<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _gap_w = parameters.pop_int()?;
    let _gap_x = parameters.pop_int()?;
    let _gap_side = parameters.pop_int()?;
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _detail = parameters.pop_reference()?;
    let _shadow_type = parameters.pop_int()?;
    let _state = parameters.pop_int()?;
    let _widget_type = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.java.swing.plaf.gtk.GTKEngine.nativePaintBoxGap(IIILjava/lang/String;IIIIIII)V"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_check(IILjava/lang/String;IIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_paint_check<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _detail = parameters.pop_reference()?;
    let _synth_state = parameters.pop_int()?;
    let _widget_type = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.java.swing.plaf.gtk.GTKEngine.nativePaintCheck(IILjava/lang/String;IIII)V"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_expander(IILjava/lang/String;IIIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_paint_expander<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _expander_style = parameters.pop_int()?;
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _detail = parameters.pop_reference()?;
    let _state = parameters.pop_int()?;
    let _widget_type = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.java.swing.plaf.gtk.GTKEngine.nativePaintExpander(IILjava/lang/String;IIIII)V"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_extension(IIILjava/lang/String;IIIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_paint_extension<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _placement = parameters.pop_int()?;
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _detail = parameters.pop_reference()?;
    let _shadow_type = parameters.pop_int()?;
    let _state = parameters.pop_int()?;
    let _widget_type = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.java.swing.plaf.gtk.GTKEngine.nativePaintExtension(IIILjava/lang/String;IIIII)V"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_flat_box(IIILjava/lang/String;IIIIZ)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_paint_flat_box<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _has_focus = parameters.pop_bool()?;
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _detail = parameters.pop_reference()?;
    let _shadow_type = parameters.pop_int()?;
    let _state = parameters.pop_int()?;
    let _widget_type = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.java.swing.plaf.gtk.GTKEngine.nativePaintFlatBox(IIILjava/lang/String;IIIIZ)V"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_focus(IILjava/lang/String;IIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_paint_focus<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _detail = parameters.pop_reference()?;
    let _state = parameters.pop_int()?;
    let _widget_type = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.java.swing.plaf.gtk.GTKEngine.nativePaintFocus(IILjava/lang/String;IIII)V"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_handle(IIILjava/lang/String;IIIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_paint_handle<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _orientation = parameters.pop_int()?;
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _detail = parameters.pop_reference()?;
    let _shadow_type = parameters.pop_int()?;
    let _state = parameters.pop_int()?;
    let _widget_type = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.java.swing.plaf.gtk.GTKEngine.nativePaintHandle(IIILjava/lang/String;IIIII)V"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_hline(IILjava/lang/String;IIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_paint_hline<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _detail = parameters.pop_reference()?;
    let _state = parameters.pop_int()?;
    let _widget_type = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.java.swing.plaf.gtk.GTKEngine.nativePaintHline(IILjava/lang/String;IIII)V"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_option(IILjava/lang/String;IIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_paint_option<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _detail = parameters.pop_reference()?;
    let _synth_state = parameters.pop_int()?;
    let _widget_type = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.java.swing.plaf.gtk.GTKEngine.nativePaintOption(IILjava/lang/String;IIII)V"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_shadow(IIILjava/lang/String;IIIIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_paint_shadow<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _dir = parameters.pop_int()?;
    let _synth_state = parameters.pop_int()?;
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _detail = parameters.pop_reference()?;
    let _shadow_type = parameters.pop_int()?;
    let _state = parameters.pop_int()?;
    let _widget_type = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.java.swing.plaf.gtk.GTKEngine.nativePaintShadow(IIILjava/lang/String;IIIIII)V"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_slider(IIILjava/lang/String;IIIIIZ)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_paint_slider<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _has_focus = parameters.pop_bool()?;
    let _orientation = parameters.pop_int()?;
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _detail = parameters.pop_reference()?;
    let _shadow_type = parameters.pop_int()?;
    let _state = parameters.pop_int()?;
    let _widget_type = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.java.swing.plaf.gtk.GTKEngine.nativePaintSlider(IIILjava/lang/String;IIIIIZ)V"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_vline(IILjava/lang/String;IIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_paint_vline<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _detail = parameters.pop_reference()?;
    let _state = parameters.pop_int()?;
    let _widget_type = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.java.swing.plaf.gtk.GTKEngine.nativePaintVline(IILjava/lang/String;IIII)V"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKEngine.native_switch_theme()V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn native_switch_theme<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.java.swing.plaf.gtk.GTKEngine.native_switch_theme()V".to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKEngine.nativeFinishPainting([III)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn native_finish_painting_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _height = parameters.pop_int()?;
    let _width = parameters.pop_int()?;
    let _buffer = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com/sun/java/swing/plaf/gtk/GTKEngine.nativeFinishPainting([III)I".to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKEngine.nativeSetRangeValue(IDDDD)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn native_set_range_value_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _visible = parameters.pop_double()?;
    let _max = parameters.pop_double()?;
    let _min = parameters.pop_double()?;
    let _value = parameters.pop_double()?;
    let _widget_type = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com/sun/java/swing/plaf/gtk/GTKEngine.nativeSetRangeValue(IDDDD)V".to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKEngine.nativeStartPainting(II)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn native_start_painting_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com/sun/java/swing/plaf/gtk/GTKEngine.nativeStartPainting(II)V".to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKEngine.native_get_gtk_setting(I)Ljava/lang/Object;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn native_get_gtk_setting_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _property = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com/sun/java/swing/plaf/gtk/GTKEngine.native_get_gtk_setting(I)Ljava/lang/Object;"
            .to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_arrow(IIILjava/lang/String;IIIII)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn native_paint_arrow_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arrow_type = parameters.pop_int()?;
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _detail = parameters.pop_reference()?;
    let _shadow_type = parameters.pop_int()?;
    let _state = parameters.pop_int()?;
    let _widget_type = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_arrow(IIILjava/lang/String;IIIII)V"
            .to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_background(IIIIII)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn native_paint_background_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _height = parameters.pop_int()?;
    let _width = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _state = parameters.pop_int()?;
    let _widget_type = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_background(IIIIII)V".to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_box(IIILjava/lang/String;IIIIII)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn native_paint_box_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _dir = parameters.pop_int()?;
    let _synth_state = parameters.pop_int()?;
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _detail = parameters.pop_reference()?;
    let _shadow_type = parameters.pop_int()?;
    let _state = parameters.pop_int()?;
    let _widget_type = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_box(IIILjava/lang/String;IIIIII)V"
            .to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_box_gap(IIILjava/lang/String;IIIIIII)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn native_paint_box_gap_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _gap_w = parameters.pop_int()?;
    let _gap_x = parameters.pop_int()?;
    let _gap_side = parameters.pop_int()?;
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _detail = parameters.pop_reference()?;
    let _shadow_type = parameters.pop_int()?;
    let _state = parameters.pop_int()?;
    let _widget_type = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_box_gap(IIILjava/lang/String;IIIIIII)V"
            .to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_check(IILjava/lang/String;IIII)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn native_paint_check_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _detail = parameters.pop_reference()?;
    let _synth_state = parameters.pop_int()?;
    let _widget_type = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_check(IILjava/lang/String;IIII)V"
            .to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_expander(IILjava/lang/String;IIIII)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn native_paint_expander_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _expander_style = parameters.pop_int()?;
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _detail = parameters.pop_reference()?;
    let _state = parameters.pop_int()?;
    let _widget_type = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_expander(IILjava/lang/String;IIIII)V"
            .to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_extension(IIILjava/lang/String;IIIII)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn native_paint_extension_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _placement = parameters.pop_int()?;
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _detail = parameters.pop_reference()?;
    let _shadow_type = parameters.pop_int()?;
    let _state = parameters.pop_int()?;
    let _widget_type = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_extension(IIILjava/lang/String;IIIII)V"
            .to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_flat_box(IIILjava/lang/String;IIIIZ)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn native_paint_flat_box_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _has_focus = parameters.pop_bool()?;
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _detail = parameters.pop_reference()?;
    let _shadow_type = parameters.pop_int()?;
    let _state = parameters.pop_int()?;
    let _widget_type = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_flat_box(IIILjava/lang/String;IIIIZ)V"
            .to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_focus(IILjava/lang/String;IIII)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn native_paint_focus_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _detail = parameters.pop_reference()?;
    let _state = parameters.pop_int()?;
    let _widget_type = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_focus(IILjava/lang/String;IIII)V"
            .to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_handle(IIILjava/lang/String;IIIII)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn native_paint_handle_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _orientation = parameters.pop_int()?;
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _detail = parameters.pop_reference()?;
    let _shadow_type = parameters.pop_int()?;
    let _state = parameters.pop_int()?;
    let _widget_type = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_handle(IIILjava/lang/String;IIIII)V"
            .to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_hline(IILjava/lang/String;IIII)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn native_paint_hline_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _detail = parameters.pop_reference()?;
    let _state = parameters.pop_int()?;
    let _widget_type = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_hline(IILjava/lang/String;IIII)V"
            .to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_option(IILjava/lang/String;IIII)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn native_paint_option_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _detail = parameters.pop_reference()?;
    let _synth_state = parameters.pop_int()?;
    let _widget_type = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_option(IILjava/lang/String;IIII)V"
            .to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_shadow(IIILjava/lang/String;IIIIII)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn native_paint_shadow_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _dir = parameters.pop_int()?;
    let _synth_state = parameters.pop_int()?;
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _detail = parameters.pop_reference()?;
    let _shadow_type = parameters.pop_int()?;
    let _state = parameters.pop_int()?;
    let _widget_type = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_shadow(IIILjava/lang/String;IIIIII)V"
            .to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_slider(IIILjava/lang/String;IIIIIZ)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn native_paint_slider_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _has_focus = parameters.pop_bool()?;
    let _orientation = parameters.pop_int()?;
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _detail = parameters.pop_reference()?;
    let _shadow_type = parameters.pop_int()?;
    let _state = parameters.pop_int()?;
    let _widget_type = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_slider(IIILjava/lang/String;IIIIIZ)V"
            .to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_vline(IILjava/lang/String;IIII)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn native_paint_vline_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _detail = parameters.pop_reference()?;
    let _state = parameters.pop_int()?;
    let _widget_type = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_vline(IILjava/lang/String;IIII)V"
            .to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "com/sun/java/swing/plaf/gtk/GTKEngine.native_switch_theme()V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn native_switch_theme_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com/sun/java/swing/plaf/gtk/GTKEngine.native_switch_theme()V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_native_finish_painting() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = native_finish_painting(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Int(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "com.sun.java.swing.plaf.gtk.GTKEngine.nativeFinishPainting([III)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_set_range_value() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = native_set_range_value(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Double(0.0),
                Value::Double(0.0),
                Value::Double(0.0),
                Value::Double(0.0),
            ]),
        )
        .await;
        assert_eq!(
            "com.sun.java.swing.plaf.gtk.GTKEngine.nativeSetRangeValue(IDDDD)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_start_painting() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result =
            native_start_painting(thread, Parameters::new(vec![Value::Int(0), Value::Int(0)]))
                .await;
        assert_eq!(
            "com.sun.java.swing.plaf.gtk.GTKEngine.nativeStartPainting(II)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_get_gtk_setting() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = native_get_gtk_setting(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "com.sun.java.swing.plaf.gtk.GTKEngine.nativeGetGTKSetting(I)Ljava/lang/Object;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_paint_arrow() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = native_paint_arrow(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "com.sun.java.swing.plaf.gtk.GTKEngine.nativePaintArrow(IIILjava/lang/String;IIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_paint_background() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = native_paint_background(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "com.sun.java.swing.plaf.gtk.GTKEngine.nativePaintBackground(IIIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_paint_box() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = native_paint_box(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "com.sun.java.swing.plaf.gtk.GTKEngine.nativePaintBox(IIILjava/lang/String;IIIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_paint_box_gap() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = native_paint_box_gap(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "com.sun.java.swing.plaf.gtk.GTKEngine.nativePaintBoxGap(IIILjava/lang/String;IIIIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_paint_check() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = native_paint_check(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "com.sun.java.swing.plaf.gtk.GTKEngine.nativePaintCheck(IILjava/lang/String;IIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_paint_expander() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = native_paint_expander(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "com.sun.java.swing.plaf.gtk.GTKEngine.nativePaintExpander(IILjava/lang/String;IIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_paint_extension() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = native_paint_extension(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "com.sun.java.swing.plaf.gtk.GTKEngine.nativePaintExtension(IIILjava/lang/String;IIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_paint_flat_box() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = native_paint_flat_box(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::from(false),
            ]),
        )
        .await;
        assert_eq!(
            "com.sun.java.swing.plaf.gtk.GTKEngine.nativePaintFlatBox(IIILjava/lang/String;IIIIZ)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_paint_focus() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = native_paint_focus(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "com.sun.java.swing.plaf.gtk.GTKEngine.nativePaintFocus(IILjava/lang/String;IIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_paint_handle() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = native_paint_handle(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "com.sun.java.swing.plaf.gtk.GTKEngine.nativePaintHandle(IIILjava/lang/String;IIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_paint_hline() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = native_paint_hline(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "com.sun.java.swing.plaf.gtk.GTKEngine.nativePaintHline(IILjava/lang/String;IIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_paint_option() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = native_paint_option(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "com.sun.java.swing.plaf.gtk.GTKEngine.nativePaintOption(IILjava/lang/String;IIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_paint_shadow() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = native_paint_shadow(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "com.sun.java.swing.plaf.gtk.GTKEngine.nativePaintShadow(IIILjava/lang/String;IIIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_paint_slider() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = native_paint_slider(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::from(false),
            ]),
        )
        .await;
        assert_eq!(
            "com.sun.java.swing.plaf.gtk.GTKEngine.nativePaintSlider(IIILjava/lang/String;IIIIIZ)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_paint_vline() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = native_paint_vline(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "com.sun.java.swing.plaf.gtk.GTKEngine.nativePaintVline(IILjava/lang/String;IIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_switch_theme() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = native_switch_theme(thread, Parameters::default()).await;
        assert_eq!(
            "com.sun.java.swing.plaf.gtk.GTKEngine.native_switch_theme()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_native_finish_painting_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_finish_painting_linux_ge_v11(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Int(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "com/sun/java/swing/plaf/gtk/GTKEngine.nativeFinishPainting([III)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_native_set_range_value_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_set_range_value_linux_ge_v11(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Double(0.0),
                Value::Double(0.0),
                Value::Double(0.0),
                Value::Double(0.0),
            ]),
        )
        .await;
        assert_eq!(
            "com/sun/java/swing/plaf/gtk/GTKEngine.nativeSetRangeValue(IDDDD)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_native_start_painting_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_start_painting_linux_ge_v11(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "com/sun/java/swing/plaf/gtk/GTKEngine.nativeStartPainting(II)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_native_get_gtk_setting_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            native_get_gtk_setting_linux_ge_v11(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "com/sun/java/swing/plaf/gtk/GTKEngine.native_get_gtk_setting(I)Ljava/lang/Object;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_native_paint_arrow_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_paint_arrow_linux_ge_v11(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_arrow(IIILjava/lang/String;IIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_native_paint_background_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_paint_background_linux_ge_v11(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_background(IIIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_native_paint_box_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_paint_box_linux_ge_v11(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_box(IIILjava/lang/String;IIIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_native_paint_box_gap_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_paint_box_gap_linux_ge_v11(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_box_gap(IIILjava/lang/String;IIIIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_native_paint_check_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_paint_check_linux_ge_v11(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_check(IILjava/lang/String;IIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_native_paint_expander_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_paint_expander_linux_ge_v11(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_expander(IILjava/lang/String;IIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_native_paint_extension_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_paint_extension_linux_ge_v11(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_extension(IIILjava/lang/String;IIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_native_paint_flat_box_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_paint_flat_box_linux_ge_v11(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::from(false),
            ]),
        )
        .await;
        assert_eq!(
            "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_flat_box(IIILjava/lang/String;IIIIZ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_native_paint_focus_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_paint_focus_linux_ge_v11(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_focus(IILjava/lang/String;IIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_native_paint_handle_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_paint_handle_linux_ge_v11(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_handle(IIILjava/lang/String;IIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_native_paint_hline_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_paint_hline_linux_ge_v11(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_hline(IILjava/lang/String;IIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_native_paint_option_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_paint_option_linux_ge_v11(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_option(IILjava/lang/String;IIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_native_paint_shadow_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_paint_shadow_linux_ge_v11(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_shadow(IIILjava/lang/String;IIIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_native_paint_slider_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_paint_slider_linux_ge_v11(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::from(false),
            ]),
        )
        .await;
        assert_eq!(
            "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_slider(IIILjava/lang/String;IIIIIZ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_native_paint_vline_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_paint_vline_linux_ge_v11(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "com/sun/java/swing/plaf/gtk/GTKEngine.native_paint_vline(IILjava/lang/String;IIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_native_switch_theme_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_switch_theme_linux_ge_v11(thread, Parameters::default()).await;
        assert_eq!(
            "com/sun/java/swing/plaf/gtk/GTKEngine.native_switch_theme()V",
            result.unwrap_err().to_string()
        );
    }
}
