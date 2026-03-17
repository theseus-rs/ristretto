use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/awt/X11GraphicsEnvironment.checkShmExt()I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn check_shm_ext<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.X11GraphicsEnvironment.checkShmExt()I".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/awt/X11GraphicsEnvironment.getDefaultScreenNum()I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_default_screen_num<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.X11GraphicsEnvironment.getDefaultScreenNum()I".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/awt/X11GraphicsEnvironment.getDisplayString()Ljava/lang/String;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_display_string<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.X11GraphicsEnvironment.getDisplayString()Ljava/lang/String;".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/awt/X11GraphicsEnvironment.getNumScreens()I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_num_screens<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.X11GraphicsEnvironment.getNumScreens()I".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/awt/X11GraphicsEnvironment.getXineramaCenterPoint()Ljava/awt/Point;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_xinerama_center_point<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.X11GraphicsEnvironment.getXineramaCenterPoint()Ljava/awt/Point;".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/awt/X11GraphicsEnvironment.initDisplay(Z)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn init_display<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.X11GraphicsEnvironment.initDisplay(Z)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/awt/X11GraphicsEnvironment.initGLX()Z", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn init_glx<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun.awt.X11GraphicsEnvironment.initGLX()Z".to_string())
            .into(),
    )
}

#[intrinsic_method(
    "sun/awt/X11GraphicsEnvironment.initXRender(ZZ)Z",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn init_x_render<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.X11GraphicsEnvironment.initXRender(ZZ)Z".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/awt/X11GraphicsEnvironment.pRunningXinerama()Z",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn p_running_xinerama<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.X11GraphicsEnvironment.pRunningXinerama()Z".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_check_shm_ext() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = check_shm_ext(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_default_screen_num() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_default_screen_num(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_display_string() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_display_string(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_num_screens() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_num_screens(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_xinerama_center_point() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_xinerama_center_point(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_init_display() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_display(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_init_glx() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_glx(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_init_x_render() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_x_render(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_p_running_xinerama() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = p_running_xinerama(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
