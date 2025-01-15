use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/font/CCharToGlyphMapper";

/// Register all native methods for `sun.font.CCharToGlyphMapper`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "countGlyphs", "(J)I", count_glyphs);
    registry.register(
        CLASS_NAME,
        "nativeCharsToGlyphs",
        "(JI[C[I)V",
        native_chars_to_glyphs,
    );
}

#[async_recursion(?Send)]
async fn count_glyphs(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.font.CCharToGlyphMapper.countGlyphs(J)I")
}

#[async_recursion(?Send)]
async fn native_chars_to_glyphs(
    _thread: Arc<Thread>,
    _arguments: Arguments,
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
        let _ = count_glyphs(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.font.CCharToGlyphMapper.nativeCharsToGlyphs(JI[C[I)V"
    )]
    async fn test_native_chars_to_glyphs() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_chars_to_glyphs(thread, Arguments::default()).await;
    }
}
