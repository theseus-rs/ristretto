use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/lwawt/macosx/CTextPipe.doDrawGlyphs(Lsun/java2d/SurfaceData;JLjava/awt/font/GlyphVector;FF)V",
    Any
)]
#[async_method]
pub(crate) async fn do_draw_glyphs(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.lwawt.macosx.CTextPipe.doDrawGlyphs(Lsun/java2d/SurfaceData;JLjava/awt/font/GlyphVector;FF)V"
    )
}

#[intrinsic_method(
    "sun/lwawt/macosx/CTextPipe.doDrawString(Lsun/java2d/SurfaceData;JLjava/lang/String;DD)V",
    Any
)]
#[async_method]
pub(crate) async fn do_draw_string(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CTextPipe.doDrawString(Lsun/java2d/SurfaceData;JLjava/lang/String;DD)V")
}

#[intrinsic_method(
    "sun/lwawt/macosx/CTextPipe.doOneUnicode(Lsun/java2d/SurfaceData;JCFF)V",
    Any
)]
#[async_method]
pub(crate) async fn do_one_unicode(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CTextPipe.doOneUnicode(Lsun/java2d/SurfaceData;JCFF)V")
}

#[intrinsic_method(
    "sun/lwawt/macosx/CTextPipe.doUnicodes(Lsun/java2d/SurfaceData;J[CIIFF)V",
    Any
)]
#[async_method]
pub(crate) async fn do_unicodes(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
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
