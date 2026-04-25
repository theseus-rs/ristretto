use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/awt/SunToolkit.closeSplashScreen()V", Any)]
#[async_method]
pub async fn close_splash_screen<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun.awt.SunToolkit.closeSplashScreen()V".to_string())
            .into(),
    )
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_close_splash_screen() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = close_splash_screen(thread, Parameters::default()).await;
        assert_eq!(
            "sun.awt.SunToolkit.closeSplashScreen()V",
            result.unwrap_err().to_string()
        );
    }
}
