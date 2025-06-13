use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("sun/font/CCharToGlyphMapper.countGlyphs(J)I", Any)]
#[async_recursion(?Send)]
pub(crate) async fn count_glyphs(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.CCharToGlyphMapper.countGlyphs(J)I")
}

#[intrinsic_method("sun/font/CCharToGlyphMapper.nativeCharsToGlyphs(JI[C[I)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn native_chars_to_glyphs(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.CCharToGlyphMapper.nativeCharsToGlyphs(JI[C[I)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.font.CCharToGlyphMapper.countGlyphs(J)I")]
    async fn test_count_glyphs() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = count_glyphs(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.font.CCharToGlyphMapper.nativeCharsToGlyphs(JI[C[I)V"
    )]
    async fn test_native_chars_to_glyphs() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_chars_to_glyphs(thread, Parameters::default()).await;
    }
}
