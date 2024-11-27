use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.font.CCharToGlyphMapper`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/font/CCharToGlyphMapper";
    registry.register(class_name, "countGlyphs", "(J)I", count_glyphs);
    registry.register(
        class_name,
        "nativeCharsToGlyphs",
        "(JI[C[I)V",
        native_chars_to_glyphs,
    );
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn count_glyphs(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn native_chars_to_glyphs(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}
