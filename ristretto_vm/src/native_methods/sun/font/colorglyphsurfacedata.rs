use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.font.ColorGlyphSurfaceData`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/font/ColorGlyphSurfaceData";
    registry.register(class_name, "initOps", "()V", init_ops);
    registry.register(class_name, "setCurrentGlyph", "(J)V", set_current_glyph);
}

#[async_recursion(?Send)]
async fn init_ops(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.font.ColorGlyphSurfaceData.initOps()V")
}

#[async_recursion(?Send)]
async fn set_current_glyph(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.font.ColorGlyphSurfaceData.setCurrentGlyph(J)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/font/ColorGlyphSurfaceData";
        assert!(registry.method(class_name, "initOps", "()V").is_some());
        assert!(registry
            .method(class_name, "setCurrentGlyph", "(J)V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.font.ColorGlyphSurfaceData.initOps()V")]
    async fn test_init_ops() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_ops(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.font.ColorGlyphSurfaceData.setCurrentGlyph(J)V")]
    async fn test_set_current_glyph() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_current_glyph(thread, Arguments::default()).await;
    }
}
