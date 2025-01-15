use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/awt/X11GraphicsEnvironment";

/// Register all native methods for `sun.awt.X11GraphicsEnvironment`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "checkShmExt", "()I", check_shm_ext);
    registry.register(
        CLASS_NAME,
        "getDefaultScreenNum",
        "()I",
        get_default_screen_num,
    );
    registry.register(
        CLASS_NAME,
        "getDisplayString",
        "()Ljava/lang/String;",
        get_display_string,
    );
    registry.register(CLASS_NAME, "getNumScreens", "()I", get_num_screens);
    registry.register(
        CLASS_NAME,
        "getXineramaCenterPoint",
        "()Ljava/awt/Point;",
        get_xinerama_center_point,
    );
    registry.register(CLASS_NAME, "initDisplay", "(Z)V", init_display);
    registry.register(CLASS_NAME, "initGLX", "()Z", init_glx);
    registry.register(CLASS_NAME, "initXRender", "(ZZ)Z", init_x_render);
    registry.register(CLASS_NAME, "pRunningXinerama", "()Z", p_running_xinerama);
}

#[async_recursion(?Send)]
async fn check_shm_ext(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsEnvironment.checkShmExt()I")
}

#[async_recursion(?Send)]
async fn get_default_screen_num(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsEnvironment.getDefaultScreenNum()I")
}

#[async_recursion(?Send)]
async fn get_display_string(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsEnvironment.getDisplayString()Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn get_num_screens(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsEnvironment.getNumScreens()I")
}

#[async_recursion(?Send)]
async fn get_xinerama_center_point(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsEnvironment.getXineramaCenterPoint()Ljava/awt/Point;")
}

#[async_recursion(?Send)]
async fn init_display(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsEnvironment.initDisplay(Z)V")
}

#[async_recursion(?Send)]
async fn init_glx(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsEnvironment.initGLX()Z")
}

#[async_recursion(?Send)]
async fn init_x_render(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsEnvironment.initXRender(ZZ)Z")
}

#[async_recursion(?Send)]
async fn p_running_xinerama(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsEnvironment.pRunningXinerama()Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.awt.X11GraphicsEnvironment.checkShmExt()I")]
    async fn test_check_shm_ext() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = check_shm_ext(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.X11GraphicsEnvironment.getDefaultScreenNum()I"
    )]
    async fn test_get_default_screen_num() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_default_screen_num(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.X11GraphicsEnvironment.getDisplayString()Ljava/lang/String;"
    )]
    async fn test_get_display_string() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_display_string(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.X11GraphicsEnvironment.getNumScreens()I"
    )]
    async fn test_get_num_screens() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_num_screens(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.X11GraphicsEnvironment.getXineramaCenterPoint()Ljava/awt/Point;"
    )]
    async fn test_get_xinerama_center_point() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_xinerama_center_point(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.X11GraphicsEnvironment.initDisplay(Z)V"
    )]
    async fn test_init_display() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_display(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.awt.X11GraphicsEnvironment.initGLX()Z")]
    async fn test_init_glx() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_glx(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.X11GraphicsEnvironment.initXRender(ZZ)Z"
    )]
    async fn test_init_x_render() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_x_render(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.X11GraphicsEnvironment.pRunningXinerama()Z"
    )]
    async fn test_p_running_xinerama() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = p_running_xinerama(thread, Arguments::default()).await;
    }
}
