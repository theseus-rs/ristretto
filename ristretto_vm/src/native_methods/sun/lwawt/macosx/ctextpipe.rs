use crate::Result;
use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/lwawt/macosx/CTextPipe";

/// Register all native methods for `sun.lwawt.macosx.CTextPipe`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "doDrawGlyphs",
        "(Lsun/java2d/SurfaceData;JLjava/awt/font/GlyphVector;FF)V",
        do_draw_glyphs,
    );
    registry.register(
        CLASS_NAME,
        "doDrawString",
        "(Lsun/java2d/SurfaceData;JLjava/lang/String;DD)V",
        do_draw_string,
    );
    registry.register(
        CLASS_NAME,
        "doOneUnicode",
        "(Lsun/java2d/SurfaceData;JCFF)V",
        do_one_unicode,
    );
    registry.register(
        CLASS_NAME,
        "doUnicodes",
        "(Lsun/java2d/SurfaceData;J[CIIFF)V",
        do_unicodes,
    );
}

#[async_recursion(?Send)]
async fn do_draw_glyphs(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!(
        "sun.lwawt.macosx.CTextPipe.doDrawGlyphs(Lsun/java2d/SurfaceData;JLjava/awt/font/GlyphVector;FF)V"
    )
}

#[async_recursion(?Send)]
async fn do_draw_string(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CTextPipe.doDrawString(Lsun/java2d/SurfaceData;JLjava/lang/String;DD)V")
}

#[async_recursion(?Send)]
async fn do_one_unicode(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CTextPipe.doOneUnicode(Lsun/java2d/SurfaceData;JCFF)V")
}

#[async_recursion(?Send)]
async fn do_unicodes(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CTextPipe.doUnicodes(Lsun/java2d/SurfaceData;J[CIIFF)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CTextPipe.doDrawGlyphs(Lsun/java2d/SurfaceData;JLjava/awt/font/GlyphVector;FF)V"
    )]
    async fn test_do_draw_glyphs() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = do_draw_glyphs(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CTextPipe.doDrawString(Lsun/java2d/SurfaceData;JLjava/lang/String;DD)V"
    )]
    async fn test_do_draw_string() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = do_draw_string(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CTextPipe.doOneUnicode(Lsun/java2d/SurfaceData;JCFF)V"
    )]
    async fn test_do_one_unicode() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = do_one_unicode(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CTextPipe.doUnicodes(Lsun/java2d/SurfaceData;J[CIIFF)V"
    )]
    async fn test_do_unicodes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = do_unicodes(thread, Parameters::default()).await;
    }
}
