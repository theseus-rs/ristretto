use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/java2d/pipe/BufferedRenderPipe.fillSpans(Lsun/java2d/pipe/RenderQueue;JIILsun/java2d/pipe/SpanIterator;JII)I",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn fill_spans(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
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
