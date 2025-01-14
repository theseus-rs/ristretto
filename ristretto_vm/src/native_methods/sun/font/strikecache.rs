use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.font.StrikeCache`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/font/StrikeCache";
    registry.register(class_name, "freeIntMemory", "([IJ)V", free_int_memory);
    registry.register(class_name, "freeIntPointer", "(I)V", free_int_pointer);
    registry.register(class_name, "freeLongMemory", "([JJ)V", free_long_memory);
    registry.register(class_name, "freeLongPointer", "(J)V", free_long_pointer);
    registry.register(
        class_name,
        "getGlyphCacheDescription",
        "([J)V",
        get_glyph_cache_description,
    );
}

#[async_recursion(?Send)]
async fn free_int_memory(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.font.StrikeCache.freeIntMemory([IJ)V")
}

#[async_recursion(?Send)]
async fn free_int_pointer(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.font.StrikeCache.freeIntPointer(I)V")
}

#[async_recursion(?Send)]
async fn free_long_memory(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.font.StrikeCache.freeLongMemory([JJ)V")
}

#[async_recursion(?Send)]
async fn free_long_pointer(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.font.StrikeCache.freeLongPointer(J)V")
}

#[async_recursion(?Send)]
async fn get_glyph_cache_description(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.font.StrikeCache.getGlyphCacheDescription([J)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/font/StrikeCache";
        assert!(registry
            .method(class_name, "freeIntMemory", "([IJ)V")
            .is_some());
        assert!(registry
            .method(class_name, "freeIntPointer", "(I)V")
            .is_some());
        assert!(registry
            .method(class_name, "freeLongMemory", "([JJ)V")
            .is_some());
        assert!(registry
            .method(class_name, "freeLongPointer", "(J)V")
            .is_some());
        assert!(registry
            .method(class_name, "getGlyphCacheDescription", "([J)V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.font.StrikeCache.freeIntMemory([IJ)V")]
    async fn test_free_int_memory() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = free_int_memory(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.font.StrikeCache.freeIntPointer(I)V")]
    async fn test_free_int_pointer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = free_int_pointer(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.font.StrikeCache.freeLongMemory([JJ)V")]
    async fn test_free_long_memory() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = free_long_memory(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.font.StrikeCache.freeLongPointer(J)V")]
    async fn test_free_long_pointer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = free_long_pointer(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.font.StrikeCache.getGlyphCacheDescription([J)V")]
    async fn test_get_glyph_cache_description() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_glyph_cache_description(thread, Arguments::default()).await;
    }
}
