use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/awt/X11GraphicsConfig.createBackBuffer(JI)J",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn create_back_buffer(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsConfig.createBackBuffer(JI)J")
}

#[intrinsic_method(
    "sun/awt/X11GraphicsConfig.destroyBackBuffer(J)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn destroy_back_buffer(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsConfig.destroyBackBuffer(J)V")
}

#[intrinsic_method("sun/awt/X11GraphicsConfig.dispose(J)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub(crate) async fn dispose(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsConfig.dispose(J)V")
}

#[intrinsic_method("sun/awt/X11GraphicsConfig.getNumColors()I", LessThanOrEqual(JAVA_8))]
#[async_method]
pub(crate) async fn get_num_colors(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsConfig.getNumColors()I")
}

#[intrinsic_method(
    "sun/awt/X11GraphicsConfig.getXResolution(I)D",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn get_x_resolution(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsConfig.getXResolution(I)D")
}

#[intrinsic_method(
    "sun/awt/X11GraphicsConfig.getYResolution(I)D",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn get_y_resolution(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsConfig.getYResolution(I)D")
}

#[intrinsic_method("sun/awt/X11GraphicsConfig.init(II)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub(crate) async fn init(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsConfig.init(II)V")
}

#[intrinsic_method("sun/awt/X11GraphicsConfig.initIDs()V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub(crate) async fn init_ids(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "sun/awt/X11GraphicsConfig.isTranslucencyCapable(J)Z",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn is_translucency_capable(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsConfig.isTranslucencyCapable(J)Z")
}

#[intrinsic_method(
    "sun/awt/X11GraphicsConfig.makeColorModel()Ljava/awt/image/ColorModel;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn make_color_model(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsConfig.makeColorModel()Ljava/awt/image/ColorModel;")
}

#[intrinsic_method(
    "sun/awt/X11GraphicsConfig.pGetBounds(I)Ljava/awt/Rectangle;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn p_get_bounds(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsConfig.pGetBounds(I)Ljava/awt/Rectangle;")
}

#[intrinsic_method("sun/awt/X11GraphicsConfig.swapBuffers(JI)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub(crate) async fn swap_buffers(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.awt.X11GraphicsConfig.swapBuffers(JI)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.X11GraphicsConfig.createBackBuffer(JI)J"
    )]
    async fn test_create_back_buffer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create_back_buffer(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.X11GraphicsConfig.destroyBackBuffer(J)V"
    )]
    async fn test_destroy_back_buffer() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = destroy_back_buffer(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.awt.X11GraphicsConfig.dispose(J)V")]
    async fn test_dispose() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = dispose(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.awt.X11GraphicsConfig.getNumColors()I")]
    async fn test_get_num_colors() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_num_colors(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.awt.X11GraphicsConfig.getXResolution(I)D")]
    async fn test_get_x_resolution() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_x_resolution(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.awt.X11GraphicsConfig.getYResolution(I)D")]
    async fn test_get_y_resolution() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_y_resolution(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.awt.X11GraphicsConfig.init(II)V")]
    async fn test_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.X11GraphicsConfig.isTranslucencyCapable(J)Z"
    )]
    async fn test_is_translucency_capable() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_translucency_capable(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.X11GraphicsConfig.makeColorModel()Ljava/awt/image/ColorModel;"
    )]
    async fn test_make_color_model() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = make_color_model(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.awt.X11GraphicsConfig.pGetBounds(I)Ljava/awt/Rectangle;"
    )]
    async fn test_p_get_bounds() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = p_get_bounds(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.awt.X11GraphicsConfig.swapBuffers(JI)V")]
    async fn test_swap_buffers() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = swap_buffers(thread, Parameters::default()).await;
    }
}
