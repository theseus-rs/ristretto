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
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _splash_ptr = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("java.awt.SplashScreen::_close(J)V".to_string()).into())
}

#[intrinsic_method("java/awt/SplashScreen._getBounds(J)Ljava/awt/Rectangle;", Any)]
#[async_method]
pub async fn get_bounds<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _splash_ptr = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java.awt.SplashScreen::_getBounds(J)Ljava/awt/Rectangle;".to_string(),
    )
    .into())
}

#[intrinsic_method("java/awt/SplashScreen._getImageFileName(J)Ljava/lang/String;", Any)]
#[async_method]
pub async fn get_image_file_name<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _splash_ptr = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "java.awt.SplashScreen::_getImageFileName(J)Ljava/lang/String;".to_string(),
    )
    .into())
}

#[intrinsic_method("java/awt/SplashScreen._getImageJarName(J)Ljava/lang/String;", Any)]
#[async_method]
pub async fn get_image_jar_name<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _splash_ptr = parameters.pop_long()?;
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
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _splash_ptr = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("java.awt.SplashScreen::_getScaleFactor(J)F".to_string())
            .into(),
    )
}

#[intrinsic_method("java/awt/SplashScreen._isVisible(J)Z", Any)]
#[async_method]
pub async fn is_visible<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _splash_ptr = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError("java.awt.SplashScreen::_isVisible(J)Z".to_string()).into())
}

#[intrinsic_method("java/awt/SplashScreen._setImageData(J[B)Z", Any)]
#[async_method]
pub async fn set_image_data<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _data = parameters.pop_reference()?;
    let _splash_ptr = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("java.awt.SplashScreen::_setImageData(J[B)Z".to_string())
            .into(),
    )
}

#[intrinsic_method("java/awt/SplashScreen._update(J[IIIIII)V", Any)]
#[async_method]
pub async fn update<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _scanline_stride = parameters.pop_int()?;
    let _height = parameters.pop_int()?;
    let _width = parameters.pop_int()?;
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _data = parameters.pop_reference()?;
    let _splash_ptr = parameters.pop_long()?;
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
        let result = close(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "java.awt.SplashScreen::_close(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_bounds() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_bounds(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "java.awt.SplashScreen::_getBounds(J)Ljava/awt/Rectangle;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_image_file_name() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_image_file_name(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "java.awt.SplashScreen::_getImageFileName(J)Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_image_jar_name() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_image_jar_name(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "java.awt.SplashScreen::_getImageJarName(J)Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_instance() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_instance(thread, Parameters::default()).await;
        assert_eq!(
            "java.awt.SplashScreen::_getInstance()J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_scale_factor() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_scale_factor(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "java.awt.SplashScreen::_getScaleFactor(J)F",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_is_visible() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = is_visible(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "java.awt.SplashScreen::_isVisible(J)Z",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_set_image_data() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_image_data(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "java.awt.SplashScreen::_setImageData(J[B)Z",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_update() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = update(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "java.awt.SplashScreen::_update(J[IIIIII)V",
            result.unwrap_err().to_string()
        );
    }
}
