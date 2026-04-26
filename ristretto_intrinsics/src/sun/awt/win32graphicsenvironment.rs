use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/awt/Win32GraphicsEnvironment.getDefaultScreen()I", Any)]
#[async_method]
pub async fn get_default_screen<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/Win32GraphicsEnvironment.getDefaultScreen()I".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/Win32GraphicsEnvironment.getNumScreens()I", Any)]
#[async_method]
pub async fn get_num_screens<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/Win32GraphicsEnvironment.getNumScreens()I".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/Win32GraphicsEnvironment.getXResolution()I", Any)]
#[async_method]
pub async fn get_xresolution<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/Win32GraphicsEnvironment.getXResolution()I".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/Win32GraphicsEnvironment.getYResolution()I", Any)]
#[async_method]
pub async fn get_yresolution<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/Win32GraphicsEnvironment.getYResolution()I".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/Win32GraphicsEnvironment.initDisplay()V", Any)]
#[async_method]
pub async fn init_display<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/Win32GraphicsEnvironment.initDisplay()V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/Win32GraphicsEnvironment.isVistaOS()Z", Any)]
#[async_method]
pub async fn is_vista_os<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/Win32GraphicsEnvironment.isVistaOS()Z".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_default_screen() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_default_screen(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/Win32GraphicsEnvironment.getDefaultScreen()I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_num_screens() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_num_screens(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/Win32GraphicsEnvironment.getNumScreens()I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_xresolution() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_xresolution(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/Win32GraphicsEnvironment.getXResolution()I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_yresolution() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_yresolution(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/Win32GraphicsEnvironment.getYResolution()I",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_init_display() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_display(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/Win32GraphicsEnvironment.initDisplay()V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_is_vista_os() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = is_vista_os(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/Win32GraphicsEnvironment.isVistaOS()Z",
            result.unwrap_err().to_string()
        );
    }
}
