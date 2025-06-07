use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/awt/X11GraphicsEnvironment.checkShmExt()I",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn check_shm_ext(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsEnvironment.checkShmExt()I")
}

#[intrinsic_method(
    "sun/awt/X11GraphicsEnvironment.getDefaultScreenNum()I",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_default_screen_num(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsEnvironment.getDefaultScreenNum()I")
}

#[intrinsic_method(
    "sun/awt/X11GraphicsEnvironment.getDisplayString()Ljava/lang/String;",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_display_string(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsEnvironment.getDisplayString()Ljava/lang/String;")
}

#[intrinsic_method(
    "sun/awt/X11GraphicsEnvironment.getNumScreens()I",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_num_screens(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsEnvironment.getNumScreens()I")
}

#[intrinsic_method(
    "sun/awt/X11GraphicsEnvironment.getXineramaCenterPoint()Ljava/awt/Point;",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_xinerama_center_point(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsEnvironment.getXineramaCenterPoint()Ljava/awt/Point;")
}

#[intrinsic_method(
    "sun/awt/X11GraphicsEnvironment.initDisplay(Z)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn init_display(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsEnvironment.initDisplay(Z)V")
}

#[intrinsic_method("sun/awt/X11GraphicsEnvironment.initGLX()Z", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn init_glx(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsEnvironment.initGLX()Z")
}

#[intrinsic_method(
    "sun/awt/X11GraphicsEnvironment.initXRender(ZZ)Z",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn init_x_render(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsEnvironment.initXRender(ZZ)Z")
}

#[intrinsic_method(
    "sun/awt/X11GraphicsEnvironment.pRunningXinerama()Z",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn p_running_xinerama(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsEnvironment.pRunningXinerama()Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.awt.X11GraphicsEnvironment.checkShmExt()I")]
    async fn test_check_shm_ext() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = check_shm_ext(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.X11GraphicsEnvironment.getDefaultScreenNum()I"
    )]
    async fn test_get_default_screen_num() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_default_screen_num(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.X11GraphicsEnvironment.getDisplayString()Ljava/lang/String;"
    )]
    async fn test_get_display_string() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_display_string(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.X11GraphicsEnvironment.getNumScreens()I"
    )]
    async fn test_get_num_screens() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_num_screens(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.X11GraphicsEnvironment.getXineramaCenterPoint()Ljava/awt/Point;"
    )]
    async fn test_get_xinerama_center_point() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_xinerama_center_point(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.X11GraphicsEnvironment.initDisplay(Z)V"
    )]
    async fn test_init_display() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_display(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.awt.X11GraphicsEnvironment.initGLX()Z")]
    async fn test_init_glx() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_glx(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.X11GraphicsEnvironment.initXRender(ZZ)Z"
    )]
    async fn test_init_x_render() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_x_render(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.X11GraphicsEnvironment.pRunningXinerama()Z"
    )]
    async fn test_p_running_xinerama() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = p_running_xinerama(thread, Parameters::default()).await;
    }
}
