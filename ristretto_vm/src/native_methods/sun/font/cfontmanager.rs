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
