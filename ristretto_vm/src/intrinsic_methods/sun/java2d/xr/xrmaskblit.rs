use crate::Result;
use crate::intrinsic_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/java2d/xr/XRMaskBlit";

/// Register all intrinsic methods for `sun.java2d.xr.XRMaskBlit`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "maskBlit", "(JJIIIIIIIII[B)V", mask_blit);
}

#[async_recursion(?Send)]
async fn mask_blit(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.xr.XRMaskBlit.maskBlit(JJIIIIIIIII[B)V");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.xr.XRMaskBlit.maskBlit(JJIIIIIIIII[B)V"
    )]
    async fn test_mask_blit() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = mask_blit(thread, Parameters::default()).await;
    }
}
