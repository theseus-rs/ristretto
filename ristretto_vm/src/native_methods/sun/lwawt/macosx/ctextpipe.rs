use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.lwawt.macosx.CTextPipe`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/lwawt/macosx/CTextPipe";
    registry.register(
        class_name,
        "doDrawGlyphs",
        "(Lsun/java2d/SurfaceData;JLjava/awt/font/GlyphVector;FF)V",
        do_draw_glyphs,
    );
    registry.register(
        class_name,
        "doDrawString",
        "(Lsun/java2d/SurfaceData;JLjava/lang/String;DD)V",
        do_draw_string,
    );
    registry.register(
        class_name,
        "doOneUnicode",
        "(Lsun/java2d/SurfaceData;JCFF)V",
        do_one_unicode,
    );
    registry.register(
        class_name,
        "doUnicodes",
        "(Lsun/java2d/SurfaceData;J[CIIFF)V",
        do_unicodes,
    );
}

#[async_recursion(?Send)]
async fn do_draw_glyphs(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CTextPipe.doDrawGlyphs(Lsun/java2d/SurfaceData;JLjava/awt/font/GlyphVector;FF)V")
}

#[async_recursion(?Send)]
async fn do_draw_string(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CTextPipe.doDrawString(Lsun/java2d/SurfaceData;JLjava/lang/String;DD)V")
}

#[async_recursion(?Send)]
async fn do_one_unicode(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CTextPipe.doOneUnicode(Lsun/java2d/SurfaceData;JCFF)V")
}

#[async_recursion(?Send)]
async fn do_unicodes(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CTextPipe.doUnicodes(Lsun/java2d/SurfaceData;J[CIIFF)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/lwawt/macosx/CTextPipe";
        assert!(registry
            .method(
                class_name,
                "doDrawGlyphs",
                "(Lsun/java2d/SurfaceData;JLjava/awt/font/GlyphVector;FF)V"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "doDrawString",
                "(Lsun/java2d/SurfaceData;JLjava/lang/String;DD)V"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "doOneUnicode",
                "(Lsun/java2d/SurfaceData;JCFF)V"
            )
            .is_some());
        assert!(registry
            .method(
                class_name,
                "doUnicodes",
                "(Lsun/java2d/SurfaceData;J[CIIFF)V"
            )
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.lwawt.macosx.CTextPipe.doDrawGlyphs(Lsun/java2d/SurfaceData;JLjava/awt/font/GlyphVector;FF)V"
    )]
    async fn test_do_draw_glyphs() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = do_draw_glyphs(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.lwawt.macosx.CTextPipe.doDrawString(Lsun/java2d/SurfaceData;JLjava/lang/String;DD)V"
    )]
    async fn test_do_draw_string() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = do_draw_string(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.lwawt.macosx.CTextPipe.doOneUnicode(Lsun/java2d/SurfaceData;JCFF)V"
    )]
    async fn test_do_one_unicode() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = do_one_unicode(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.lwawt.macosx.CTextPipe.doUnicodes(Lsun/java2d/SurfaceData;J[CIIFF)V"
    )]
    async fn test_do_unicodes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = do_unicodes(thread, Arguments::default()).await;
    }
}
