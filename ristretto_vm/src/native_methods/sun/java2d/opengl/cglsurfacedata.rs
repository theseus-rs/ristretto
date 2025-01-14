use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_11: Version = Version::Java11 { minor: 0 };

/// Register all native methods for `sun.java2d.opengl.CGLSurfaceData`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/java2d/opengl/CGLSurfaceData";
    let java_version = registry.java_version();

    if java_version <= &JAVA_11 {
        registry.register(
            class_name,
            "createCGLContextOnSurface",
            "(Lsun/java2d/opengl/CGLSurfaceData;J)J",
            create_cgl_context_on_surface,
        );
        registry.register(class_name, "destroyCGLContext", "(J)V", destroy_cgl_context);
        registry.register(
            class_name,
            "makeCGLContextCurrentOnSurface",
            "(Lsun/java2d/opengl/CGLSurfaceData;J)Z",
            make_cgl_context_current_on_surface,
        );
        registry.register(class_name, "validate", "(IIIIZ)V", validate);
    }

    registry.register(class_name, "clearWindow", "()V", clear_window);
    registry.register(
        class_name,
        "initOps",
        "(Lsun/java2d/opengl/OGLGraphicsConfig;JJJIIZ)V",
        init_ops,
    );
}

#[async_recursion(?Send)]
async fn clear_window(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.CGLSurfaceData.clearWindow()");
}

#[async_recursion(?Send)]
async fn create_cgl_context_on_surface(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.CGLSurfaceData.createCGLContextOnSurface(Lsun/java2d/opengl/CGLSurfaceData;J)J");
}

#[async_recursion(?Send)]
async fn destroy_cgl_context(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.CGLSurfaceData.destroyCGLContext(J)V");
}

#[async_recursion(?Send)]
async fn init_ops(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.CGLSurfaceData.initOps(Lsun/java2d/opengl/OGLGraphicsConfig;JJJIIZ)V");
}

#[async_recursion(?Send)]
async fn make_cgl_context_current_on_surface(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.CGLSurfaceData.makeCGLContextCurrentOnSurface(Lsun/java2d/opengl/CGLSurfaceData;J)Z");
}

#[async_recursion(?Send)]
async fn validate(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.CGLSurfaceData.validate(IIIIZ)V");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/java2d/opengl/CGLSurfaceData";
        assert!(registry.method(class_name, "clearWindow", "()V").is_some());
        assert!(registry
            .method(
                class_name,
                "createCGLContextOnSurface",
                "(Lsun/java2d/opengl/CGLSurfaceData;J)J"
            )
            .is_some());
        assert!(registry
            .method(class_name, "destroyCGLContext", "(J)V")
            .is_some());
        assert!(registry
            .method(
                class_name,
                "makeCGLContextCurrentOnSurface",
                "(Lsun/java2d/opengl/CGLSurfaceData;J)Z"
            )
            .is_some());
        assert!(registry
            .method(class_name, "validate", "(IIIIZ)V")
            .is_some());
        assert!(registry
            .method(
                class_name,
                "initOps",
                "(Lsun/java2d/opengl/OGLGraphicsConfig;JJJIIZ)V"
            )
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.opengl.CGLSurfaceData.clearWindow()")]
    async fn test_clear_window() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = clear_window(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.java2d.opengl.CGLSurfaceData.createCGLContextOnSurface(Lsun/java2d/opengl/CGLSurfaceData;J)J"
    )]
    async fn test_create_cgl_context_on_surface() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create_cgl_context_on_surface(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.opengl.CGLSurfaceData.destroyCGLContext(J)V")]
    async fn test_destroy_cgl_context() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = destroy_cgl_context(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.java2d.opengl.CGLSurfaceData.initOps(Lsun/java2d/opengl/OGLGraphicsConfig;JJJIIZ)V"
    )]
    async fn test_init_ops() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_ops(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.java2d.opengl.CGLSurfaceData.makeCGLContextCurrentOnSurface(Lsun/java2d/opengl/CGLSurfaceData;J)Z"
    )]
    async fn test_make_cgl_context_current_on_surface() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = make_cgl_context_current_on_surface(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.opengl.CGLSurfaceData.validate(IIIIZ)V")]
    async fn test_validate() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = validate(thread, Arguments::default()).await;
    }
}
