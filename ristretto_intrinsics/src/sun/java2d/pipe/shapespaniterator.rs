use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/java2d/pipe/ShapeSpanIterator.addSegment(I[F)V", Any)]
#[async_method]
pub async fn add_segment<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.pipe.ShapeSpanIterator.addSegment(I[F)V");
}

#[intrinsic_method("sun/java2d/pipe/ShapeSpanIterator.appendPoly([I[IIII)V", Any)]
#[async_method]
pub async fn append_poly<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.pipe.ShapeSpanIterator.appendPoly([I[IIII)V");
}

#[intrinsic_method("sun/java2d/pipe/ShapeSpanIterator.closePath()V", Any)]
#[async_method]
pub async fn close_path<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.pipe.ShapeSpanIterator.closePath()V");
}

#[intrinsic_method("sun/java2d/pipe/ShapeSpanIterator.curveTo(FFFFFF)V", Any)]
#[async_method]
pub async fn curve_to<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.pipe.ShapeSpanIterator.curveTo(FFFFFF)V");
}

#[intrinsic_method("sun/java2d/pipe/ShapeSpanIterator.dispose()V", Any)]
#[async_method]
pub async fn dispose<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.pipe.ShapeSpanIterator.dispose()V");
}

#[intrinsic_method("sun/java2d/pipe/ShapeSpanIterator.getNativeConsumer()J", Any)]
#[async_method]
pub async fn get_native_consumer<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.pipe.ShapeSpanIterator.getNativeConsumer()J");
}

#[intrinsic_method("sun/java2d/pipe/ShapeSpanIterator.getNativeIterator()J", Any)]
#[async_method]
pub async fn get_native_iterator<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.pipe.ShapeSpanIterator.getNativeIterator()J");
}

#[intrinsic_method("sun/java2d/pipe/ShapeSpanIterator.getPathBox([I)V", Any)]
#[async_method]
pub async fn get_path_box<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.pipe.ShapeSpanIterator.getPathBox([I)V");
}

#[intrinsic_method("sun/java2d/pipe/ShapeSpanIterator.initIDs()V", Any)]
#[async_method]
pub async fn init_ids<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("sun/java2d/pipe/ShapeSpanIterator.intersectClipBox(IIII)V", Any)]
#[async_method]
pub async fn intersect_clip_box<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.pipe.ShapeSpanIterator.intersectClipBox(IIII)V");
}

#[intrinsic_method("sun/java2d/pipe/ShapeSpanIterator.lineTo(FF)V", Any)]
#[async_method]
pub async fn line_to<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.pipe.ShapeSpanIterator.lineTo(FF)V");
}

#[intrinsic_method("sun/java2d/pipe/ShapeSpanIterator.moveTo(FF)V", Any)]
#[async_method]
pub async fn move_to<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.pipe.ShapeSpanIterator.moveTo(FF)V");
}

#[intrinsic_method("sun/java2d/pipe/ShapeSpanIterator.nextSpan([I)Z", Any)]
#[async_method]
pub async fn next_span<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.pipe.ShapeSpanIterator.nextSpan([I)Z");
}

#[intrinsic_method("sun/java2d/pipe/ShapeSpanIterator.pathDone()V", Any)]
#[async_method]
pub async fn path_done<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.pipe.ShapeSpanIterator.pathDone()V");
}

#[intrinsic_method("sun/java2d/pipe/ShapeSpanIterator.quadTo(FFFF)V", Any)]
#[async_method]
pub async fn quad_to<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.pipe.ShapeSpanIterator.quadTo(FFFF)V");
}

#[intrinsic_method("sun/java2d/pipe/ShapeSpanIterator.setNormalize(Z)V", Any)]
#[async_method]
pub async fn set_normalize<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.pipe.ShapeSpanIterator.setNormalize(Z)V");
}

#[intrinsic_method("sun/java2d/pipe/ShapeSpanIterator.setOutputAreaXYXY(IIII)V", Any)]
#[async_method]
pub async fn set_output_area_xyxy<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.pipe.ShapeSpanIterator.setOutputAreaXYXY(IIII)V");
}

#[intrinsic_method("sun/java2d/pipe/ShapeSpanIterator.setRule(I)V", Any)]
#[async_method]
pub async fn set_rule<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.pipe.ShapeSpanIterator.setRule(I)V");
}

#[intrinsic_method("sun/java2d/pipe/ShapeSpanIterator.skipDownTo(I)V", Any)]
#[async_method]
pub async fn skip_down_to<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.pipe.ShapeSpanIterator.skipDownTo(I)V");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.pipe.ShapeSpanIterator.addSegment(I[F)V"
    )]
    async fn test_add_segment() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = add_segment(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.pipe.ShapeSpanIterator.appendPoly([I[IIII)V"
    )]
    async fn test_append_poly() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = append_poly(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.pipe.ShapeSpanIterator.closePath()V"
    )]
    async fn test_close_path() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = close_path(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.pipe.ShapeSpanIterator.curveTo(FFFFFF)V"
    )]
    async fn test_curve_to() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = curve_to(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.java2d.pipe.ShapeSpanIterator.dispose()V")]
    async fn test_dispose() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = dispose(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.pipe.ShapeSpanIterator.getNativeConsumer()J"
    )]
    async fn test_get_native_consumer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_native_consumer(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.pipe.ShapeSpanIterator.getNativeIterator()J"
    )]
    async fn test_get_native_iterator() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_native_iterator(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.pipe.ShapeSpanIterator.getPathBox([I)V"
    )]
    async fn test_get_path_box() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_path_box(thread, Parameters::default()).await;
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
        expected = "not yet implemented: sun.java2d.pipe.ShapeSpanIterator.intersectClipBox(IIII)V"
    )]
    async fn test_intersect_clip_box() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = intersect_clip_box(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.java2d.pipe.ShapeSpanIterator.lineTo(FF)V")]
    async fn test_line_to() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = line_to(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.java2d.pipe.ShapeSpanIterator.moveTo(FF)V")]
    async fn test_move_to() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = move_to(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.pipe.ShapeSpanIterator.nextSpan([I)Z"
    )]
    async fn test_next_span() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = next_span(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.java2d.pipe.ShapeSpanIterator.pathDone()V")]
    async fn test_path_done() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = path_done(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.pipe.ShapeSpanIterator.quadTo(FFFF)V"
    )]
    async fn test_quad_to() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = quad_to(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.pipe.ShapeSpanIterator.setNormalize(Z)V"
    )]
    async fn test_set_normalize() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_normalize(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.pipe.ShapeSpanIterator.setOutputAreaXYXY(IIII)V"
    )]
    async fn test_set_output_area_xyxy() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_output_area_xyxy(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.java2d.pipe.ShapeSpanIterator.setRule(I)V")]
    async fn test_set_rule() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_rule(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.pipe.ShapeSpanIterator.skipDownTo(I)V"
    )]
    async fn test_skip_down_to() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = skip_down_to(thread, Parameters::default()).await;
    }
}
