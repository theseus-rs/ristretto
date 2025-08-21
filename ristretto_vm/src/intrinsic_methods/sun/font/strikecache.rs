use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::{Any, GreaterThanOrEqual, LessThanOrEqual};
use ristretto_classfile::{JAVA_21, JAVA_25};
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("sun/font/StrikeCache.freeIntMemory([IJ)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn free_int_memory(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.StrikeCache.freeIntMemory([IJ)V")
}

#[intrinsic_method("sun/font/StrikeCache.freeIntPointer(I)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn free_int_pointer(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.StrikeCache.freeIntPointer(I)V")
}

#[intrinsic_method("sun/font/StrikeCache.freeLongMemory([JJ)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn free_long_memory(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.StrikeCache.freeLongMemory([JJ)V")
}

#[intrinsic_method("sun/font/StrikeCache.freeLongPointer(J)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn free_long_pointer(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.StrikeCache.freeLongPointer(J)V")
}

#[intrinsic_method(
    "sun/font/StrikeCache.getGlyphCacheDescription([J)V",
    LessThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_glyph_cache_description(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.StrikeCache.getGlyphCacheDescription([J)V")
}

#[intrinsic_method(
    "sun/font/StrikeCache.getInvisibleGlyphPtr()J",
    GreaterThanOrEqual(JAVA_25)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_invisible_glyph_ptr(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.StrikeCache.getInvisibleGlyphPtr()J")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.font.StrikeCache.freeIntMemory([IJ)V")]
    async fn test_free_int_memory() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = free_int_memory(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.font.StrikeCache.freeIntPointer(I)V")]
    async fn test_free_int_pointer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = free_int_pointer(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.font.StrikeCache.freeLongMemory([JJ)V")]
    async fn test_free_long_memory() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = free_long_memory(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.font.StrikeCache.freeLongPointer(J)V")]
    async fn test_free_long_pointer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = free_long_pointer(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.font.StrikeCache.getGlyphCacheDescription([J)V"
    )]
    async fn test_get_glyph_cache_description() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_glyph_cache_description(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.font.StrikeCache.getInvisibleGlyphPtr()J")]
    async fn test_get_invisible_glyph_ptr() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_invisible_glyph_ptr(thread, Parameters::default()).await;
    }
}
