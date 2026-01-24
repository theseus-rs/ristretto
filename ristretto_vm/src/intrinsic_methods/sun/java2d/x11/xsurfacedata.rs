use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("sun/java2d/x11/XSurfaceData.XCreateGC(J)J", LessThanOrEqual(JAVA_8))]
#[async_method]
pub(crate) async fn x_create_gc(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.XSurfaceData.XCreateGC(J)J");
}

#[intrinsic_method("sun/java2d/x11/XSurfaceData.XResetClip(J)V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub(crate) async fn x_reset_clip(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.XSurfaceData.XResetClip(J)V");
}

#[intrinsic_method(
    "sun/java2d/x11/XSurfaceData.XSetClip(JIIIILsun/java2d/pipe/Region;)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn x_set_clip(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.XSurfaceData.XSetClip(JIIIILsun/java2d/pipe/Region;)V");
}

#[intrinsic_method(
    "sun/java2d/x11/XSurfaceData.XSetGraphicsExposures(JZ)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn x_set_graphics_exposures(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.XSurfaceData.XSetGraphicsExposures(JZ)V");
}

#[intrinsic_method(
    "sun/java2d/x11/XSurfaceData.flushNativeSurface()V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn flush_native_surface(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.XSurfaceData.flushNativeSurface()V");
}

#[intrinsic_method(
    "sun/java2d/x11/XSurfaceData.initOps(Lsun/awt/X11ComponentPeer;Lsun/awt/X11GraphicsConfig;I)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn init_ops(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.java2d.x11.XSurfaceData.initOps(Lsun/awt/X11ComponentPeer;Lsun/awt/X11GraphicsConfig;I)V"
    );
}

#[intrinsic_method(
    "sun/java2d/x11/XSurfaceData.isDrawableValid()Z",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub(crate) async fn is_drawable_valid(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.XSurfaceData.isDrawableValid()Z");
}

#[intrinsic_method("sun/java2d/x11/XSurfaceData.setInvalid()V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub(crate) async fn set_invalid(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.x11.XSurfaceData.setInvalid()V");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.java2d.x11.XSurfaceData.XCreateGC(J)J")]
    async fn test_x_create_gc() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = x_create_gc(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.java2d.x11.XSurfaceData.XResetClip(J)V")]
    async fn test_x_reset_clip() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = x_reset_clip(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.x11.XSurfaceData.XSetClip(JIIIILsun/java2d/pipe/Region;)V"
    )]
    async fn test_x_set_clip() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = x_set_clip(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.x11.XSurfaceData.XSetGraphicsExposures(JZ)V"
    )]
    async fn test_x_set_graphics_exposures() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = x_set_graphics_exposures(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.x11.XSurfaceData.flushNativeSurface()V"
    )]
    async fn test_flush_native_surface() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = flush_native_surface(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.x11.XSurfaceData.initOps(Lsun/awt/X11ComponentPeer;Lsun/awt/X11GraphicsConfig;I)V"
    )]
    async fn test_init_ops() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_ops(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.x11.XSurfaceData.isDrawableValid()Z"
    )]
    async fn test_is_drawable_valid() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_drawable_valid(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.java2d.x11.XSurfaceData.setInvalid()V")]
    async fn test_set_invalid() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_invalid(thread, Parameters::default()).await;
    }
}
