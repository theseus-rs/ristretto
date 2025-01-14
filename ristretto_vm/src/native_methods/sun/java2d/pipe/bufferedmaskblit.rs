use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.java2d.pipe.BufferedMaskBlit`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/java2d/pipe/BufferedMaskBlit";
    registry.register(
        class_name,
        "enqueueTile",
        "(JILsun/java2d/SurfaceData;JI[BIIIIIIIII)I",
        enqueue_tile,
    );
}

#[async_recursion(?Send)]
async fn enqueue_tile(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.pipe.BufferedMaskBlit.enqueueTile(JILsun/java2d/SurfaceData;JI[BIIIIIIIII)I");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/java2d/pipe/BufferedMaskBlit";
        assert!(registry
            .method(
                class_name,
                "enqueueTile",
                "(JILsun/java2d/SurfaceData;JI[BIIIIIIIII)I"
            )
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.java2d.pipe.BufferedMaskBlit.enqueueTile(JILsun/java2d/SurfaceData;JI[BIIIIIIIII)I"
    )]
    async fn test_enqueue_tile() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = enqueue_tile(thread, Arguments::default()).await;
    }
}
