use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.awt.X11GraphicsConfig`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/awt/X11GraphicsConfig";
    registry.register(class_name, "createBackBuffer", "(JI)J", create_back_buffer);
    registry.register(class_name, "destroyBackBuffer", "(J)V", destroy_back_buffer);
    registry.register(class_name, "dispose", "(J)V", dispose);
    registry.register(class_name, "getNumColors", "()I", get_num_colors);
    registry.register(class_name, "getXResolution", "(I)D", get_x_resolution);
    registry.register(class_name, "getYResolution", "(I)D", get_y_resolution);
    registry.register(class_name, "init", "(II)V", init);
    registry.register(class_name, "initIDs", "()V", init_ids);
    registry.register(
        class_name,
        "isTranslucencyCapable",
        "(J)Z",
        is_translucency_capable,
    );
    registry.register(
        class_name,
        "makeColorModel",
        "()Ljava/awt/image/ColorModel;",
        make_color_model,
    );
    registry.register(
        class_name,
        "pGetBounds",
        "(I)Ljava/awt/Rectangle;",
        p_get_bounds,
    );
    registry.register(class_name, "swapBuffers", "(JI)V", swap_buffers);
}

#[async_recursion(?Send)]
async fn create_back_buffer(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsConfig.createBackBuffer(JI)J")
}

#[async_recursion(?Send)]
async fn destroy_back_buffer(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsConfig.destroyBackBuffer(J)V")
}

#[async_recursion(?Send)]
async fn dispose(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsConfig.dispose(J)V")
}

#[async_recursion(?Send)]
async fn get_num_colors(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsConfig.getNumColors()I")
}

#[async_recursion(?Send)]
async fn get_x_resolution(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsConfig.getXResolution(I)D")
}

#[async_recursion(?Send)]
async fn get_y_resolution(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsConfig.getYResolution(I)D")
}

#[async_recursion(?Send)]
async fn init(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsConfig.init(II)V")
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn is_translucency_capable(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsConfig.isTranslucencyCapable(J)Z")
}

#[async_recursion(?Send)]
async fn make_color_model(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsConfig.makeColorModel()Ljava/awt/image/ColorModel;")
}

#[async_recursion(?Send)]
async fn p_get_bounds(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsConfig.pGetBounds(I)Ljava/awt/Rectangle;")
}

#[async_recursion(?Send)]
async fn swap_buffers(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsConfig.swapBuffers(JI)V")
}
