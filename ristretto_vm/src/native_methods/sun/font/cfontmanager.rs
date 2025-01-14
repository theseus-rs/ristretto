use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.font.CFontManager`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/font/CFontManager";
    registry.register(
        class_name,
        "loadNativeDirFonts",
        "(Ljava/lang/String;)V",
        load_native_dir_fonts,
    );
    registry.register(class_name, "loadNativeFonts", "()V", load_native_fonts);
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

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/font/CFontManager";
        assert!(registry
            .method(class_name, "loadNativeDirFonts", "(Ljava/lang/String;)V")
            .is_some());
        assert!(registry
            .method(class_name, "loadNativeFonts", "()V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.font.CFontManager.loadNativeDirFonts(Ljava/lang/String;)V")]
    async fn test_load_native_dir_fonts() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = load_native_dir_fonts(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.font.CFontManager.loadNativeFonts()V")]
    async fn test_load_native_fonts() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = load_native_fonts(thread, Arguments::default()).await;
    }
}
