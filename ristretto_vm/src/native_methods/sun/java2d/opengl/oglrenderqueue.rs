use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/java2d/opengl/OGLRenderQueue";

/// Register all native methods for `sun.java2d.opengl.OGLRenderQueue`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "flushBuffer", "(JI)V", flush_buffer);
}

#[async_recursion(?Send)]
async fn flush_buffer(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.OGLRenderQueue.flushBuffer(JI)V");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.opengl.OGLRenderQueue.flushBuffer(JI)V"
    )]
    async fn test_flush_buffer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = flush_buffer(thread, Arguments::default()).await;
    }
}
