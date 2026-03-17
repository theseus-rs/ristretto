use ristretto_classfile::JAVA_8;
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
    _parameters: Parameters,
) -> Result<Option<Value>> {
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
    _parameters: Parameters,
) -> Result<Option<Value>> {
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
    _parameters: Parameters,
) -> Result<Option<Value>> {
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
    _parameters: Parameters,
) -> Result<Option<Value>> {
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
    _parameters: Parameters,
) -> Result<Option<Value>> {
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
    _parameters: Parameters,
) -> Result<Option<Value>> {
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
    _parameters: Parameters,
) -> Result<Option<Value>> {
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
    _parameters: Parameters,
) -> Result<Option<Value>> {
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
    _parameters: Parameters,
) -> Result<Option<Value>> {
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
    _parameters: Parameters,
) -> Result<Option<Value>> {
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
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.xr.XRBackendNative.copyArea(IIJIIIIII)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/java2d/xr/XRBackendNative.createGC(I)J", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn create_gc<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
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
    _parameters: Parameters,
) -> Result<Option<Value>> {
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
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.xr.XRBackendNative.createPixmap(IIII)I".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/java2d/xr/XRBackendNative.freeGC(J)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn free_gc<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
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
    _parameters: Parameters,
) -> Result<Option<Value>> {
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
    _parameters: Parameters,
) -> Result<Option<Value>> {
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
    _parameters: Parameters,
) -> Result<Option<Value>> {
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
    _parameters: Parameters,
) -> Result<Option<Value>> {
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
    _parameters: Parameters,
) -> Result<Option<Value>> {
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
    _parameters: Parameters,
) -> Result<Option<Value>> {
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
    _parameters: Parameters,
) -> Result<Option<Value>> {
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
    _parameters: Parameters,
) -> Result<Option<Value>> {
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
    _parameters: Parameters,
) -> Result<Option<Value>> {
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
    _parameters: Parameters,
) -> Result<Option<Value>> {
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
    _parameters: Parameters,
) -> Result<Option<Value>> {
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
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.xr.XRBackendNative.setPictureRepeat(II)V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_gc_rectangles_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = gc_rectangles_native(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_xr_add_glyphs_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xr_add_glyphs_native(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_xr_create_linear_gradient_paint_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xr_create_linear_gradient_paint_native(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_xr_create_radial_gradient_paint_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xr_create_radial_gradient_paint_native(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_xr_free_glyphs_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xr_free_glyphs_native(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_xr_set_clip_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xr_set_clip_native(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_xr_set_transform_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = xr_set_transform_native(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_x_render_composite_text_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = x_render_composite_text_native(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_x_render_create_glyph_set_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = x_render_create_glyph_set_native(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_x_render_rectangles_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = x_render_rectangles_native(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_copy_area() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = copy_area(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_gc() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create_gc(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_picture_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create_picture_native(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_pixmap() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create_pixmap(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_free_gc() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = free_gc(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_free_picture() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = free_picture(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_free_pixmap() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = free_pixmap(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_pad_blit_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = pad_blit_native(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_put_mask_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = put_mask_native(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_render_composite() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = render_composite(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_render_composite_trapezoids_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = render_composite_trapezoids_native(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_render_rectangle() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = render_rectangle(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_filter() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_filter(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_gc_exposures() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_gc_exposures(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_gc_foreground() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_gc_foreground(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_gc_mode() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_gc_mode(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_picture_repeat() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_picture_repeat(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
