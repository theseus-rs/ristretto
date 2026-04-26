use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/java2d/opengl/OGLContext.getOGLIdString()Ljava/lang/String;", Any)]
#[async_method]
pub async fn get_ogl_id_string<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.java2d.opengl.OGLContext.getOGLIdString()Ljava/lang/String;".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_ogl_id_string() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_ogl_id_string(thread, Parameters::default()).await;
        assert_eq!(
            "sun.java2d.opengl.OGLContext.getOGLIdString()Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }
}
