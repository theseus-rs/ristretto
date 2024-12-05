use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.awt.SplashScreen`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/awt/SplashScreen";
    registry.register(class_name, "_close", "(J)V", close);
    registry.register(
        class_name,
        "_getBounds",
        "(J)Ljava/awt/Rectangle;",
        get_bounds,
    );
    registry.register(
        class_name,
        "_getImageFileName",
        "(J)Ljava/lang/String;",
        get_image_file_name,
    );
    registry.register(
        class_name,
        "_getImageJarName",
        "(J)Ljava/lang/String;",
        get_image_jar_name,
    );
    registry.register(class_name, "_getInstance", "()J", get_instance);
    registry.register(class_name, "_getScaleFactor", "(J)F", get_scale_factor);
    registry.register(class_name, "_isVisible", "(J)Z", is_visible);
    registry.register(class_name, "_setImageData", "(J[B)Z", set_image_data);
    registry.register(class_name, "_update", "(J[IIIIII)V", update);
}

#[async_recursion(?Send)]
async fn close(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.awt.SplashScreen::_close(J)V");
}

#[async_recursion(?Send)]
async fn get_bounds(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.awt.SplashScreen::_getBounds(J)Ljava/awt/Rectangle;");
}

#[async_recursion(?Send)]
async fn get_image_file_name(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.awt.SplashScreen::_getImageFileName(J)Ljava/lang/String;");
}

#[async_recursion(?Send)]
async fn get_image_jar_name(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.awt.SplashScreen::_getImageJarName(J)Ljava/lang/String;");
}

#[async_recursion(?Send)]
async fn get_instance(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.awt.SplashScreen::_getInstance()J");
}

#[async_recursion(?Send)]
async fn get_scale_factor(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.awt.SplashScreen::_getScaleFactor(J)F");
}

#[async_recursion(?Send)]
async fn is_visible(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.awt.SplashScreen::_isVisible(J)Z");
}

#[async_recursion(?Send)]
async fn set_image_data(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.awt.SplashScreen::_setImageData(J[B)Z");
}

#[async_recursion(?Send)]
async fn update(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.awt.SplashScreen::_update(J[IIIIII)V");
}
