use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::Equal;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/nio/fs/MagicFileTypeDetector.initialize0()Z", Equal(JAVA_8))]
#[async_method]
pub async fn initialize0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/nio/fs/MagicFileTypeDetector.initialize0()Z".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/nio/fs/MagicFileTypeDetector.probe0(J)[B", Equal(JAVA_8))]
#[async_method]
pub async fn probe0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _path_address = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/nio/fs/MagicFileTypeDetector.probe0(J)[B".to_string())
            .into(),
    )
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_initialize0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = initialize0(thread, Parameters::default()).await;
        assert_eq!(
            "sun/nio/fs/MagicFileTypeDetector.initialize0()Z",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_probe0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = probe0(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/nio/fs/MagicFileTypeDetector.probe0(J)[B",
            result.unwrap_err().to_string()
        );
    }
}
