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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/awt/X11GraphicsConfig";
        assert!(registry
            .method(class_name, "createBackBuffer", "(JI)J")
            .is_some());
        assert!(registry
            .method(class_name, "destroyBackBuffer", "(J)V")
            .is_some());
        assert!(registry.method(class_name, "dispose", "(J)V").is_some());
        assert!(registry.method(class_name, "getNumColors", "()I").is_some());
        assert!(registry
            .method(class_name, "getXResolution", "(I)D")
            .is_some());
        assert!(registry
            .method(class_name, "getYResolution", "(I)D")
            .is_some());
        assert!(registry.method(class_name, "init", "(II)V").is_some());
        assert!(registry.method(class_name, "initIDs", "()V").is_some());
        assert!(registry
            .method(class_name, "isTranslucencyCapable", "(J)Z")
            .is_some());
        assert!(registry
            .method(
                class_name,
                "makeColorModel",
                "()Ljava/awt/image/ColorModel;"
            )
            .is_some());
        assert!(registry
            .method(class_name, "pGetBounds", "(I)Ljava/awt/Rectangle;")
            .is_some());
        assert!(registry
            .method(class_name, "swapBuffers", "(JI)V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.awt.X11GraphicsConfig.createBackBuffer(JI)J")]
    async fn test_create_back_buffer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create_back_buffer(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.awt.X11GraphicsConfig.destroyBackBuffer(J)V")]
    async fn test_destroy_back_buffer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = destroy_back_buffer(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.awt.X11GraphicsConfig.dispose(J)V")]
    async fn test_dispose() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = dispose(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.awt.X11GraphicsConfig.getNumColors()I")]
    async fn test_get_num_colors() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_num_colors(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.awt.X11GraphicsConfig.getXResolution(I)D")]
    async fn test_get_x_resolution() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_x_resolution(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.awt.X11GraphicsConfig.getYResolution(I)D")]
    async fn test_get_y_resolution() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_y_resolution(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.awt.X11GraphicsConfig.init(II)V")]
    async fn test_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.awt.X11GraphicsConfig.isTranslucencyCapable(J)Z")]
    async fn test_is_translucency_capable() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_translucency_capable(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.awt.X11GraphicsConfig.makeColorModel()Ljava/awt/image/ColorModel;"
    )]
    async fn test_make_color_model() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = make_color_model(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.awt.X11GraphicsConfig.pGetBounds(I)Ljava/awt/Rectangle;")]
    async fn test_p_get_bounds() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = p_get_bounds(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.awt.X11GraphicsConfig.swapBuffers(JI)V")]
    async fn test_swap_buffers() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = swap_buffers(thread, Arguments::default()).await;
    }
}
