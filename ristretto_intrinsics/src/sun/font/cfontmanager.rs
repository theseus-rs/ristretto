use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/font/CFontManager.loadNativeDirFonts(Ljava/lang/String;)V", Any)]
#[async_method]
pub async fn load_native_dir_fonts<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.font.CFontManager.loadNativeDirFonts(Ljava/lang/String;)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/font/CFontManager.loadNativeFonts()V", Any)]
#[async_method]
pub async fn load_native_fonts<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun.font.CFontManager.loadNativeFonts()V".to_string())
            .into(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_load_native_dir_fonts() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = load_native_dir_fonts(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_load_native_fonts() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = load_native_fonts(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
