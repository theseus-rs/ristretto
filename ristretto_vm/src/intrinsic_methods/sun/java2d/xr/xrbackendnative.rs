use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.GCRectanglesNative(IJ[II)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn gc_rectangles_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRBackendNative.GCRectanglesNative(IJ[II)V");
}

#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.XRAddGlyphsNative(I[JI[BI)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn xr_add_glyphs_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRBackendNative.XRAddGlyphsNative(I[JI[BI)V");
}

#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.XRCreateLinearGradientPaintNative([F[SIIIIII)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn xr_create_linear_gradient_paint_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRBackendNative.XRCreateLinearGradientPaintNative([F[SIIIIII)I");
}

#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.XRCreateRadialGradientPaintNative([F[SIIIIII)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn xr_create_radial_gradient_paint_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRBackendNative.XRCreateRadialGradientPaintNative([F[SIIIIII)I");
}

#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.XRFreeGlyphsNative(I[II)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn xr_free_glyphs_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRBackendNative.XRFreeGlyphsNative(I[II)V");
}

#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.XRSetClipNative(JIIIILsun/java2d/pipe/Region;Z)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn xr_set_clip_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRBackendNative.XRSetClipNative(JIIIILsun/java2d/pipe/Region;Z)V");
}

#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.XRSetTransformNative(IIIIIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn xr_set_transform_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRBackendNative.XRSetTransformNative(IIIIIII)V");
}

#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.XRenderCompositeTextNative(IIIIIJ[I[III)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn x_render_composite_text_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRBackendNative.XRenderCompositeTextNative(IIIIIJ[I[III)V");
}

#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.XRenderCreateGlyphSetNative(J)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn x_render_create_glyph_set_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRBackendNative.XRenderCreateGlyphSetNative(J)I");
}

#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.XRenderRectanglesNative(IBSSSS[II)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn x_render_rectangles_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRBackendNative.XRenderRectanglesNative(IBSSSS[II)V");
}

#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.copyArea(IIJIIIIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn copy_area(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRBackendNative.copyArea(IIJIIIIII)V");
}

#[intrinsic_method("sun/java2d/xr/XRBackendNative.createGC(I)J", LessThanOrEqual(JAVA_8))]
#[async_method]
pub(crate) async fn create_gc(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRBackendNative.createGC(I)J");
}

#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.createPictureNative(IJ)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn create_picture_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRBackendNative.createPictureNative(IJ)I");
}

#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.createPixmap(IIII)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn create_pixmap(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRBackendNative.createPixmap(IIII)I");
}

#[intrinsic_method("sun/java2d/xr/XRBackendNative.freeGC(J)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub(crate) async fn free_gc(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRBackendNative.freeGC(J)V");
}

#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.freePicture(I)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn free_picture(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRBackendNative.freePicture(I)V");
}

#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.freePixmap(I)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn free_pixmap(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRBackendNative.freePixmap(I)V");
}

#[intrinsic_method("sun/java2d/xr/XRBackendNative.initIDs()V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub(crate) async fn init_ids(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.padBlitNative(BIIIIIIIIIIIIIIIIIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn pad_blit_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRBackendNative.padBlitNative(BIIIIIIIIIIIIIIIIIII)V");
}

#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.putMaskNative(IJ[BIIIIIIIIFJ)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn put_mask_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRBackendNative.putMaskNative(IJ[BIIIIIIIIFJ)V");
}

#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.renderComposite(BIIIIIIIIIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn render_composite(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRBackendNative.renderComposite(BIIIIIIIIIII)V");
}

#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.renderCompositeTrapezoidsNative(BIJIII[I)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn render_composite_trapezoids_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRBackendNative.renderCompositeTrapezoidsNative(BIJIII[I)V");
}

#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.renderRectangle(IBSSSSIIII)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn render_rectangle(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRBackendNative.renderRectangle(IBSSSSIIII)V");
}

#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.setFilter(II)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn set_filter(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRBackendNative.setFilter(II)V");
}

#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.setGCExposures(JZ)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn set_gc_exposures(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRBackendNative.setGCExposures(JZ)V");
}

#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.setGCForeground(JI)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn set_gc_foreground(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRBackendNative.setGCForeground(JI)V");
}

#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.setGCMode(JZ)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn set_gc_mode(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRBackendNative.setGCMode(JZ)V");
}

