use crate::Result;
use crate::intrinsic_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/java2d/loops/MaskBlit";

/// Register all intrinsic methods for `sun.java2d.loops.MaskBlit`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "MaskBlit", "(Lsun/java2d/SurfaceData;Lsun/java2d/SurfaceData;Ljava/awt/Composite;Lsun/java2d/pipe/Region;IIIIII[BII)V", mask_blit);
}

#[async_recursion(?Send)]
async fn mask_blit(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "sun.java2d.loops.MaskBlit.MaskBlit(Lsun/java2d/SurfaceData;Lsun/java2d/SurfaceData;Ljava/awt/Composite;Lsun/java2d/pipe/Region;IIIIII[BII)V"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.loops.MaskBlit.MaskBlit(Lsun/java2d/SurfaceData;Lsun/java2d/SurfaceData;Ljava/awt/Composite;Lsun/java2d/pipe/Region;IIIIII[BII)V"
    )]
    async fn test_mask_blit() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = mask_blit(thread, Parameters::default()).await;
    }
}
