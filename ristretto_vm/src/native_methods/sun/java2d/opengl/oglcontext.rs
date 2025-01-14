use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.java2d.opengl.OGLContext`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/java2d/opengl/OGLContext";
    registry.register(
        class_name,
        "getOGLIdString",
        "()Ljava/lang/String;",
        get_ogl_id_string,
    );
}

#[async_recursion(?Send)]
async fn get_ogl_id_string(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.OGLContext.getOGLIdString()Ljava/lang/String;");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/java2d/opengl/OGLContext";
        assert!(registry
            .method(class_name, "getOGLIdString", "()Ljava/lang/String;")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.java2d.opengl.OGLContext.getOGLIdString()Ljava/lang/String;")]
    async fn test_get_ogl_id_string() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_ogl_id_string(thread, Arguments::default()).await;
    }
}
