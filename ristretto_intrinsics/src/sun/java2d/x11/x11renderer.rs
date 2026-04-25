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
    "sun/java2d/x11/X11Renderer.XDoPath(Lsun/java2d/SunGraphics2D;JJIILjava/awt/geom/Path2D$Float;Z)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn x_do_path<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _is_fill = parameters.pop_bool()?;
    let _p2df = parameters.pop_reference()?;
    let _trans_y = parameters.pop_int()?;
    let _trans_x = parameters.pop_int()?;
    let _xgc = parameters.pop_long()?;
    let _p_xs_data = parameters.pop_long()?;
    let _sg2d = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun.java2d.x11.X11Renderer.XDoPath(Lsun/java2d/SunGraphics2D;JJIILjava/awt/geom/Path2D$Float;Z)V".to_string()).into())
}

#[intrinsic_method(
    "sun/java2d/x11/X11Renderer.XDrawArc(JJIIIIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn x_draw_arc<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _angle_extent = parameters.pop_int()?;
    let _angle_start = parameters.pop_int()?;
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _xgc = parameters.pop_long()?;
    let _p_xs_data = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.x11.X11Renderer.XDrawArc(JJIIIIII)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/x11/X11Renderer.XDrawLine(JJIIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn x_draw_line<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _y2 = parameters.pop_int()?;
    let _x2 = parameters.pop_int()?;
    let _y1 = parameters.pop_int()?;
    let _x1 = parameters.pop_int()?;
    let _xgc = parameters.pop_long()?;
    let _p_xs_data = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.x11.X11Renderer.XDrawLine(JJIIII)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/x11/X11Renderer.XDrawOval(JJIIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn x_draw_oval<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _xgc = parameters.pop_long()?;
    let _p_xs_data = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.x11.X11Renderer.XDrawOval(JJIIII)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/x11/X11Renderer.XDrawPoly(JJII[I[IIZ)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn x_draw_poly<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _isclosed = parameters.pop_bool()?;
    let _npoints = parameters.pop_int()?;
    let _ypoints = parameters.pop_reference()?;
    let _xpoints = parameters.pop_reference()?;
    let _transy = parameters.pop_int()?;
    let _transx = parameters.pop_int()?;
    let _xgc = parameters.pop_long()?;
    let _p_xs_data = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.x11.X11Renderer.XDrawPoly(JJII[I[IIZ)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/x11/X11Renderer.XDrawRect(JJIIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn x_draw_rect<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _xgc = parameters.pop_long()?;
    let _p_xs_data = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.x11.X11Renderer.XDrawRect(JJIIII)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/x11/X11Renderer.XDrawRoundRect(JJIIIIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn x_draw_round_rect<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arc_h = parameters.pop_int()?;
    let _arc_w = parameters.pop_int()?;
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _xgc = parameters.pop_long()?;
    let _p_xs_data = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.x11.X11Renderer.XDrawRoundRect(JJIIIIII)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/x11/X11Renderer.XFillArc(JJIIIIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn x_fill_arc<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _angle_extent = parameters.pop_int()?;
    let _angle_start = parameters.pop_int()?;
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _xgc = parameters.pop_long()?;
    let _p_xs_data = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.x11.X11Renderer.XFillArc(JJIIIIII)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/x11/X11Renderer.XFillOval(JJIIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn x_fill_oval<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _xgc = parameters.pop_long()?;
    let _p_xs_data = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.x11.X11Renderer.XFillOval(JJIIII)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/x11/X11Renderer.XFillPoly(JJII[I[II)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn x_fill_poly<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _npoints = parameters.pop_int()?;
    let _ypoints = parameters.pop_reference()?;
    let _xpoints = parameters.pop_reference()?;
    let _transy = parameters.pop_int()?;
    let _transx = parameters.pop_int()?;
    let _xgc = parameters.pop_long()?;
    let _p_xs_data = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.x11.X11Renderer.XFillPoly(JJII[I[II)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/x11/X11Renderer.XFillRect(JJIIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn x_fill_rect<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _xgc = parameters.pop_long()?;
    let _p_xs_data = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.x11.X11Renderer.XFillRect(JJIIII)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/x11/X11Renderer.XFillRoundRect(JJIIIIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn x_fill_round_rect<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arc_h = parameters.pop_int()?;
    let _arc_w = parameters.pop_int()?;
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _xgc = parameters.pop_long()?;
    let _p_xs_data = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.x11.X11Renderer.XFillRoundRect(JJIIIIII)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/x11/X11Renderer.XFillSpans(JJLsun/java2d/pipe/SpanIterator;JII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn x_fill_spans<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _transy = parameters.pop_int()?;
    let _transx = parameters.pop_int()?;
    let _iterator = parameters.pop_long()?;
    let _si = parameters.pop_reference()?;
    let _xgc = parameters.pop_long()?;
    let _p_xs_data = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.x11.X11Renderer.XFillSpans(JJLsun/java2d/pipe/SpanIterator;JII)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/x11/X11Renderer.devCopyArea(JJIIIIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn dev_copy_area<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _dsty = parameters.pop_int()?;
    let _dstx = parameters.pop_int()?;
    let _srcy = parameters.pop_int()?;
    let _srcx = parameters.pop_int()?;
    let _xgc = parameters.pop_long()?;
    let _sd_ops = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.x11.X11Renderer.devCopyArea(JJIIIIII)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/x11/X11Renderer.XDoPath(Lsun/java2d/SunGraphics2D;JJIILjava/awt/geom/Path2D$Float;Z)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn xdo_path_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _is_fill = parameters.pop_bool()?;
    let _p2df = parameters.pop_reference()?;
    let _trans_y = parameters.pop_int()?;
    let _trans_x = parameters.pop_int()?;
    let _xgc = parameters.pop_long()?;
    let _p_xs_data = parameters.pop_long()?;
    let _sg2d = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun/java2d/x11/X11Renderer.XDoPath(Lsun/java2d/SunGraphics2D;JJIILjava/awt/geom/Path2D$Float;Z)V".to_string()).into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/x11/X11Renderer.XDrawArc(JJIIIIII)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn xdraw_arc_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _angle_extent = parameters.pop_int()?;
    let _angle_start = parameters.pop_int()?;
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _xgc = parameters.pop_long()?;
    let _p_xs_data = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/x11/X11Renderer.XDrawArc(JJIIIIII)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/x11/X11Renderer.XDrawLine(JJIIII)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn xdraw_line_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _y2 = parameters.pop_int()?;
    let _x2 = parameters.pop_int()?;
    let _y1 = parameters.pop_int()?;
    let _x1 = parameters.pop_int()?;
    let _xgc = parameters.pop_long()?;
    let _p_xs_data = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/x11/X11Renderer.XDrawLine(JJIIII)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/x11/X11Renderer.XDrawOval(JJIIII)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn xdraw_oval_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _xgc = parameters.pop_long()?;
    let _p_xs_data = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/x11/X11Renderer.XDrawOval(JJIIII)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/x11/X11Renderer.XDrawPoly(JJII[I[IIZ)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn xdraw_poly_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _isclosed = parameters.pop_bool()?;
    let _npoints = parameters.pop_int()?;
    let _ypoints = parameters.pop_reference()?;
    let _xpoints = parameters.pop_reference()?;
    let _transy = parameters.pop_int()?;
    let _transx = parameters.pop_int()?;
    let _xgc = parameters.pop_long()?;
    let _p_xs_data = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/x11/X11Renderer.XDrawPoly(JJII[I[IIZ)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/x11/X11Renderer.XDrawRect(JJIIII)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn xdraw_rect_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _xgc = parameters.pop_long()?;
    let _p_xs_data = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/x11/X11Renderer.XDrawRect(JJIIII)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/x11/X11Renderer.XDrawRoundRect(JJIIIIII)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn xdraw_round_rect_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arc_h = parameters.pop_int()?;
    let _arc_w = parameters.pop_int()?;
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _xgc = parameters.pop_long()?;
    let _p_xs_data = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/x11/X11Renderer.XDrawRoundRect(JJIIIIII)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/x11/X11Renderer.XFillArc(JJIIIIII)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn xfill_arc_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _angle_extent = parameters.pop_int()?;
    let _angle_start = parameters.pop_int()?;
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _xgc = parameters.pop_long()?;
    let _p_xs_data = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/x11/X11Renderer.XFillArc(JJIIIIII)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/x11/X11Renderer.XFillOval(JJIIII)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn xfill_oval_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _xgc = parameters.pop_long()?;
    let _p_xs_data = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/x11/X11Renderer.XFillOval(JJIIII)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/x11/X11Renderer.XFillPoly(JJII[I[II)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn xfill_poly_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _npoints = parameters.pop_int()?;
    let _ypoints = parameters.pop_reference()?;
    let _xpoints = parameters.pop_reference()?;
    let _transy = parameters.pop_int()?;
    let _transx = parameters.pop_int()?;
    let _xgc = parameters.pop_long()?;
    let _p_xs_data = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/x11/X11Renderer.XFillPoly(JJII[I[II)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/x11/X11Renderer.XFillRect(JJIIII)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn xfill_rect_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _xgc = parameters.pop_long()?;
    let _p_xs_data = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/x11/X11Renderer.XFillRect(JJIIII)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/x11/X11Renderer.XFillRoundRect(JJIIIIII)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn xfill_round_rect_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arc_h = parameters.pop_int()?;
    let _arc_w = parameters.pop_int()?;
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _xgc = parameters.pop_long()?;
    let _p_xs_data = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/x11/X11Renderer.XFillRoundRect(JJIIIIII)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/x11/X11Renderer.XFillSpans(JJLsun/java2d/pipe/SpanIterator;JII)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn xfill_spans_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _transy = parameters.pop_int()?;
    let _transx = parameters.pop_int()?;
    let _iterator = parameters.pop_long()?;
    let _si = parameters.pop_reference()?;
    let _xgc = parameters.pop_long()?;
    let _p_xs_data = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/x11/X11Renderer.XFillSpans(JJLsun/java2d/pipe/SpanIterator;JII)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/x11/X11Renderer.devCopyArea(JJIIIIII)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn dev_copy_area_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _dsty = parameters.pop_int()?;
    let _dstx = parameters.pop_int()?;
    let _srcy = parameters.pop_int()?;
    let _srcx = parameters.pop_int()?;
    let _xgc = parameters.pop_long()?;
    let _sd_ops = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/x11/X11Renderer.devCopyArea(JJIIIIII)V".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_x_do_path() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = x_do_path(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Long(0),
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::from(false),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.x11.X11Renderer.XDoPath(Lsun/java2d/SunGraphics2D;JJIILjava/awt/geom/Path2D$Float;Z)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_x_draw_arc() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = x_draw_arc(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
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
            "sun.java2d.x11.X11Renderer.XDrawArc(JJIIIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_x_draw_line() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = x_draw_line(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.x11.X11Renderer.XDrawLine(JJIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_x_draw_oval() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = x_draw_oval(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.x11.X11Renderer.XDrawOval(JJIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_x_draw_poly() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = x_draw_poly(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
                Value::from(false),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.x11.X11Renderer.XDrawPoly(JJII[I[IIZ)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_x_draw_rect() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = x_draw_rect(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.x11.X11Renderer.XDrawRect(JJIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_x_draw_round_rect() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = x_draw_round_rect(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
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
            "sun.java2d.x11.X11Renderer.XDrawRoundRect(JJIIIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_x_fill_arc() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = x_fill_arc(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
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
            "sun.java2d.x11.X11Renderer.XFillArc(JJIIIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_x_fill_oval() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = x_fill_oval(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.x11.X11Renderer.XFillOval(JJIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_x_fill_poly() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = x_fill_poly(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.x11.X11Renderer.XFillPoly(JJII[I[II)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_x_fill_rect() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = x_fill_rect(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.x11.X11Renderer.XFillRect(JJIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_x_fill_round_rect() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = x_fill_round_rect(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
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
            "sun.java2d.x11.X11Renderer.XFillRoundRect(JJIIIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_x_fill_spans() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = x_fill_spans(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Object(None),
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.x11.X11Renderer.XFillSpans(JJLsun/java2d/pipe/SpanIterator;JII)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_dev_copy_area() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = dev_copy_area(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
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
            "sun.java2d.x11.X11Renderer.devCopyArea(JJIIIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xdo_path_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xdo_path_linux_ge_v11(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Long(0),
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::from(false),
            ]),
        )
        .await;
        assert_eq!(
            "sun/java2d/x11/X11Renderer.XDoPath(Lsun/java2d/SunGraphics2D;JJIILjava/awt/geom/Path2D$Float;Z)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xdraw_arc_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xdraw_arc_linux_ge_v11(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
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
            "sun/java2d/x11/X11Renderer.XDrawArc(JJIIIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xdraw_line_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xdraw_line_linux_ge_v11(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/java2d/x11/X11Renderer.XDrawLine(JJIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xdraw_oval_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xdraw_oval_linux_ge_v11(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/java2d/x11/X11Renderer.XDrawOval(JJIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xdraw_poly_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xdraw_poly_linux_ge_v11(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
                Value::from(false),
            ]),
        )
        .await;
        assert_eq!(
            "sun/java2d/x11/X11Renderer.XDrawPoly(JJII[I[IIZ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xdraw_rect_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xdraw_rect_linux_ge_v11(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/java2d/x11/X11Renderer.XDrawRect(JJIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xdraw_round_rect_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xdraw_round_rect_linux_ge_v11(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
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
            "sun/java2d/x11/X11Renderer.XDrawRoundRect(JJIIIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xfill_arc_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xfill_arc_linux_ge_v11(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
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
            "sun/java2d/x11/X11Renderer.XFillArc(JJIIIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xfill_oval_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xfill_oval_linux_ge_v11(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/java2d/x11/X11Renderer.XFillOval(JJIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xfill_poly_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xfill_poly_linux_ge_v11(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/java2d/x11/X11Renderer.XFillPoly(JJII[I[II)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xfill_rect_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xfill_rect_linux_ge_v11(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/java2d/x11/X11Renderer.XFillRect(JJIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xfill_round_rect_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xfill_round_rect_linux_ge_v11(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
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
            "sun/java2d/x11/X11Renderer.XFillRoundRect(JJIIIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xfill_spans_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xfill_spans_linux_ge_v11(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
                Value::Object(None),
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/java2d/x11/X11Renderer.XFillSpans(JJLsun/java2d/pipe/SpanIterator;JII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_dev_copy_area_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = dev_copy_area_linux_ge_v11(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Long(0),
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
            "sun/java2d/x11/X11Renderer.devCopyArea(JJIIIIII)V",
            result.unwrap_err().to_string()
        );
    }
}
