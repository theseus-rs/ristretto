use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.java2d.loops.FillSpans`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/java2d/loops/FillSpans";
    registry.register(
        class_name,
        "FillSpans",
        "(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;IJLsun/java2d/pipe/SpanIterator;)V",
        fill_spans,
    );
}

#[async_recursion(?Send)]
async fn fill_spans(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.loops.FillSpans.FillSpans(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;IJLsun/java2d/pipe/SpanIterator;)V");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/java2d/loops/FillSpans";
        assert!(registry
            .method(
                class_name,
                "FillSpans",
                "(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;IJLsun/java2d/pipe/SpanIterator;)V"
            )
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.java2d.loops.FillSpans.FillSpans(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;IJLsun/java2d/pipe/SpanIterator;)V"
    )]
    async fn test_fill_spans() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = fill_spans(thread, Arguments::default()).await;
    }
}
