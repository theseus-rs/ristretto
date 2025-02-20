use crate::Result;
use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/java2d/pipe/BufferedRenderPipe";

/// Register all native methods for `sun.java2d.pipe.BufferedRenderPipe`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "fillSpans",
        "(Lsun/java2d/pipe/RenderQueue;JIILsun/java2d/pipe/SpanIterator;JII)I",
        fill_spans,
    );
}

#[async_recursion(?Send)]
async fn fill_spans(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "sun.java2d.pipe.BufferedRenderPipe.fillSpans(Lsun/java2d/pipe/RenderQueue;JIILsun/java2d/pipe/SpanIterator;JII)I"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.pipe.BufferedRenderPipe.fillSpans(Lsun/java2d/pipe/RenderQueue;JIILsun/java2d/pipe/SpanIterator;JII)I"
    )]
    async fn test_fill_spans() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = fill_spans(thread, Parameters::default()).await;
    }
}
