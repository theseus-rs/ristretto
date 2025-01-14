use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.java2d.opengl.GLXSurfaceData`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/java2d/opengl/GLXSurfaceData";
    registry.register(
        class_name,
        "initOps",
        "(Lsun/java2d/opengl/OGLGraphicsConfig;Lsun/awt/X11ComponentPeer;J)V",
        init_ops,
    );
}

#[async_recursion(?Send)]
async fn init_ops(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.GLXSurfaceData.initOps(Lsun/java2d/opengl/OGLGraphicsConfig;Lsun/awt/X11ComponentPeer;J)V");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/java2d/opengl/GLXSurfaceData";
        assert!(registry
            .method(
                class_name,
                "initOps",
                "(Lsun/java2d/opengl/OGLGraphicsConfig;Lsun/awt/X11ComponentPeer;J)V"
            )
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.java2d.opengl.GLXSurfaceData.initOps(Lsun/java2d/opengl/OGLGraphicsConfig;Lsun/awt/X11ComponentPeer;J)V"
    )]
    async fn test_init_ops() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init_ops(thread, Arguments::default()).await;
    }
}
