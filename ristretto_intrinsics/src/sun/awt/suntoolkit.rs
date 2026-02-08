use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/awt/SunToolkit.closeSplashScreen()V", Any)]
#[async_method]
pub async fn close_splash_screen<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.SunToolkit.closeSplashScreen()V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.awt.SunToolkit.closeSplashScreen()V")]
    async fn test_close_splash_screen() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = close_splash_screen(thread, Parameters::default()).await;
    }
}
