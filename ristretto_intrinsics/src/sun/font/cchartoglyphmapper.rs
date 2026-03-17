use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/font/CCharToGlyphMapper.countGlyphs(J)I", Any)]
#[async_method]
pub async fn count_glyphs<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun.font.CCharToGlyphMapper.countGlyphs(J)I".to_string())
            .into(),
    )
}

#[intrinsic_method("sun/font/CCharToGlyphMapper.nativeCharsToGlyphs(JI[C[I)V", Any)]
#[async_method]
pub async fn native_chars_to_glyphs<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.font.CCharToGlyphMapper.nativeCharsToGlyphs(JI[C[I)V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_count_glyphs() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = count_glyphs(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_chars_to_glyphs() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_chars_to_glyphs(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
