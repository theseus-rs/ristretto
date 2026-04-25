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
    "sun/java2d/xr/XRBackendNative.GCRectanglesNative(IJ[II)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn gc_rectangles_native<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _rect_cnt = parameters.pop_int()?;
    let _rect_array = parameters.pop_reference()?;
    let _gc = parameters.pop_long()?;
    let _drawable = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.xr.XRBackendNative.GCRectanglesNative(IJ[II)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.XRAddGlyphsNative(I[JI[BI)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn xr_add_glyphs_native<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _pixel_data_length = parameters.pop_int()?;
    let _pixel_data = parameters.pop_reference()?;
    let _glyph_cnt = parameters.pop_int()?;
    let _glyph_info_ptrs = parameters.pop_reference()?;
    let _glyph_set = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.xr.XRBackendNative.XRAddGlyphsNative(I[JI[BI)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.XRCreateLinearGradientPaintNative([F[SIIIIII)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn xr_create_linear_gradient_paint_native<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _repeat = parameters.pop_int()?;
    let _num_stops = parameters.pop_int()?;
    let _y2 = parameters.pop_int()?;
    let _x2 = parameters.pop_int()?;
    let _y1 = parameters.pop_int()?;
    let _x1 = parameters.pop_int()?;
    let _pixels_array = parameters.pop_reference()?;
    let _fractions_array = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.xr.XRBackendNative.XRCreateLinearGradientPaintNative([F[SIIIIII)I".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.XRCreateRadialGradientPaintNative([F[SIIIIII)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn xr_create_radial_gradient_paint_native<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _repeat = parameters.pop_int()?;
    let _outer_radius = parameters.pop_int()?;
    let _inner_radius = parameters.pop_int()?;
    let _center_y = parameters.pop_int()?;
    let _center_x = parameters.pop_int()?;
    let _num_stops = parameters.pop_int()?;
    let _pixels_array = parameters.pop_reference()?;
    let _fractions_array = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.xr.XRBackendNative.XRCreateRadialGradientPaintNative([F[SIIIIII)I".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.XRFreeGlyphsNative(I[II)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn xr_free_glyphs_native<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _id_cnt = parameters.pop_int()?;
    let _gids = parameters.pop_reference()?;
    let _glyph_set = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.xr.XRBackendNative.XRFreeGlyphsNative(I[II)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.XRSetClipNative(JIIIILsun/java2d/pipe/Region;Z)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn xr_set_clip_native<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _is_gc = parameters.pop_bool()?;
    let _complexclip = parameters.pop_reference()?;
    let _y2 = parameters.pop_int()?;
    let _x2 = parameters.pop_int()?;
    let _y1 = parameters.pop_int()?;
    let _x1 = parameters.pop_int()?;
    let _dst = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.xr.XRBackendNative.XRSetClipNative(JIIIILsun/java2d/pipe/Region;Z)V"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.XRSetTransformNative(IIIIIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn xr_set_transform_native<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _m12 = parameters.pop_int()?;
    let _m11 = parameters.pop_int()?;
    let _m10 = parameters.pop_int()?;
    let _m02 = parameters.pop_int()?;
    let _m01 = parameters.pop_int()?;
    let _m00 = parameters.pop_int()?;
    let _pic = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.xr.XRBackendNative.XRSetTransformNative(IIIIIII)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.XRenderCompositeTextNative(IIIIIJ[I[III)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn x_render_composite_text_native<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _glyph_cnt = parameters.pop_int()?;
    let _elt_cnt = parameters.pop_int()?;
    let _glyph_i_ds = parameters.pop_reference()?;
    let _elt_array = parameters.pop_reference()?;
    let _mask_format = parameters.pop_long()?;
    let _src_y = parameters.pop_int()?;
    let _src_x = parameters.pop_int()?;
    let _dst = parameters.pop_int()?;
    let _src = parameters.pop_int()?;
    let _op = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.xr.XRBackendNative.XRenderCompositeTextNative(IIIIIJ[I[III)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.XRenderCreateGlyphSetNative(J)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn x_render_create_glyph_set_native<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _format = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.xr.XRBackendNative.XRenderCreateGlyphSetNative(J)I".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.XRenderRectanglesNative(IBSSSS[II)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn x_render_rectangles_native<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _rect_cnt = parameters.pop_int()?;
    let _rects = parameters.pop_reference()?;
    let _alpha = parameters.pop_int()?;
    let _blue = parameters.pop_int()?;
    let _green = parameters.pop_int()?;
    let _red = parameters.pop_int()?;
    let _op = parameters.pop_int()?;
    let _dst = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.xr.XRBackendNative.XRenderRectanglesNative(IBSSSS[II)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.copyArea(IIJIIIIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn copy_area<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _dsty = parameters.pop_int()?;
    let _dstx = parameters.pop_int()?;
    let _height = parameters.pop_int()?;
    let _width = parameters.pop_int()?;
    let _srcy = parameters.pop_int()?;
    let _srcx = parameters.pop_int()?;
    let _gc = parameters.pop_long()?;
    let _dst = parameters.pop_int()?;
    let _src = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.xr.XRBackendNative.copyArea(IIJIIIIII)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/java2d/xr/XRBackendNative.createGC(I)J", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn create_gc<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _drawable = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun.java2d.xr.XRBackendNative.createGC(I)J".to_string())
            .into(),
    )
}

#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.createPictureNative(IJ)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn create_picture_native<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _format_id = parameters.pop_long()?;
    let _drawable = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.xr.XRBackendNative.createPictureNative(IJ)I".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.createPixmap(IIII)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn create_pixmap<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _height = parameters.pop_int()?;
    let _width = parameters.pop_int()?;
    let _depth = parameters.pop_int()?;
    let _drawable = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.xr.XRBackendNative.createPixmap(IIII)I".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/java2d/xr/XRBackendNative.freeGC(J)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn free_gc<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _gc = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun.java2d.xr.XRBackendNative.freeGC(J)V".to_string())
            .into(),
    )
}

#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.freePicture(I)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn free_picture<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _picture = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.xr.XRBackendNative.freePicture(I)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.freePixmap(I)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn free_pixmap<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _pixmap = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun.java2d.xr.XRBackendNative.freePixmap(I)V".to_string())
            .into(),
    )
}

#[intrinsic_method("sun/java2d/xr/XRBackendNative.initIDs()V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.padBlitNative(BIIIIIIIIIIIIIIIIIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn pad_blit_native<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _dy = parameters.pop_int()?;
    let _dx = parameters.pop_int()?;
    let _sy = parameters.pop_int()?;
    let _sx = parameters.pop_int()?;
    let _last_mask_height = parameters.pop_int()?;
    let _last_mask_width = parameters.pop_int()?;
    let _mask_height = parameters.pop_int()?;
    let _mask_width = parameters.pop_int()?;
    let _m12 = parameters.pop_int()?;
    let _m11 = parameters.pop_int()?;
    let _m10 = parameters.pop_int()?;
    let _m02 = parameters.pop_int()?;
    let _m01 = parameters.pop_int()?;
    let _m00 = parameters.pop_int()?;
    let _dst_pict = parameters.pop_int()?;
    let _mask_pict = parameters.pop_int()?;
    let _src_pict = parameters.pop_int()?;
    let _op = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.xr.XRBackendNative.padBlitNative(BIIIIIIIIIIIIIIIIIII)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.putMaskNative(IJ[BIIIIIIIIFJ)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn put_mask_native<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _x_img = parameters.pop_long()?;
    let _ea = parameters.pop_float()?;
    let _mask_scan = parameters.pop_int()?;
    let _mask_off = parameters.pop_int()?;
    let _height = parameters.pop_int()?;
    let _width = parameters.pop_int()?;
    let _dy = parameters.pop_int()?;
    let _dx = parameters.pop_int()?;
    let _sy = parameters.pop_int()?;
    let _sx = parameters.pop_int()?;
    let _image_data = parameters.pop_reference()?;
    let _gc = parameters.pop_long()?;
    let _drawable = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.xr.XRBackendNative.putMaskNative(IJ[BIIIIIIIIFJ)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.renderComposite(BIIIIIIIIIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn render_composite<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _height = parameters.pop_int()?;
    let _width = parameters.pop_int()?;
    let _dst_y = parameters.pop_int()?;
    let _dst_x = parameters.pop_int()?;
    let _mask_y = parameters.pop_int()?;
    let _mask_x = parameters.pop_int()?;
    let _src_y = parameters.pop_int()?;
    let _src_x = parameters.pop_int()?;
    let _dst = parameters.pop_int()?;
    let _mask = parameters.pop_int()?;
    let _src = parameters.pop_int()?;
    let _op = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.xr.XRBackendNative.renderComposite(BIIIIIIIIIII)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.renderCompositeTrapezoidsNative(BIJIII[I)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn render_composite_trapezoids_native<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _trapezoids = parameters.pop_reference()?;
    let _src_y = parameters.pop_int()?;
    let _src_x = parameters.pop_int()?;
    let _dst = parameters.pop_int()?;
    let _mask_format = parameters.pop_long()?;
    let _src = parameters.pop_int()?;
    let _op = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.xr.XRBackendNative.renderCompositeTrapezoidsNative(BIJIII[I)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.renderRectangle(IBSSSSIIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn render_rectangle<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _height = parameters.pop_int()?;
    let _width = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _alpha = parameters.pop_int()?;
    let _blue = parameters.pop_int()?;
    let _green = parameters.pop_int()?;
    let _red = parameters.pop_int()?;
    let _op = parameters.pop_int()?;
    let _dst = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.xr.XRBackendNative.renderRectangle(IBSSSSIIII)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.setFilter(II)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn set_filter<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _filter = parameters.pop_int()?;
    let _picture = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun.java2d.xr.XRBackendNative.setFilter(II)V".to_string())
            .into(),
    )
}

#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.setGCExposures(JZ)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn set_gc_exposures<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _exposure = parameters.pop_bool()?;
    let _gc = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.xr.XRBackendNative.setGCExposures(JZ)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.setGCForeground(JI)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn set_gc_foreground<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _pixel = parameters.pop_int()?;
    let _gc = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.xr.XRBackendNative.setGCForeground(JI)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.setGCMode(JZ)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn set_gc_mode<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _copy = parameters.pop_bool()?;
    let _gc = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun.java2d.xr.XRBackendNative.setGCMode(JZ)V".to_string())
            .into(),
    )
}

#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.setPictureRepeat(II)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn set_picture_repeat<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _repeat = parameters.pop_int()?;
    let _picture = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.xr.XRBackendNative.setPictureRepeat(II)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.GCRectanglesNative(IJ[II)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn gcrectangles_native_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _rect_cnt = parameters.pop_int()?;
    let _rect_array = parameters.pop_reference()?;
    let _gc = parameters.pop_long()?;
    let _drawable = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/xr/XRBackendNative.GCRectanglesNative(IJ[II)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.XRAddGlyphsNative(I[JI[BI)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn xradd_glyphs_native_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _pixel_data_length = parameters.pop_int()?;
    let _pixel_data = parameters.pop_reference()?;
    let _glyph_cnt = parameters.pop_int()?;
    let _glyph_info_ptrs = parameters.pop_reference()?;
    let _glyph_set = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/xr/XRBackendNative.XRAddGlyphsNative(I[JI[BI)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.XRCreateLinearGradientPaintNative([F[SIIIIII)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn xrcreate_linear_gradient_paint_native_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _repeat = parameters.pop_int()?;
    let _num_stops = parameters.pop_int()?;
    let _y2 = parameters.pop_int()?;
    let _x2 = parameters.pop_int()?;
    let _y1 = parameters.pop_int()?;
    let _x1 = parameters.pop_int()?;
    let _pixels_array = parameters.pop_reference()?;
    let _fractions_array = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/xr/XRBackendNative.XRCreateLinearGradientPaintNative([F[SIIIIII)I".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.XRCreateRadialGradientPaintNative([F[SIIIIII)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn xrcreate_radial_gradient_paint_native_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _repeat = parameters.pop_int()?;
    let _outer_radius = parameters.pop_int()?;
    let _inner_radius = parameters.pop_int()?;
    let _center_y = parameters.pop_int()?;
    let _center_x = parameters.pop_int()?;
    let _num_stops = parameters.pop_int()?;
    let _pixels_array = parameters.pop_reference()?;
    let _fractions_array = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/xr/XRBackendNative.XRCreateRadialGradientPaintNative([F[SIIIIII)I".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.XRFreeGlyphsNative(I[II)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn xrfree_glyphs_native_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _id_cnt = parameters.pop_int()?;
    let _gids = parameters.pop_reference()?;
    let _glyph_set = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/xr/XRBackendNative.XRFreeGlyphsNative(I[II)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.XRSetClipNative(JIIIILsun/java2d/pipe/Region;Z)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn xrset_clip_native_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _is_gc = parameters.pop_bool()?;
    let _complexclip = parameters.pop_reference()?;
    let _y2 = parameters.pop_int()?;
    let _x2 = parameters.pop_int()?;
    let _y1 = parameters.pop_int()?;
    let _x1 = parameters.pop_int()?;
    let _dst = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/xr/XRBackendNative.XRSetClipNative(JIIIILsun/java2d/pipe/Region;Z)V"
            .to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.XRSetTransformNative(IIIIIII)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn xrset_transform_native_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _m12 = parameters.pop_int()?;
    let _m11 = parameters.pop_int()?;
    let _m10 = parameters.pop_int()?;
    let _m02 = parameters.pop_int()?;
    let _m01 = parameters.pop_int()?;
    let _m00 = parameters.pop_int()?;
    let _pic = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/xr/XRBackendNative.XRSetTransformNative(IIIIIII)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.XRenderCompositeTextNative(IIIIIJ[I[III)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn xrender_composite_text_native_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _glyph_cnt = parameters.pop_int()?;
    let _elt_cnt = parameters.pop_int()?;
    let _glyph_i_ds = parameters.pop_reference()?;
    let _elt_array = parameters.pop_reference()?;
    let _mask_format = parameters.pop_long()?;
    let _src_y = parameters.pop_int()?;
    let _src_x = parameters.pop_int()?;
    let _dst = parameters.pop_int()?;
    let _src = parameters.pop_int()?;
    let _op = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/xr/XRBackendNative.XRenderCompositeTextNative(IIIIIJ[I[III)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.XRenderCreateGlyphSetNative(J)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn xrender_create_glyph_set_native_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _format = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/xr/XRBackendNative.XRenderCreateGlyphSetNative(J)I".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.XRenderRectanglesNative(IBSSSS[II)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn xrender_rectangles_native_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _rect_cnt = parameters.pop_int()?;
    let _rects = parameters.pop_reference()?;
    let _alpha = parameters.pop_int()?;
    let _blue = parameters.pop_int()?;
    let _green = parameters.pop_int()?;
    let _red = parameters.pop_int()?;
    let _op = parameters.pop_int()?;
    let _dst = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/xr/XRBackendNative.XRenderRectanglesNative(IBSSSS[II)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.copyArea(IIJIIIIII)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn copy_area_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _dsty = parameters.pop_int()?;
    let _dstx = parameters.pop_int()?;
    let _height = parameters.pop_int()?;
    let _width = parameters.pop_int()?;
    let _srcy = parameters.pop_int()?;
    let _srcx = parameters.pop_int()?;
    let _gc = parameters.pop_long()?;
    let _dst = parameters.pop_int()?;
    let _src = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/xr/XRBackendNative.copyArea(IIJIIIIII)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.createGC(I)J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn create_gc_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _drawable = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/java2d/xr/XRBackendNative.createGC(I)J".to_string())
            .into(),
    )
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.createPictureNative(IJ)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn create_picture_native_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _format_id = parameters.pop_long()?;
    let _drawable = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/xr/XRBackendNative.createPictureNative(IJ)I".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.createPixmap(IIII)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn create_pixmap_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _height = parameters.pop_int()?;
    let _width = parameters.pop_int()?;
    let _depth = parameters.pop_int()?;
    let _drawable = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/xr/XRBackendNative.createPixmap(IIII)I".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.freeGC(J)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn free_gc_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _gc = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/java2d/xr/XRBackendNative.freeGC(J)V".to_string())
            .into(),
    )
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.freePicture(I)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn free_picture_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _picture = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/xr/XRBackendNative.freePicture(I)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.freePixmap(I)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn free_pixmap_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _pixmap = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/java2d/xr/XRBackendNative.freePixmap(I)V".to_string())
            .into(),
    )
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.initIDs()V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn init_ids_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/java2d/xr/XRBackendNative.initIDs()V".to_string())
            .into(),
    )
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.padBlitNative(BIIIIIIIIIIIIIIIIIII)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn pad_blit_native_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _h = parameters.pop_int()?;
    let _w = parameters.pop_int()?;
    let _dy = parameters.pop_int()?;
    let _dx = parameters.pop_int()?;
    let _sy = parameters.pop_int()?;
    let _sx = parameters.pop_int()?;
    let _last_mask_height = parameters.pop_int()?;
    let _last_mask_width = parameters.pop_int()?;
    let _mask_height = parameters.pop_int()?;
    let _mask_width = parameters.pop_int()?;
    let _m12 = parameters.pop_int()?;
    let _m11 = parameters.pop_int()?;
    let _m10 = parameters.pop_int()?;
    let _m02 = parameters.pop_int()?;
    let _m01 = parameters.pop_int()?;
    let _m00 = parameters.pop_int()?;
    let _dst_pict = parameters.pop_int()?;
    let _mask_pict = parameters.pop_int()?;
    let _src_pict = parameters.pop_int()?;
    let _op = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/xr/XRBackendNative.padBlitNative(BIIIIIIIIIIIIIIIIIII)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.putMaskNative(IJ[BIIIIIIIIFJ)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn put_mask_native_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _x_img = parameters.pop_long()?;
    let _ea = parameters.pop_float()?;
    let _mask_scan = parameters.pop_int()?;
    let _mask_off = parameters.pop_int()?;
    let _height = parameters.pop_int()?;
    let _width = parameters.pop_int()?;
    let _dy = parameters.pop_int()?;
    let _dx = parameters.pop_int()?;
    let _sy = parameters.pop_int()?;
    let _sx = parameters.pop_int()?;
    let _image_data = parameters.pop_reference()?;
    let _gc = parameters.pop_long()?;
    let _drawable = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/xr/XRBackendNative.putMaskNative(IJ[BIIIIIIIIFJ)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.renderComposite(BIIIIIIIIIII)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn render_composite_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _height = parameters.pop_int()?;
    let _width = parameters.pop_int()?;
    let _dst_y = parameters.pop_int()?;
    let _dst_x = parameters.pop_int()?;
    let _mask_y = parameters.pop_int()?;
    let _mask_x = parameters.pop_int()?;
    let _src_y = parameters.pop_int()?;
    let _src_x = parameters.pop_int()?;
    let _dst = parameters.pop_int()?;
    let _mask = parameters.pop_int()?;
    let _src = parameters.pop_int()?;
    let _op = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/xr/XRBackendNative.renderComposite(BIIIIIIIIIII)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.renderRectangle(IBSSSSIIII)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn render_rectangle_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _height = parameters.pop_int()?;
    let _width = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _alpha = parameters.pop_int()?;
    let _blue = parameters.pop_int()?;
    let _green = parameters.pop_int()?;
    let _red = parameters.pop_int()?;
    let _op = parameters.pop_int()?;
    let _dst = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/xr/XRBackendNative.renderRectangle(IBSSSSIIII)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.setFilter(II)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn set_filter_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _filter = parameters.pop_int()?;
    let _picture = parameters.pop_int()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/java2d/xr/XRBackendNative.setFilter(II)V".to_string())
            .into(),
    )
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.setGCExposures(JZ)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn set_gcexposures_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _exposure = parameters.pop_bool()?;
    let _gc = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/xr/XRBackendNative.setGCExposures(JZ)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.setGCForeground(JI)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn set_gcforeground_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _pixel = parameters.pop_int()?;
    let _gc = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/xr/XRBackendNative.setGCForeground(JI)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.setGCMode(JZ)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn set_gcmode_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _copy = parameters.pop_bool()?;
    let _gc = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/java2d/xr/XRBackendNative.setGCMode(JZ)V".to_string())
            .into(),
    )
}
#[cfg(target_os = "linux")]
#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.setPictureRepeat(II)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn set_picture_repeat_linux_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _repeat = parameters.pop_int()?;
    let _picture = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/java2d/xr/XRBackendNative.setPictureRepeat(II)V".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_gc_rectangles_native() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = gc_rectangles_native(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Long(0),
                Value::Object(None),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.xr.XRBackendNative.GCRectanglesNative(IJ[II)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_xr_add_glyphs_native() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = xr_add_glyphs_native(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Object(None),
                Value::Int(0),
                Value::Object(None),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.xr.XRBackendNative.XRAddGlyphsNative(I[JI[BI)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_xr_create_linear_gradient_paint_native() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = xr_create_linear_gradient_paint_native(
            thread,
            Parameters::new(vec![
                Value::Object(None),
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
            "sun.java2d.xr.XRBackendNative.XRCreateLinearGradientPaintNative([F[SIIIIII)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_xr_create_radial_gradient_paint_native() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = xr_create_radial_gradient_paint_native(
            thread,
            Parameters::new(vec![
                Value::Object(None),
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
            "sun.java2d.xr.XRBackendNative.XRCreateRadialGradientPaintNative([F[SIIIIII)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_xr_free_glyphs_native() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = xr_free_glyphs_native(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Object(None), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun.java2d.xr.XRBackendNative.XRFreeGlyphsNative(I[II)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_xr_set_clip_native() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = xr_set_clip_native(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::from(false),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.xr.XRBackendNative.XRSetClipNative(JIIIILsun/java2d/pipe/Region;Z)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_xr_set_transform_native() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = xr_set_transform_native(
            thread,
            Parameters::new(vec![
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
            "sun.java2d.xr.XRBackendNative.XRSetTransformNative(IIIIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_x_render_composite_text_native() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = x_render_composite_text_native(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Long(0),
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.xr.XRBackendNative.XRenderCompositeTextNative(IIIIIJ[I[III)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_x_render_create_glyph_set_native() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result =
            x_render_create_glyph_set_native(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.java2d.xr.XRBackendNative.XRenderCreateGlyphSetNative(J)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_x_render_rectangles_native() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = x_render_rectangles_native(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.xr.XRBackendNative.XRenderRectanglesNative(IBSSSS[II)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_copy_area() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = copy_area(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
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
            "sun.java2d.xr.XRBackendNative.copyArea(IIJIIIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_create_gc() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = create_gc(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun.java2d.xr.XRBackendNative.createGC(I)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_create_picture_native() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result =
            create_picture_native(thread, Parameters::new(vec![Value::Int(0), Value::Long(0)]))
                .await;
        assert_eq!(
            "sun.java2d.xr.XRBackendNative.createPictureNative(IJ)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_create_pixmap() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = create_pixmap(
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
            "sun.java2d.xr.XRBackendNative.createPixmap(IIII)I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_free_gc() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = free_gc(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.java2d.xr.XRBackendNative.freeGC(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_free_picture() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = free_picture(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun.java2d.xr.XRBackendNative.freePicture(I)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_free_pixmap() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = free_pixmap(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun.java2d.xr.XRBackendNative.freePixmap(I)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::java8_thread().await?;
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_pad_blit_native() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = pad_blit_native(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
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
            "sun.java2d.xr.XRBackendNative.padBlitNative(BIIIIIIIIIIIIIIIIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_put_mask_native() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = put_mask_native(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Long(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Float(0.0),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.xr.XRBackendNative.putMaskNative(IJ[BIIIIIIIIFJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_render_composite() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = render_composite(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
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
            "sun.java2d.xr.XRBackendNative.renderComposite(BIIIIIIIIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_render_composite_trapezoids_native() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = render_composite_trapezoids_native(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun.java2d.xr.XRBackendNative.renderCompositeTrapezoidsNative(BIJIII[I)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_render_rectangle() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = render_rectangle(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
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
            "sun.java2d.xr.XRBackendNative.renderRectangle(IBSSSSIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_set_filter() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = set_filter(thread, Parameters::new(vec![Value::Int(0), Value::Int(0)])).await;
        assert_eq!(
            "sun.java2d.xr.XRBackendNative.setFilter(II)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_set_gc_exposures() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = set_gc_exposures(
            thread,
            Parameters::new(vec![Value::Long(0), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "sun.java2d.xr.XRBackendNative.setGCExposures(JZ)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_set_gc_foreground() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result =
            set_gc_foreground(thread, Parameters::new(vec![Value::Long(0), Value::Int(0)])).await;
        assert_eq!(
            "sun.java2d.xr.XRBackendNative.setGCForeground(JI)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_set_gc_mode() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result = set_gc_mode(
            thread,
            Parameters::new(vec![Value::Long(0), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "sun.java2d.xr.XRBackendNative.setGCMode(JZ)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_set_picture_repeat() {
        let (_vm, thread) = crate::test::java8_thread().await.expect("thread");
        let result =
            set_picture_repeat(thread, Parameters::new(vec![Value::Int(0), Value::Int(0)])).await;
        assert_eq!(
            "sun.java2d.xr.XRBackendNative.setPictureRepeat(II)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_gcrectangles_native_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = gcrectangles_native_linux_ge_v11(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Long(0),
                Value::Object(None),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/java2d/xr/XRBackendNative.GCRectanglesNative(IJ[II)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xradd_glyphs_native_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xradd_glyphs_native_linux_ge_v11(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Object(None),
                Value::Int(0),
                Value::Object(None),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/java2d/xr/XRBackendNative.XRAddGlyphsNative(I[JI[BI)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xrcreate_linear_gradient_paint_native_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xrcreate_linear_gradient_paint_native_linux_ge_v11(
            thread,
            Parameters::new(vec![
                Value::Object(None),
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
            "sun/java2d/xr/XRBackendNative.XRCreateLinearGradientPaintNative([F[SIIIIII)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xrcreate_radial_gradient_paint_native_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xrcreate_radial_gradient_paint_native_linux_ge_v11(
            thread,
            Parameters::new(vec![
                Value::Object(None),
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
            "sun/java2d/xr/XRBackendNative.XRCreateRadialGradientPaintNative([F[SIIIIII)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xrfree_glyphs_native_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xrfree_glyphs_native_linux_ge_v11(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Object(None), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/java2d/xr/XRBackendNative.XRFreeGlyphsNative(I[II)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xrset_clip_native_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xrset_clip_native_linux_ge_v11(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::from(false),
            ]),
        )
        .await;
        assert_eq!(
            "sun/java2d/xr/XRBackendNative.XRSetClipNative(JIIIILsun/java2d/pipe/Region;Z)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xrset_transform_native_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xrset_transform_native_linux_ge_v11(
            thread,
            Parameters::new(vec![
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
            "sun/java2d/xr/XRBackendNative.XRSetTransformNative(IIIIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xrender_composite_text_native_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xrender_composite_text_native_linux_ge_v11(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Long(0),
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/java2d/xr/XRBackendNative.XRenderCompositeTextNative(IIIIIJ[I[III)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xrender_create_glyph_set_native_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xrender_create_glyph_set_native_linux_ge_v11(
            thread,
            Parameters::new(vec![Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/java2d/xr/XRBackendNative.XRenderCreateGlyphSetNative(J)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_xrender_rectangles_native_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xrender_rectangles_native_linux_ge_v11(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Object(None),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/java2d/xr/XRBackendNative.XRenderRectanglesNative(IBSSSS[II)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_copy_area_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = copy_area_linux_ge_v11(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
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
            "sun/java2d/xr/XRBackendNative.copyArea(IIJIIIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_create_gc_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create_gc_linux_ge_v11(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/java2d/xr/XRBackendNative.createGC(I)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_create_picture_native_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create_picture_native_linux_ge_v11(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun/java2d/xr/XRBackendNative.createPictureNative(IJ)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_create_pixmap_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create_pixmap_linux_ge_v11(
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
            "sun/java2d/xr/XRBackendNative.createPixmap(IIII)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_free_gc_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = free_gc_linux_ge_v11(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/java2d/xr/XRBackendNative.freeGC(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_free_picture_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = free_picture_linux_ge_v11(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/java2d/xr/XRBackendNative.freePicture(I)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_free_pixmap_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = free_pixmap_linux_ge_v11(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "sun/java2d/xr/XRBackendNative.freePixmap(I)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_init_ids_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ids_linux_ge_v11(thread, Parameters::default()).await;
        assert_eq!(
            "sun/java2d/xr/XRBackendNative.initIDs()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_pad_blit_native_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = pad_blit_native_linux_ge_v11(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
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
            "sun/java2d/xr/XRBackendNative.padBlitNative(BIIIIIIIIIIIIIIIIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_put_mask_native_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = put_mask_native_linux_ge_v11(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Long(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Float(0.0),
                Value::Long(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/java2d/xr/XRBackendNative.putMaskNative(IJ[BIIIIIIIIFJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_render_composite_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = render_composite_linux_ge_v11(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
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
            "sun/java2d/xr/XRBackendNative.renderComposite(BIIIIIIIIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_render_rectangle_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = render_rectangle_linux_ge_v11(
            thread,
            Parameters::new(vec![
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
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
            "sun/java2d/xr/XRBackendNative.renderRectangle(IBSSSSIIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_set_filter_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            set_filter_linux_ge_v11(thread, Parameters::new(vec![Value::Int(0), Value::Int(0)]))
                .await;
        assert_eq!(
            "sun/java2d/xr/XRBackendNative.setFilter(II)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_set_gcexposures_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_gcexposures_linux_ge_v11(
            thread,
            Parameters::new(vec![Value::Long(0), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "sun/java2d/xr/XRBackendNative.setGCExposures(JZ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_set_gcforeground_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_gcforeground_linux_ge_v11(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/java2d/xr/XRBackendNative.setGCForeground(JI)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_set_gcmode_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_gcmode_linux_ge_v11(
            thread,
            Parameters::new(vec![Value::Long(0), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "sun/java2d/xr/XRBackendNative.setGCMode(JZ)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_set_picture_repeat_linux_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_picture_repeat_linux_ge_v11(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/java2d/xr/XRBackendNative.setPictureRepeat(II)V",
            result.unwrap_err().to_string()
        );
    }
}
