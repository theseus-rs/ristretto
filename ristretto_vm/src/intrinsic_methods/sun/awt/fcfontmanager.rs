use crate::Result;
use crate::intrinsic_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/awt/FcFontManager";

/// Register all intrinsic methods for `sun.awt.FcFontManager`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "getFontPathNative",
        "(ZZ)Ljava/lang/String;",
        get_font_path_native,
    );
}

#[async_recursion(?Send)]
async fn get_font_path_native(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.FcFontManager.getFontPathNative(ZZ)Ljava/lang/String;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.FcFontManager.getFontPathNative(ZZ)Ljava/lang/String;"
    )]
    async fn test_get_font_path_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_font_path_native(thread, Parameters::default()).await;
    }
}
