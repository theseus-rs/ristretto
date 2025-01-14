use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_11: Version = Version::Java11 { minor: 0 };

/// Register all native methods for `sun.java2d.loops.DrawGlyphListLCD`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/java2d/loops/DrawGlyphListLCD";
    let java_version = registry.java_version();

    if java_version <= &JAVA_11 {
        registry.register(
            class_name,
            "DrawGlyphListLCD",
            "(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;Lsun/font/GlyphList;)V",
            draw_glyph_list_lcd,
        );
    } else {
        registry.register(
            class_name,
            "DrawGlyphListLCD",
            "(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;Lsun/font/GlyphList;II)V",
            draw_glyph_list_lcd,
        );
    }
}

#[async_recursion(?Send)]
async fn draw_glyph_list_lcd(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.loops.DrawGlyphListLCD.DrawGlyphListLCD(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;Lsun/font/GlyphList;)V");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/java2d/loops/DrawGlyphListLCD";
        assert!(registry
            .method(
                class_name,
                "DrawGlyphListLCD",
                "(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;Lsun/font/GlyphList;)V"
            )
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.java2d.loops.DrawGlyphListLCD.DrawGlyphListLCD(Lsun/java2d/SunGraphics2D;Lsun/java2d/SurfaceData;Lsun/font/GlyphList;)V"
    )]
    async fn test_draw_glyph_list_lcd() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = draw_glyph_list_lcd(thread, Arguments::default()).await;
    }
}
