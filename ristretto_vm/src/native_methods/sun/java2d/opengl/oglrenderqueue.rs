use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.java2d.opengl.OGLRenderQueue`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/java2d/opengl/OGLRenderQueue";
    registry.register(class_name, "flushBuffer", "(JI)V", flush_buffer);
}

#[async_recursion(?Send)]
async fn flush_buffer(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.OGLRenderQueue.flushBuffer(JI)V");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/java2d/opengl/OGLRenderQueue";
        assert!(registry
            .method(class_name, "flushBuffer", "(JI)V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.opengl.OGLRenderQueue.flushBuffer(JI)V")]
    async fn test_flush_buffer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = flush_buffer(thread, Arguments::default()).await;
    }
}
