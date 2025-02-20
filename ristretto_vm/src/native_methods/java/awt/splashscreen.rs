use crate::Result;
use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "java/awt/SplashScreen";

/// Register all native methods for `java.awt.SplashScreen`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "_close", "(J)V", close);
    registry.register(
        CLASS_NAME,
        "_getBounds",
        "(J)Ljava/awt/Rectangle;",
        get_bounds,
    );
    registry.register(
        CLASS_NAME,
        "_getImageFileName",
        "(J)Ljava/lang/String;",
        get_image_file_name,
    );
    registry.register(
        CLASS_NAME,
        "_getImageJarName",
        "(J)Ljava/lang/String;",
        get_image_jar_name,
    );
    registry.register(CLASS_NAME, "_getInstance", "()J", get_instance);
    registry.register(CLASS_NAME, "_getScaleFactor", "(J)F", get_scale_factor);
    registry.register(CLASS_NAME, "_isVisible", "(J)Z", is_visible);
    registry.register(CLASS_NAME, "_setImageData", "(J[B)Z", set_image_data);
    registry.register(CLASS_NAME, "_update", "(J[IIIIII)V", update);
}

#[async_recursion(?Send)]
async fn close(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.awt.SplashScreen::_close(J)V");
}

#[async_recursion(?Send)]
async fn get_bounds(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.awt.SplashScreen::_getBounds(J)Ljava/awt/Rectangle;");
}

#[async_recursion(?Send)]
async fn get_image_file_name(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.awt.SplashScreen::_getImageFileName(J)Ljava/lang/String;");
}

#[async_recursion(?Send)]
async fn get_image_jar_name(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.awt.SplashScreen::_getImageJarName(J)Ljava/lang/String;");
}

#[async_recursion(?Send)]
async fn get_instance(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.awt.SplashScreen::_getInstance()J");
}

#[async_recursion(?Send)]
async fn get_scale_factor(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.awt.SplashScreen::_getScaleFactor(J)F");
}

#[async_recursion(?Send)]
async fn is_visible(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.awt.SplashScreen::_isVisible(J)Z");
}

#[async_recursion(?Send)]
async fn set_image_data(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.awt.SplashScreen::_setImageData(J[B)Z");
}

#[async_recursion(?Send)]
async fn update(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
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
