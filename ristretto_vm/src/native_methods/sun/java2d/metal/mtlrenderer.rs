use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.java2d.metal.MTLRenderer`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/java2d/metal/MTLRenderer";
    registry.register(class_name, "drawPoly", "([I[IIZII)V", draw_poly);
}

#[async_recursion(?Send)]
async fn draw_poly(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.metal.MTLRenderer.drawPoly([I[IIZII)V");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/java2d/metal/MTLRenderer";
        assert!(registry
            .method(class_name, "drawPoly", "([I[IIZII)V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.metal.MTLRenderer.drawPoly([I[IIZII)V")]
    async fn test_draw_poly() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = draw_poly(thread, Arguments::default()).await;
    }
}
