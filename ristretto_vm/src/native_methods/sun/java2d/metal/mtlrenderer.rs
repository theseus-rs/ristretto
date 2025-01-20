use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/java2d/metal/MTLRenderer";

/// Register all native methods for `sun.java2d.metal.MTLRenderer`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "drawPoly", "([I[IIZII)V", draw_poly);
}

#[async_recursion(?Send)]
async fn draw_poly(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.metal.MTLRenderer.drawPoly([I[IIZII)V");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.metal.MTLRenderer.drawPoly([I[IIZII)V"
    )]
    async fn test_draw_poly() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = draw_poly(thread, Parameters::default()).await;
    }
}
