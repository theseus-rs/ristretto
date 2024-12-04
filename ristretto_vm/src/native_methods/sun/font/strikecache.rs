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
    todo!()
}

#[async_recursion(?Send)]
async fn free_int_pointer(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn free_long_memory(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn free_long_pointer(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[async_recursion(?Send)]
async fn get_glyph_cache_description(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}
