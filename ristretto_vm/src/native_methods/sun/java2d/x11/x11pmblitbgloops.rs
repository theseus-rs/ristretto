use crate::Result;
use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/java2d/x11/X11PMBlitBgLoops";

/// Register all native methods for `sun.java2d.x11.X11PMBlitBgLoops`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "nativeBlitBg", "(JJJIIIIIII)V", native_blit_bg);
}

#[async_recursion(?Send)]
async fn native_blit_bg(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.X11PMBlitBgLoops.nativeBlitBg(JJJIIIIIII)V");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.x11.X11PMBlitBgLoops.nativeBlitBg(JJJIIIIIII)V"
    )]
    async fn test_native_blit_bg() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_blit_bg(thread, Parameters::default()).await;
    }
}
