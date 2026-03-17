use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/lwawt/macosx/CTextPipe.doDrawGlyphs(Lsun/java2d/SurfaceData;JLjava/awt/font/GlyphVector;FF)V",
    Any
)]
#[async_method]
pub async fn do_draw_glyphs<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.lwawt.macosx.CTextPipe.doDrawGlyphs(Lsun/java2d/SurfaceData;JLjava/awt/font/GlyphVector;FF)V".to_string()).into())
}

#[intrinsic_method(
    "sun/lwawt/macosx/CTextPipe.doDrawString(Lsun/java2d/SurfaceData;JLjava/lang/String;DD)V",
    Any
)]
#[async_method]
pub async fn do_draw_string<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CTextPipe.doDrawString(Lsun/java2d/SurfaceData;JLjava/lang/String;DD)V"
            .to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/lwawt/macosx/CTextPipe.doOneUnicode(Lsun/java2d/SurfaceData;JCFF)V",
    Any
)]
#[async_method]
pub async fn do_one_unicode<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CTextPipe.doOneUnicode(Lsun/java2d/SurfaceData;JCFF)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/lwawt/macosx/CTextPipe.doUnicodes(Lsun/java2d/SurfaceData;J[CIIFF)V",
    Any
)]
#[async_method]
pub async fn do_unicodes<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CTextPipe.doUnicodes(Lsun/java2d/SurfaceData;J[CIIFF)V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_do_draw_glyphs() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = do_draw_glyphs(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_do_draw_string() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = do_draw_string(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_do_one_unicode() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = do_one_unicode(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_do_unicodes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = do_unicodes(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
