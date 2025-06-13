use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("java/awt/SplashScreen._close(J)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn close(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.awt.SplashScreen::_close(J)V");
}

#[intrinsic_method("java/awt/SplashScreen._getBounds(J)Ljava/awt/Rectangle;", Any)]
#[async_recursion(?Send)]
pub(crate) async fn get_bounds(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.awt.SplashScreen::_getBounds(J)Ljava/awt/Rectangle;");
}

#[intrinsic_method("java/awt/SplashScreen._getImageFileName(J)Ljava/lang/String;", Any)]
#[async_recursion(?Send)]
pub(crate) async fn get_image_file_name(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.awt.SplashScreen::_getImageFileName(J)Ljava/lang/String;");
}

#[intrinsic_method("java/awt/SplashScreen._getImageJarName(J)Ljava/lang/String;", Any)]
#[async_recursion(?Send)]
pub(crate) async fn get_image_jar_name(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.awt.SplashScreen::_getImageJarName(J)Ljava/lang/String;");
}

#[intrinsic_method("java/awt/SplashScreen._getInstance()J", Any)]
#[async_recursion(?Send)]
pub(crate) async fn get_instance(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.awt.SplashScreen::_getInstance()J");
}

#[intrinsic_method("java/awt/SplashScreen._getScaleFactor(J)F", Any)]
#[async_recursion(?Send)]
pub(crate) async fn get_scale_factor(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.awt.SplashScreen::_getScaleFactor(J)F");
}

#[intrinsic_method("java/awt/SplashScreen._isVisible(J)Z", Any)]
#[async_recursion(?Send)]
pub(crate) async fn is_visible(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.awt.SplashScreen::_isVisible(J)Z");
}

#[intrinsic_method("java/awt/SplashScreen._setImageData(J[B)Z", Any)]
#[async_recursion(?Send)]
pub(crate) async fn set_image_data(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.awt.SplashScreen::_setImageData(J[B)Z");
}

#[intrinsic_method("java/awt/SplashScreen._update(J[IIIIII)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn update(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.awt.SplashScreen::_update(J[IIIIII)V");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.awt.SplashScreen::_close(J)V")]
    async fn test_close() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = close(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.awt.SplashScreen::_getBounds(J)Ljava/awt/Rectangle;"
    )]
    async fn test_get_bounds() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_bounds(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.awt.SplashScreen::_getImageFileName(J)Ljava/lang/String;"
    )]
    async fn test_get_image_file_name() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_image_file_name(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.awt.SplashScreen::_getImageJarName(J)Ljava/lang/String;"
    )]
    async fn test_get_image_jar_name() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_image_jar_name(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.awt.SplashScreen::_getInstance()J")]
    async fn test_get_instance() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_instance(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.awt.SplashScreen::_getScaleFactor(J)F")]
    async fn test_get_scale_factor() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_scale_factor(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.awt.SplashScreen::_isVisible(J)Z")]
    async fn test_is_visible() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_visible(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.awt.SplashScreen::_setImageData(J[B)Z")]
    async fn test_set_image_data() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_image_data(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.awt.SplashScreen::_update(J[IIIIII)V")]
    async fn test_update() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = update(thread, Parameters::default()).await;
    }
}
