use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.java2d.xr.XRBackendNative`.
#[expect(clippy::too_many_lines)]
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/java2d/xr/XRBackendNative";
    registry.register(
        class_name,
        "GCRectanglesNative",
        "(IJ[II)V",
        gc_rectangles_native,
    );
    registry.register(
        class_name,
        "XRAddGlyphsNative",
        "(I[JI[BI)V",
        xr_add_glyphs_native,
    );
    registry.register(
        class_name,
        "XRCreateLinearGradientPaintNative",
        "([F[SIIIIII)I",
        xr_create_linear_gradient_paint_native,
    );
    registry.register(
        class_name,
        "XRCreateRadialGradientPaintNative",
        "([F[SIIIIII)I",
        xr_create_radial_gradient_paint_native,
    );
    registry.register(
        class_name,
        "XRFreeGlyphsNative",
        "(I[II)V",
        xr_free_glyphs_native,
    );
    registry.register(
        class_name,
        "XRSetClipNative",
        "(JIIIILsun/java2d/pipe/Region;Z)V",
        xr_set_clip_native,
    );
    registry.register(
        class_name,
        "XRSetTransformNative",
        "(IIIIIII)V",
        xr_set_transform_native,
    );
    registry.register(
        class_name,
        "XRenderCompositeTextNative",
        "(IIIIIJ[I[III)V",
        x_render_composite_text_native,
    );
    registry.register(
        class_name,
        "XRenderCreateGlyphSetNative",
        "(J)I",
        x_render_create_glyph_set_native,
    );
    registry.register(
        class_name,
        "XRenderRectanglesNative",
        "(IBSSSS[II)V",
        x_render_rectangles_native,
    );
    registry.register(class_name, "copyArea", "(IIJIIIIII)V", copy_area);
    registry.register(class_name, "createGC", "(I)J", create_gc);
    registry.register(
        class_name,
        "createPictureNative",
        "(IJ)I",
        create_picture_native,
    );
    registry.register(class_name, "createPixmap", "(IIII)I", create_pixmap);
    registry.register(class_name, "freeGC", "(J)V", free_gc);
    registry.register(class_name, "freePicture", "(I)V", free_picture);
    registry.register(class_name, "freePixmap", "(I)V", free_pixmap);
    registry.register(class_name, "initIDs", "()V", init_ids);
    registry.register(
        class_name,
        "padBlitNative",
        "(BIIIIIIIIIIIIIIIIIII)V",
        pad_blit_native,
    );
    registry.register(
        class_name,
        "putMaskNative",
        "(IJ[BIIIIIIIIFJ)V",
        put_mask_native,
    );
    registry.register(
        class_name,
        "renderComposite",
        "(BIIIIIIIIIII)V",
        render_composite,
    );
    registry.register(
        class_name,
        "renderCompositeTrapezoidsNative",
        "(BIJIII[I)V",
        render_composite_trapezoids_native,
    );
    registry.register(
        class_name,
        "renderRectangle",
        "(IBSSSSIIII)V",
        render_rectangle,
    );
    registry.register(class_name, "setFilter", "(II)V", set_filter);
    registry.register(class_name, "setGCExposures", "(JZ)V", set_gc_exposures);
    registry.register(class_name, "setGCForeground", "(JI)V", set_gc_foreground);
    registry.register(class_name, "setGCMode", "(JZ)V", set_gc_mode);
    registry.register(class_name, "setPictureRepeat", "(II)V", set_picture_repeat);
}

#[async_recursion(?Send)]
async fn gc_rectangles_native(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn xr_add_glyphs_native(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn xr_create_linear_gradient_paint_native(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn xr_create_radial_gradient_paint_native(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn xr_free_glyphs_native(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn xr_set_clip_native(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn xr_set_transform_native(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn x_render_composite_text_native(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn x_render_create_glyph_set_native(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn x_render_rectangles_native(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn copy_area(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn create_gc(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn create_picture_native(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn create_pixmap(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn free_gc(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn free_picture(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn free_pixmap(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn pad_blit_native(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn put_mask_native(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn render_composite(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn render_composite_trapezoids_native(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn render_rectangle(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn set_filter(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn set_gc_exposures(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn set_gc_foreground(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn set_gc_mode(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn set_picture_repeat(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}
