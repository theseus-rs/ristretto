use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.awt.X11GraphicsEnvironment`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/awt/X11GraphicsEnvironment";
    registry.register(class_name, "checkShmExt", "()I", check_shm_ext);
    registry.register(
        class_name,
        "getDefaultScreenNum",
        "()I",
        get_default_screen_num,
    );
    registry.register(
        class_name,
        "getDisplayString",
        "()Ljava/lang/String;",
        get_display_string,
    );
    registry.register(class_name, "getNumScreens", "()I", get_num_screens);
    registry.register(
        class_name,
        "getXineramaCenterPoint",
        "()Ljava/awt/Point;",
        get_xinerama_center_point,
    );
    registry.register(class_name, "initDisplay", "(Z)V", init_display);
    registry.register(class_name, "initGLX", "()Z", init_glx);
    registry.register(class_name, "initXRender", "(ZZ)Z", init_x_render);
    registry.register(class_name, "pRunningXinerama", "()Z", p_running_xinerama);
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
