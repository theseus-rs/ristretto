use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/font/CFontManager";

/// Register all native methods for `sun.font.CFontManager`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "loadNativeDirFonts",
        "(Ljava/lang/String;)V",
        load_native_dir_fonts,
    );
    registry.register(CLASS_NAME, "loadNativeFonts", "()V", load_native_fonts);
}

#[async_recursion(?Send)]
async fn load_native_dir_fonts(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.font.CFontManager.loadNativeDirFonts(Ljava/lang/String;)V")
}

#[async_recursion(?Send)]
async fn load_native_fonts(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.font.CFontManager.loadNativeFonts()V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.font.CFontManager.loadNativeDirFonts(Ljava/lang/String;)V"
    )]
    async fn test_load_native_dir_fonts() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = load_native_dir_fonts(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.font.CFontManager.loadNativeFonts()V")]
    async fn test_load_native_fonts() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = load_native_fonts(thread, Arguments::default()).await;
    }
}
