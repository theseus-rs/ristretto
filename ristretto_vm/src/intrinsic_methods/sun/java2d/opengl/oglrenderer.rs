use crate::Result;
use crate::intrinsic_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/java2d/opengl/OGLRenderer";

/// Register all intrinsic methods for `sun.java2d.opengl.OGLRenderer`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "drawPoly", "([I[IIZII)V", draw_poly);
}

#[async_recursion(?Send)]
async fn draw_poly(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.OGLRenderer.drawPoly([I[IIZII)V");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.opengl.OGLRenderer.drawPoly([I[IIZII)V"
    )]
    async fn test_draw_poly() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = draw_poly(thread, Parameters::default()).await;
    }
}
