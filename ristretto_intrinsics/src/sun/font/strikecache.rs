use ristretto_classfile::VersionSpecification::{Any, GreaterThanOrEqual, LessThanOrEqual};
use ristretto_classfile::{JAVA_21, JAVA_25};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/font/StrikeCache.freeIntMemory([IJ)V", Any)]
#[async_method]
pub async fn free_int_memory<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun.font.StrikeCache.freeIntMemory([IJ)V".to_string())
            .into(),
    )
}

#[intrinsic_method("sun/font/StrikeCache.freeIntPointer(I)V", Any)]
#[async_method]
pub async fn free_int_pointer<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun.font.StrikeCache.freeIntPointer(I)V".to_string())
            .into(),
    )
}

#[intrinsic_method("sun/font/StrikeCache.freeLongMemory([JJ)V", Any)]
#[async_method]
pub async fn free_long_memory<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun.font.StrikeCache.freeLongMemory([JJ)V".to_string())
            .into(),
    )
}

#[intrinsic_method("sun/font/StrikeCache.freeLongPointer(J)V", Any)]
#[async_method]
pub async fn free_long_pointer<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun.font.StrikeCache.freeLongPointer(J)V".to_string())
            .into(),
    )
}

#[intrinsic_method(
    "sun/font/StrikeCache.getGlyphCacheDescription([J)V",
    LessThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn get_glyph_cache_description<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.font.StrikeCache.getGlyphCacheDescription([J)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/font/StrikeCache.getInvisibleGlyphPtr()J",
    GreaterThanOrEqual(JAVA_25)
)]
#[async_method]
pub async fn get_invisible_glyph_ptr<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun.font.StrikeCache.getInvisibleGlyphPtr()J".to_string())
            .into(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_free_int_memory() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = free_int_memory(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_free_int_pointer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = free_int_pointer(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_free_long_memory() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = free_long_memory(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_free_long_pointer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = free_long_pointer(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_glyph_cache_description() {
        let (_vm, thread) = crate::test::java21_thread().await.expect("thread");
        let result = get_glyph_cache_description(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_invisible_glyph_ptr() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_invisible_glyph_ptr(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
