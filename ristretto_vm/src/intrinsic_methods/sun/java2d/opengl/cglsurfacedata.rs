use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::{Any, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("sun/java2d/opengl/CGLSurfaceData.clearWindow()V", Any)]
#[async_method]
pub(crate) async fn clear_window(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.CGLSurfaceData.clearWindow()");
}

#[intrinsic_method(
    "sun/java2d/opengl/CGLSurfaceData.createCGLContextOnSurface(Lsun/java2d/opengl/CGLSurfaceData;J)J",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn create_cgl_context_on_surface(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.java2d.opengl.CGLSurfaceData.createCGLContextOnSurface(Lsun/java2d/opengl/CGLSurfaceData;J)J"
    );
}

#[intrinsic_method(
    "sun/java2d/opengl/CGLSurfaceData.destroyCGLContext(J)V",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn destroy_cgl_context(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.CGLSurfaceData.destroyCGLContext(J)V");
}

#[intrinsic_method(
    "sun/java2d/opengl/CGLSurfaceData.initOps(Lsun/java2d/opengl/OGLGraphicsConfig;JJJIIZ)V",
    Any
)]
#[async_method]
pub(crate) async fn init_ops(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.CGLSurfaceData.initOps(Lsun/java2d/opengl/OGLGraphicsConfig;JJJIIZ)V");
}

#[intrinsic_method(
    "sun/java2d/opengl/CGLSurfaceData.makeCGLContextCurrentOnSurface(Lsun/java2d/opengl/CGLSurfaceData;J)Z",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn make_cgl_context_current_on_surface(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.java2d.opengl.CGLSurfaceData.makeCGLContextCurrentOnSurface(Lsun/java2d/opengl/CGLSurfaceData;J)Z"
    );
}

#[intrinsic_method(
    "sun/java2d/opengl/CGLSurfaceData.validate(IIIIZ)V",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn validate(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.CGLSurfaceData.validate(IIIIZ)V");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.opengl.CGLSurfaceData.clearWindow()"
    )]
    async fn test_clear_window() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = clear_window(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.opengl.CGLSurfaceData.createCGLContextOnSurface(Lsun/java2d/opengl/CGLSurfaceData;J)J"
    )]
    async fn test_create_cgl_context_on_surface() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create_cgl_context_on_surface(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.opengl.CGLSurfaceData.destroyCGLContext(J)V"
    )]
    async fn test_destroy_cgl_context() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = destroy_cgl_context(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.opengl.CGLSurfaceData.initOps(Lsun/java2d/opengl/OGLGraphicsConfig;JJJIIZ)V"
    )]
    async fn test_init_ops() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_ops(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.opengl.CGLSurfaceData.makeCGLContextCurrentOnSurface(Lsun/java2d/opengl/CGLSurfaceData;J)Z"
    )]
    async fn test_make_cgl_context_current_on_surface() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = make_cgl_context_current_on_surface(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.opengl.CGLSurfaceData.validate(IIIIZ)V"
    )]
    async fn test_validate() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = validate(thread, Parameters::default()).await;
    }
}
