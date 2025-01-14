use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.java2d.pipe.ShapeSpanIterator`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/java2d/pipe/ShapeSpanIterator";
    registry.register(class_name, "addSegment", "(I[F)V", add_segment);
    registry.register(class_name, "appendPoly", "([I[IIII)V", append_poly);
    registry.register(class_name, "closePath", "()V", close_path);
    registry.register(class_name, "curveTo", "(FFFFFF)V", curve_to);
    registry.register(class_name, "dispose", "()V", dispose);
    registry.register(class_name, "getNativeConsumer", "()J", get_native_consumer);
    registry.register(class_name, "getNativeIterator", "()J", get_native_iterator);
    registry.register(class_name, "getPathBox", "([I)V", get_path_box);
    registry.register(class_name, "initIDs", "()V", init_ids);
    registry.register(
        class_name,
        "intersectClipBox",
        "(IIII)V",
        intersect_clip_box,
    );
    registry.register(class_name, "lineTo", "(FF)V", line_to);
    registry.register(class_name, "moveTo", "(FF)V", move_to);
    registry.register(class_name, "nextSpan", "([I)Z", next_span);
    registry.register(class_name, "pathDone", "()V", path_done);
    registry.register(class_name, "quadTo", "(FFFF)V", quad_to);
    registry.register(class_name, "setNormalize", "(Z)V", set_normalize);
    registry.register(
        class_name,
        "setOutputAreaXYXY",
        "(IIII)V",
        set_output_area_xyxy,
    );
    registry.register(class_name, "setRule", "(I)V", set_rule);
    registry.register(class_name, "skipDownTo", "(I)V", skip_down_to);
}

#[async_recursion(?Send)]
async fn add_segment(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.pipe.ShapeSpanIterator.addSegment(I[F)V");
}

#[async_recursion(?Send)]
async fn append_poly(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.pipe.ShapeSpanIterator.appendPoly([I[IIII)V");
}

#[async_recursion(?Send)]
async fn close_path(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.pipe.ShapeSpanIterator.closePath()V");
}

#[async_recursion(?Send)]
async fn curve_to(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.pipe.ShapeSpanIterator.curveTo(FFFFFF)V");
}

#[async_recursion(?Send)]
async fn dispose(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.pipe.ShapeSpanIterator.dispose()V");
}

#[async_recursion(?Send)]
async fn get_native_consumer(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.pipe.ShapeSpanIterator.getNativeConsumer()J");
}

#[async_recursion(?Send)]
async fn get_native_iterator(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.pipe.ShapeSpanIterator.getNativeIterator()J");
}

#[async_recursion(?Send)]
async fn get_path_box(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.pipe.ShapeSpanIterator.getPathBox([I)V");
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn intersect_clip_box(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.pipe.ShapeSpanIterator.intersectClipBox(IIII)V");
}

#[async_recursion(?Send)]
async fn line_to(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.pipe.ShapeSpanIterator.lineTo(FF)V");
}

#[async_recursion(?Send)]
async fn move_to(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.pipe.ShapeSpanIterator.moveTo(FF)V");
}

#[async_recursion(?Send)]
async fn next_span(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.pipe.ShapeSpanIterator.nextSpan([I)Z");
}

#[async_recursion(?Send)]
async fn path_done(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.pipe.ShapeSpanIterator.pathDone()V");
}

#[async_recursion(?Send)]
async fn quad_to(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.pipe.ShapeSpanIterator.quadTo(FFFF)V");
}

#[async_recursion(?Send)]
async fn set_normalize(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.pipe.ShapeSpanIterator.setNormalize(Z)V");
}

#[async_recursion(?Send)]
async fn set_output_area_xyxy(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.java2d.pipe.ShapeSpanIterator.setOutputAreaXYXY(IIII)V");
}

#[async_recursion(?Send)]
async fn set_rule(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.pipe.ShapeSpanIterator.setRule(I)V");
}

#[async_recursion(?Send)]
async fn skip_down_to(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.pipe.ShapeSpanIterator.skipDownTo(I)V");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/java2d/pipe/ShapeSpanIterator";
        assert!(registry
            .method(class_name, "addSegment", "(I[F)V")
            .is_some());
        assert!(registry
            .method(class_name, "appendPoly", "([I[IIII)V")
            .is_some());
        assert!(registry.method(class_name, "closePath", "()V").is_some());
        assert!(registry
            .method(class_name, "curveTo", "(FFFFFF)V")
            .is_some());
        assert!(registry.method(class_name, "dispose", "()V").is_some());
        assert!(registry
            .method(class_name, "getNativeConsumer", "()J")
            .is_some());
        assert!(registry
            .method(class_name, "getNativeIterator", "()J")
            .is_some());
        assert!(registry.method(class_name, "getPathBox", "([I)V").is_some());
        assert!(registry.method(class_name, "initIDs", "()V").is_some());
        assert!(registry
            .method(class_name, "intersectClipBox", "(IIII)V")
            .is_some());
        assert!(registry.method(class_name, "lineTo", "(FF)V").is_some());
        assert!(registry.method(class_name, "moveTo", "(FF)V").is_some());
        assert!(registry.method(class_name, "nextSpan", "([I)Z").is_some());
        assert!(registry.method(class_name, "pathDone", "()V").is_some());
        assert!(registry.method(class_name, "quadTo", "(FFFF)V").is_some());
        assert!(registry
            .method(class_name, "setNormalize", "(Z)V")
            .is_some());
        assert!(registry
            .method(class_name, "setOutputAreaXYXY", "(IIII)V")
            .is_some());
        assert!(registry.method(class_name, "setRule", "(I)V").is_some());
        assert!(registry.method(class_name, "skipDownTo", "(I)V").is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.pipe.ShapeSpanIterator.addSegment(I[F)V")]
    async fn test_add_segment() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = add_segment(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.pipe.ShapeSpanIterator.appendPoly([I[IIII)V")]
    async fn test_append_poly() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = append_poly(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.pipe.ShapeSpanIterator.closePath()V")]
    async fn test_close_path() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = close_path(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.pipe.ShapeSpanIterator.curveTo(FFFFFF)V")]
    async fn test_curve_to() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = curve_to(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.pipe.ShapeSpanIterator.dispose()V")]
    async fn test_dispose() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = dispose(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.pipe.ShapeSpanIterator.getNativeConsumer()J")]
    async fn test_get_native_consumer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_native_consumer(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.pipe.ShapeSpanIterator.getNativeIterator()J")]
    async fn test_get_native_iterator() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_native_iterator(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.pipe.ShapeSpanIterator.getPathBox([I)V")]
    async fn test_get_path_box() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_path_box(thread, Arguments::default()).await;
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Arguments::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.pipe.ShapeSpanIterator.intersectClipBox(IIII)V")]
    async fn test_intersect_clip_box() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = intersect_clip_box(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.pipe.ShapeSpanIterator.lineTo(FF)V")]
    async fn test_line_to() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = line_to(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.pipe.ShapeSpanIterator.moveTo(FF)V")]
    async fn test_move_to() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = move_to(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.pipe.ShapeSpanIterator.nextSpan([I)Z")]
    async fn test_next_span() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = next_span(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.pipe.ShapeSpanIterator.pathDone()V")]
    async fn test_path_done() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = path_done(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.pipe.ShapeSpanIterator.quadTo(FFFF)V")]
    async fn test_quad_to() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = quad_to(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.pipe.ShapeSpanIterator.setNormalize(Z)V")]
    async fn test_set_normalize() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_normalize(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.pipe.ShapeSpanIterator.setOutputAreaXYXY(IIII)V")]
    async fn test_set_output_area_xyxy() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_output_area_xyxy(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.pipe.ShapeSpanIterator.setRule(I)V")]
    async fn test_set_rule() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_rule(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.pipe.ShapeSpanIterator.skipDownTo(I)V")]
    async fn test_skip_down_to() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = skip_down_to(thread, Arguments::default()).await;
    }
}
