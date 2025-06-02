use crate::Result;
use crate::intrinsic_methods::registry::{JAVA_21, JAVA_24, MethodRegistry};
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/font/StrikeCache";

/// Register all intrinsic methods for `sun.font.StrikeCache`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= JAVA_21 {
        registry.register(
            CLASS_NAME,
            "getGlyphCacheDescription",
            "([J)V",
            get_glyph_cache_description,
        );
    }

    if registry.java_major_version() >= JAVA_24 {
        registry.register(
            CLASS_NAME,
            "getInvisibleGlyphPtr",
            "()J",
            get_invisible_glyph_ptr,
        );
    }

    registry.register(CLASS_NAME, "freeIntMemory", "([IJ)V", free_int_memory);
    registry.register(CLASS_NAME, "freeIntPointer", "(I)V", free_int_pointer);
    registry.register(CLASS_NAME, "freeLongMemory", "([JJ)V", free_long_memory);
    registry.register(CLASS_NAME, "freeLongPointer", "(J)V", free_long_pointer);
}

#[async_recursion(?Send)]
async fn free_int_memory(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.font.StrikeCache.freeIntMemory([IJ)V")
}

#[async_recursion(?Send)]
async fn free_int_pointer(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.font.StrikeCache.freeIntPointer(I)V")
}

#[async_recursion(?Send)]
async fn free_long_memory(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.font.StrikeCache.freeLongMemory([JJ)V")
}

#[async_recursion(?Send)]
async fn free_long_pointer(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.font.StrikeCache.freeLongPointer(J)V")
}

#[async_recursion(?Send)]
async fn get_glyph_cache_description(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.StrikeCache.getGlyphCacheDescription([J)V")
}

#[async_recursion(?Send)]
async fn get_invisible_glyph_ptr(
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
