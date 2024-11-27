use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.java2d.pipe.BufferedRenderPipe`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/java2d/pipe/BufferedRenderPipe";
    registry.register(
        class_name,
        "fillSpans",
        "(Lsun/java2d/pipe/RenderQueue;JIILsun/java2d/pipe/SpanIterator;JII)I",
        fill_spans,
    );
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn fill_spans(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}