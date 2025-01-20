use crate::native_methods::registry::{MethodRegistry, JAVA_11};
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/java2d/opengl/CGLSurfaceData";

/// Register all native methods for `sun.java2d.opengl.CGLSurfaceData`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= JAVA_11 {
        registry.register(
            CLASS_NAME,
            "createCGLContextOnSurface",
            "(Lsun/java2d/opengl/CGLSurfaceData;J)J",
            create_cgl_context_on_surface,
        );
        registry.register(CLASS_NAME, "destroyCGLContext", "(J)V", destroy_cgl_context);
        registry.register(
            CLASS_NAME,
            "makeCGLContextCurrentOnSurface",
            "(Lsun/java2d/opengl/CGLSurfaceData;J)Z",
            make_cgl_context_current_on_surface,
        );
        registry.register(CLASS_NAME, "validate", "(IIIIZ)V", validate);
    }

    registry.register(CLASS_NAME, "clearWindow", "()V", clear_window);
    registry.register(
        CLASS_NAME,
        "initOps",
        "(Lsun/java2d/opengl/OGLGraphicsConfig;JJJIIZ)V",
        init_ops,
    );
}

#[async_recursion(?Send)]
async fn clear_window(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.CGLSurfaceData.clearWindow()");
}

#[async_recursion(?Send)]
async fn create_cgl_context_on_surface(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.CGLSurfaceData.createCGLContextOnSurface(Lsun/java2d/opengl/CGLSurfaceData;J)J");
}

#[async_recursion(?Send)]
async fn destroy_cgl_context(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.CGLSurfaceData.destroyCGLContext(J)V");
}

#[async_recursion(?Send)]
async fn init_ops(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.CGLSurfaceData.initOps(Lsun/java2d/opengl/OGLGraphicsConfig;JJJIIZ)V");
}

#[async_recursion(?Send)]
async fn make_cgl_context_current_on_surface(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.CGLSurfaceData.makeCGLContextCurrentOnSurface(Lsun/java2d/opengl/CGLSurfaceData;J)Z");
}

#[async_recursion(?Send)]
async fn validate(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
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
