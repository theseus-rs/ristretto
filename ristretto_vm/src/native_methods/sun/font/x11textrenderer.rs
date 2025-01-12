use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.font.X11TextRenderer`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/font/X11TextRenderer";
    registry.register(
        class_name,
        "doDrawGlyphList",
        "(JJLsun/java2d/pipe/Region;Lsun/font/GlyphList;)V",
        do_draw_glyph_list,
    );
}

#[async_recursion(?Send)]
async fn do_draw_glyph_list(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!(
        "sun.font.X11TextRenderer.doDrawGlyphList(JJLsun/java2d/pipe/Region;Lsun/font/GlyphList;)V"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/font/X11TextRenderer";
        assert!(registry
            .method(
                class_name,
                "doDrawGlyphList",
                "(JJLsun/java2d/pipe/Region;Lsun/font/GlyphList;)V"
            )
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.font.X11TextRenderer.doDrawGlyphList(JJLsun/java2d/pipe/Region;Lsun/font/GlyphList;)V"
    )]
    async fn test_do_draw_glyph_list() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = do_draw_glyph_list(thread, Arguments::default()).await;
    }
}
