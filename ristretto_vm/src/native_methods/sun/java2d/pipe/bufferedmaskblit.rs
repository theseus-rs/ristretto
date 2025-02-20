use crate::Result;
use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/java2d/pipe/BufferedMaskBlit";

/// Register all native methods for `sun.java2d.pipe.BufferedMaskBlit`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "enqueueTile",
        "(JILsun/java2d/SurfaceData;JI[BIIIIIIIII)I",
        enqueue_tile,
    );
}

#[async_recursion(?Send)]
async fn enqueue_tile(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.pipe.BufferedMaskBlit.enqueueTile(JILsun/java2d/SurfaceData;JI[BIIIIIIIII)I");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.pipe.BufferedMaskBlit.enqueueTile(JILsun/java2d/SurfaceData;JI[BIIIIIIIII)I"
    )]
    async fn test_enqueue_tile() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = enqueue_tile(thread, Parameters::default()).await;
    }
}
