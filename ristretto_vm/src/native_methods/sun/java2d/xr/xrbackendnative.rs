use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/java2d/xr/XRBackendNative";

/// Register all native methods for `sun.java2d.xr.XRBackendNative`.
#[expect(clippy::too_many_lines)]
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "GCRectanglesNative",
        "(IJ[II)V",
        gc_rectangles_native,
    );
    registry.register(
        CLASS_NAME,
        "XRAddGlyphsNative",
        "(I[JI[BI)V",
        xr_add_glyphs_native,
    );
    registry.register(
        CLASS_NAME,
        "XRCreateLinearGradientPaintNative",
        "([F[SIIIIII)I",
        xr_create_linear_gradient_paint_native,
    );
    registry.register(
        CLASS_NAME,
        "XRCreateRadialGradientPaintNative",
        "([F[SIIIIII)I",
        xr_create_radial_gradient_paint_native,
    );
    registry.register(
        CLASS_NAME,
        "XRFreeGlyphsNative",
        "(I[II)V",
        xr_free_glyphs_native,
    );
    registry.register(
        CLASS_NAME,
        "XRSetClipNative",
        "(JIIIILsun/java2d/pipe/Region;Z)V",
        xr_set_clip_native,
    );
    registry.register(
        CLASS_NAME,
        "XRSetTransformNative",
        "(IIIIIII)V",
        xr_set_transform_native,
    );
    registry.register(
        CLASS_NAME,
        "XRenderCompositeTextNative",
        "(IIIIIJ[I[III)V",
        x_render_composite_text_native,
    );
    registry.register(
        CLASS_NAME,
        "XRenderCreateGlyphSetNative",
        "(J)I",
        x_render_create_glyph_set_native,
    );
    registry.register(
        CLASS_NAME,
        "XRenderRectanglesNative",
        "(IBSSSS[II)V",
        x_render_rectangles_native,
    );
    registry.register(CLASS_NAME, "copyArea", "(IIJIIIIII)V", copy_area);
    registry.register(CLASS_NAME, "createGC", "(I)J", create_gc);
    registry.register(
        CLASS_NAME,
        "createPictureNative",
        "(IJ)I",
        create_picture_native,
    );
    registry.register(CLASS_NAME, "createPixmap", "(IIII)I", create_pixmap);
    registry.register(CLASS_NAME, "freeGC", "(J)V", free_gc);
    registry.register(CLASS_NAME, "freePicture", "(I)V", free_picture);
    registry.register(CLASS_NAME, "freePixmap", "(I)V", free_pixmap);
    registry.register(CLASS_NAME, "initIDs", "()V", init_ids);
    registry.register(
        CLASS_NAME,
        "padBlitNative",
        "(BIIIIIIIIIIIIIIIIIII)V",
        pad_blit_native,
    );
    registry.register(
        CLASS_NAME,
        "putMaskNative",
        "(IJ[BIIIIIIIIFJ)V",
        put_mask_native,
    );
    registry.register(
        CLASS_NAME,
        "renderComposite",
        "(BIIIIIIIIIII)V",
        render_composite,
    );
    registry.register(
        CLASS_NAME,
        "renderCompositeTrapezoidsNative",
        "(BIJIII[I)V",
        render_composite_trapezoids_native,
    );
    registry.register(
        CLASS_NAME,
        "renderRectangle",
        "(IBSSSSIIII)V",
        render_rectangle,
    );
    registry.register(CLASS_NAME, "setFilter", "(II)V", set_filter);
    registry.register(CLASS_NAME, "setGCExposures", "(JZ)V", set_gc_exposures);
    registry.register(CLASS_NAME, "setGCForeground", "(JI)V", set_gc_foreground);
    registry.register(CLASS_NAME, "setGCMode", "(JZ)V", set_gc_mode);
    registry.register(CLASS_NAME, "setPictureRepeat", "(II)V", set_picture_repeat);
}

