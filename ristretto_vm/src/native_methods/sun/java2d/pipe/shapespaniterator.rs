use crate::Result;
use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/java2d/pipe/ShapeSpanIterator";

/// Register all native methods for `sun.java2d.pipe.ShapeSpanIterator`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "addSegment", "(I[F)V", add_segment);
    registry.register(CLASS_NAME, "appendPoly", "([I[IIII)V", append_poly);
    registry.register(CLASS_NAME, "closePath", "()V", close_path);
    registry.register(CLASS_NAME, "curveTo", "(FFFFFF)V", curve_to);
    registry.register(CLASS_NAME, "dispose", "()V", dispose);
    registry.register(CLASS_NAME, "getNativeConsumer", "()J", get_native_consumer);
    registry.register(CLASS_NAME, "getNativeIterator", "()J", get_native_iterator);
    registry.register(CLASS_NAME, "getPathBox", "([I)V", get_path_box);
    registry.register(CLASS_NAME, "initIDs", "()V", init_ids);
    registry.register(
        CLASS_NAME,
        "intersectClipBox",
        "(IIII)V",
        intersect_clip_box,
    );
    registry.register(CLASS_NAME, "lineTo", "(FF)V", line_to);
    registry.register(CLASS_NAME, "moveTo", "(FF)V", move_to);
    registry.register(CLASS_NAME, "nextSpan", "([I)Z", next_span);
    registry.register(CLASS_NAME, "pathDone", "()V", path_done);
    registry.register(CLASS_NAME, "quadTo", "(FFFF)V", quad_to);
    registry.register(CLASS_NAME, "setNormalize", "(Z)V", set_normalize);
    registry.register(
        CLASS_NAME,
        "setOutputAreaXYXY",
        "(IIII)V",
        set_output_area_xyxy,
    );
    registry.register(CLASS_NAME, "setRule", "(I)V", set_rule);
    registry.register(CLASS_NAME, "skipDownTo", "(I)V", skip_down_to);
}

#[async_recursion(?Send)]
async fn add_segment(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.pipe.ShapeSpanIterator.addSegment(I[F)V");
}

#[async_recursion(?Send)]
async fn append_poly(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.pipe.ShapeSpanIterator.appendPoly([I[IIII)V");
}

#[async_recursion(?Send)]
async fn close_path(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.pipe.ShapeSpanIterator.closePath()V");
}

#[async_recursion(?Send)]
async fn curve_to(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.pipe.ShapeSpanIterator.curveTo(FFFFFF)V");
}

#[async_recursion(?Send)]
async fn dispose(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.pipe.ShapeSpanIterator.dispose()V");
}

#[async_recursion(?Send)]
async fn get_native_consumer(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.pipe.ShapeSpanIterator.getNativeConsumer()J");
}

#[async_recursion(?Send)]
async fn get_native_iterator(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.pipe.ShapeSpanIterator.getNativeIterator()J");
}

#[async_recursion(?Send)]
async fn get_path_box(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.pipe.ShapeSpanIterator.getPathBox([I)V");
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn intersect_clip_box(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.pipe.ShapeSpanIterator.intersectClipBox(IIII)V");
}

#[async_recursion(?Send)]
async fn line_to(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.pipe.ShapeSpanIterator.lineTo(FF)V");
}

#[async_recursion(?Send)]
async fn move_to(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.pipe.ShapeSpanIterator.moveTo(FF)V");
}

#[async_recursion(?Send)]
async fn next_span(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.pipe.ShapeSpanIterator.nextSpan([I)Z");
}

#[async_recursion(?Send)]
async fn path_done(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.pipe.ShapeSpanIterator.pathDone()V");
}

#[async_recursion(?Send)]
async fn quad_to(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.pipe.ShapeSpanIterator.quadTo(FFFF)V");
}

#[async_recursion(?Send)]
async fn set_normalize(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.pipe.ShapeSpanIterator.setNormalize(Z)V");
}

#[async_recursion(?Send)]
async fn set_output_area_xyxy(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.pipe.ShapeSpanIterator.setOutputAreaXYXY(IIII)V");
}

#[async_recursion(?Send)]
async fn set_rule(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.pipe.ShapeSpanIterator.setRule(I)V");
}

#[async_recursion(?Send)]
async fn skip_down_to(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
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
