use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("java/awt/SplashScreen._close(J)V", Any)]
#[async_method]
pub async fn close<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("java.awt.SplashScreen::_close(J)V".to_string()).into())
}

#[intrinsic_method("java/awt/SplashScreen._getBounds(J)Ljava/awt/Rectangle;", Any)]
#[async_method]
pub async fn get_bounds<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "java.awt.SplashScreen::_getBounds(J)Ljava/awt/Rectangle;".to_string(),
    )
    .into())
}

#[intrinsic_method("java/awt/SplashScreen._getImageFileName(J)Ljava/lang/String;", Any)]
#[async_method]
pub async fn get_image_file_name<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "java.awt.SplashScreen::_getImageFileName(J)Ljava/lang/String;".to_string(),
    )
    .into())
}

#[intrinsic_method("java/awt/SplashScreen._getImageJarName(J)Ljava/lang/String;", Any)]
#[async_method]
pub async fn get_image_jar_name<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "java.awt.SplashScreen::_getImageJarName(J)Ljava/lang/String;".to_string(),
    )
    .into())
}

#[intrinsic_method("java/awt/SplashScreen._getInstance()J", Any)]
#[async_method]
pub async fn get_instance<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("java.awt.SplashScreen::_getInstance()J".to_string())
            .into(),
    )
}

#[intrinsic_method("java/awt/SplashScreen._getScaleFactor(J)F", Any)]
#[async_method]
pub async fn get_scale_factor<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("java.awt.SplashScreen::_getScaleFactor(J)F".to_string())
            .into(),
    )
}

#[intrinsic_method("java/awt/SplashScreen._isVisible(J)Z", Any)]
#[async_method]
pub async fn is_visible<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("java.awt.SplashScreen::_isVisible(J)Z".to_string()).into())
}

#[intrinsic_method("java/awt/SplashScreen._setImageData(J[B)Z", Any)]
#[async_method]
pub async fn set_image_data<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("java.awt.SplashScreen::_setImageData(J[B)Z".to_string())
            .into(),
    )
}

#[intrinsic_method("java/awt/SplashScreen._update(J[IIIIII)V", Any)]
#[async_method]
pub async fn update<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("java.awt.SplashScreen::_update(J[IIIIII)V".to_string())
            .into(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_close() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = close(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_bounds() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_bounds(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_image_file_name() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_image_file_name(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_image_jar_name() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_image_jar_name(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_instance() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_instance(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_scale_factor() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_scale_factor(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_is_visible() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = is_visible(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_image_data() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_image_data(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_update() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = update(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