#[async_recursion(?Send)]
async fn gc_rectangles_native(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRBackendNative.GCRectanglesNative(IJ[II)V");
}

#[async_recursion(?Send)]
async fn xr_add_glyphs_native(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRBackendNative.XRAddGlyphsNative(I[JI[BI)V");
}

#[async_recursion(?Send)]
async fn xr_create_linear_gradient_paint_native(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRBackendNative.XRCreateLinearGradientPaintNative([F[SIIIIII)I");
}

#[async_recursion(?Send)]
async fn xr_create_radial_gradient_paint_native(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRBackendNative.XRCreateRadialGradientPaintNative([F[SIIIIII)I");
}

#[async_recursion(?Send)]
async fn xr_free_glyphs_native(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRBackendNative.XRFreeGlyphsNative(I[II)V");
}

#[async_recursion(?Send)]
async fn xr_set_clip_native(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRBackendNative.XRSetClipNative(JIIIILsun/java2d/pipe/Region;Z)V");
}

#[async_recursion(?Send)]
async fn xr_set_transform_native(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRBackendNative.XRSetTransformNative(IIIIIII)V");
}

#[async_recursion(?Send)]
async fn x_render_composite_text_native(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRBackendNative.XRenderCompositeTextNative(IIIIIJ[I[III)V");
}

#[async_recursion(?Send)]
async fn x_render_create_glyph_set_native(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRBackendNative.XRenderCreateGlyphSetNative(J)I");
}

#[async_recursion(?Send)]
async fn x_render_rectangles_native(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRBackendNative.XRenderRectanglesNative(IBSSSS[II)V");
}

#[async_recursion(?Send)]
async fn copy_area(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRBackendNative.copyArea(IIJIIIIII)V");
}

#[async_recursion(?Send)]
async fn create_gc(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRBackendNative.createGC(I)J");
}

#[async_recursion(?Send)]
async fn create_picture_native(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRBackendNative.createPictureNative(IJ)I");
}

#[async_recursion(?Send)]
async fn create_pixmap(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRBackendNative.createPixmap(IIII)I");
}

#[async_recursion(?Send)]
async fn free_gc(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRBackendNative.freeGC(J)V");
}

#[async_recursion(?Send)]
async fn free_picture(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRBackendNative.freePicture(I)V");
}

#[async_recursion(?Send)]
async fn free_pixmap(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRBackendNative.freePixmap(I)V");
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn pad_blit_native(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRBackendNative.padBlitNative(BIIIIIIIIIIIIIIIIIII)V");
}

#[async_recursion(?Send)]
async fn put_mask_native(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRBackendNative.putMaskNative(IJ[BIIIIIIIIFJ)V");
}

#[async_recursion(?Send)]
async fn render_composite(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRBackendNative.renderComposite(BIIIIIIIIIII)V");
}

#[async_recursion(?Send)]
async fn render_composite_trapezoids_native(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRBackendNative.renderCompositeTrapezoidsNative(BIJIII[I)V");
}

#[async_recursion(?Send)]
async fn render_rectangle(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRBackendNative.renderRectangle(IBSSSSIIII)V");
}

#[async_recursion(?Send)]
async fn set_filter(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRBackendNative.setFilter(II)V");
}

#[async_recursion(?Send)]
async fn set_gc_exposures(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRBackendNative.setGCExposures(JZ)V");
}

#[async_recursion(?Send)]
async fn set_gc_foreground(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRBackendNative.setGCForeground(JI)V");
}

#[async_recursion(?Send)]
async fn set_gc_mode(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRBackendNative.setGCMode(JZ)V");
}

#[async_recursion(?Send)]
async fn set_picture_repeat(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRBackendNative.setPictureRepeat(II)V");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.xr.XRBackendNative.GCRectanglesNative(IJ[II)V"
    )]
    async fn test_gc_rectangles_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = gc_rectangles_native(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.xr.XRBackendNative.XRAddGlyphsNative(I[JI[BI)V"
    )]
    async fn test_xr_add_glyphs_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = xr_add_glyphs_native(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.xr.XRBackendNative.XRCreateLinearGradientPaintNative([F[SIIIIII)I"
    )]
    async fn test_xr_create_linear_gradient_paint_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = xr_create_linear_gradient_paint_native(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.xr.XRBackendNative.XRCreateRadialGradientPaintNative([F[SIIIIII)I"
    )]
    async fn test_xr_create_radial_gradient_paint_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = xr_create_radial_gradient_paint_native(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.xr.XRBackendNative.XRFreeGlyphsNative(I[II)V"
    )]
    async fn test_xr_free_glyphs_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = xr_free_glyphs_native(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.xr.XRBackendNative.XRSetClipNative(JIIIILsun/java2d/pipe/Region;Z)V"
    )]
    async fn test_xr_set_clip_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = xr_set_clip_native(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.xr.XRBackendNative.XRSetTransformNative(IIIIIII)V"
    )]
    async fn test_xr_set_transform_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = xr_set_transform_native(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.xr.XRBackendNative.XRenderCompositeTextNative(IIIIIJ[I[III)V"
    )]
    async fn test_x_render_composite_text_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = x_render_composite_text_native(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.xr.XRBackendNative.XRenderCreateGlyphSetNative(J)I"
    )]
    async fn test_x_render_create_glyph_set_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = x_render_create_glyph_set_native(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.xr.XRBackendNative.XRenderRectanglesNative(IBSSSS[II)V"
    )]
    async fn test_x_render_rectangles_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = x_render_rectangles_native(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.xr.XRBackendNative.copyArea(IIJIIIIII)V"
    )]
    async fn test_copy_area() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = copy_area(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.java2d.xr.XRBackendNative.createGC(I)J")]
    async fn test_create_gc() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create_gc(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.xr.XRBackendNative.createPictureNative(IJ)I"
    )]
    async fn test_create_picture_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create_picture_native(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.xr.XRBackendNative.createPixmap(IIII)I"
    )]
    async fn test_create_pixmap() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create_pixmap(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.java2d.xr.XRBackendNative.freeGC(J)V")]
    async fn test_free_gc() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = free_gc(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.java2d.xr.XRBackendNative.freePicture(I)V")]
    async fn test_free_picture() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = free_picture(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.java2d.xr.XRBackendNative.freePixmap(I)V")]
    async fn test_free_pixmap() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = free_pixmap(thread, Arguments::default()).await;
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Arguments::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.xr.XRBackendNative.padBlitNative(BIIIIIIIIIIIIIIIIIII)V"
    )]
    async fn test_pad_blit_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = pad_blit_native(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.xr.XRBackendNative.putMaskNative(IJ[BIIIIIIIIFJ)V"
    )]
    async fn test_put_mask_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_mask_native(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.xr.XRBackendNative.renderComposite(BIIIIIIIIIII)V"
    )]
    async fn test_render_composite() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = render_composite(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.xr.XRBackendNative.renderCompositeTrapezoidsNative(BIJIII[I)V"
    )]
    async fn test_render_composite_trapezoids_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = render_composite_trapezoids_native(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.xr.XRBackendNative.renderRectangle(IBSSSSIIII)V"
    )]
    async fn test_render_rectangle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = render_rectangle(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.java2d.xr.XRBackendNative.setFilter(II)V")]
    async fn test_set_filter() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_filter(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.xr.XRBackendNative.setGCExposures(JZ)V"
    )]
    async fn test_set_gc_exposures() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_gc_exposures(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.xr.XRBackendNative.setGCForeground(JI)V"
    )]
    async fn test_set_gc_foreground() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_gc_foreground(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.java2d.xr.XRBackendNative.setGCMode(JZ)V")]
    async fn test_set_gc_mode() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_gc_mode(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.xr.XRBackendNative.setPictureRepeat(II)V"
    )]
    async fn test_set_picture_repeat() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_picture_repeat(thread, Arguments::default()).await;
    }
}