#[intrinsic_method(
    "sun/java2d/xr/XRBackendNative.setPictureRepeat(II)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn set_picture_repeat(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
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
        let _ = gc_rectangles_native(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.xr.XRBackendNative.XRAddGlyphsNative(I[JI[BI)V"
    )]
    async fn test_xr_add_glyphs_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = xr_add_glyphs_native(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.xr.XRBackendNative.XRCreateLinearGradientPaintNative([F[SIIIIII)I"
    )]
    async fn test_xr_create_linear_gradient_paint_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = xr_create_linear_gradient_paint_native(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.xr.XRBackendNative.XRCreateRadialGradientPaintNative([F[SIIIIII)I"
    )]
    async fn test_xr_create_radial_gradient_paint_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = xr_create_radial_gradient_paint_native(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.xr.XRBackendNative.XRFreeGlyphsNative(I[II)V"
    )]
    async fn test_xr_free_glyphs_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = xr_free_glyphs_native(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.xr.XRBackendNative.XRSetClipNative(JIIIILsun/java2d/pipe/Region;Z)V"
    )]
    async fn test_xr_set_clip_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = xr_set_clip_native(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.xr.XRBackendNative.XRSetTransformNative(IIIIIII)V"
    )]
    async fn test_xr_set_transform_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = xr_set_transform_native(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.xr.XRBackendNative.XRenderCompositeTextNative(IIIIIJ[I[III)V"
    )]
    async fn test_x_render_composite_text_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = x_render_composite_text_native(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.xr.XRBackendNative.XRenderCreateGlyphSetNative(J)I"
    )]
    async fn test_x_render_create_glyph_set_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = x_render_create_glyph_set_native(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.xr.XRBackendNative.XRenderRectanglesNative(IBSSSS[II)V"
    )]
    async fn test_x_render_rectangles_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = x_render_rectangles_native(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.xr.XRBackendNative.copyArea(IIJIIIIII)V"
    )]
    async fn test_copy_area() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = copy_area(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.java2d.xr.XRBackendNative.createGC(I)J")]
    async fn test_create_gc() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create_gc(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.xr.XRBackendNative.createPictureNative(IJ)I"
    )]
    async fn test_create_picture_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create_picture_native(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.xr.XRBackendNative.createPixmap(IIII)I"
    )]
    async fn test_create_pixmap() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create_pixmap(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.java2d.xr.XRBackendNative.freeGC(J)V")]
    async fn test_free_gc() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = free_gc(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.java2d.xr.XRBackendNative.freePicture(I)V")]
    async fn test_free_picture() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = free_picture(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.java2d.xr.XRBackendNative.freePixmap(I)V")]
    async fn test_free_pixmap() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = free_pixmap(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.xr.XRBackendNative.padBlitNative(BIIIIIIIIIIIIIIIIIII)V"
    )]
    async fn test_pad_blit_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = pad_blit_native(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.xr.XRBackendNative.putMaskNative(IJ[BIIIIIIIIFJ)V"
    )]
    async fn test_put_mask_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = put_mask_native(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.xr.XRBackendNative.renderComposite(BIIIIIIIIIII)V"
    )]
    async fn test_render_composite() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = render_composite(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.xr.XRBackendNative.renderCompositeTrapezoidsNative(BIJIII[I)V"
    )]
    async fn test_render_composite_trapezoids_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = render_composite_trapezoids_native(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.xr.XRBackendNative.renderRectangle(IBSSSSIIII)V"
    )]
    async fn test_render_rectangle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = render_rectangle(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.java2d.xr.XRBackendNative.setFilter(II)V")]
    async fn test_set_filter() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_filter(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.xr.XRBackendNative.setGCExposures(JZ)V"
    )]
    async fn test_set_gc_exposures() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_gc_exposures(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.xr.XRBackendNative.setGCForeground(JI)V"
    )]
    async fn test_set_gc_foreground() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_gc_foreground(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.java2d.xr.XRBackendNative.setGCMode(JZ)V")]
    async fn test_set_gc_mode() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_gc_mode(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.xr.XRBackendNative.setPictureRepeat(II)V"
    )]
    async fn test_set_picture_repeat() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_picture_repeat(thread, Parameters::default()).await;
    }
}
