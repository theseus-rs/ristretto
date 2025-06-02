use crate::Result;
use crate::intrinsic_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/java2d/opengl/OGLContext";

/// Register all intrinsic methods for `sun.java2d.opengl.OGLContext`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "getOGLIdString",
        "()Ljava/lang/String;",
        get_ogl_id_string,
    );
}

#[async_recursion(?Send)]
async fn get_ogl_id_string(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.java2d.opengl.OGLContext.getOGLIdString()Ljava/lang/String;");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.java2d.opengl.OGLContext.getOGLIdString()Ljava/lang/String;"
    )]
    async fn test_get_ogl_id_string() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_ogl_id_string(thread, Parameters::default()).await;
    }
}
