use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("sun/java2d/opengl/OGLContext.getOGLIdString()Ljava/lang/String;", Any)]
#[async_method]
pub(crate) async fn get_ogl_id_string(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
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
