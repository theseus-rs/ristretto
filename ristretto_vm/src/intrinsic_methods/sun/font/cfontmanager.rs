use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("sun/font/CFontManager.loadNativeDirFonts(Ljava/lang/String;)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn load_native_dir_fonts(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.font.CFontManager.loadNativeDirFonts(Ljava/lang/String;)V")
}

#[intrinsic_method("sun/font/CFontManager.loadNativeFonts()V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn load_native_fonts(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
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
        let _ = load_native_dir_fonts(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.font.CFontManager.loadNativeFonts()V")]
    async fn test_load_native_fonts() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = load_native_fonts(thread, Parameters::default()).await;
    }
}
