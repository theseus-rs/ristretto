use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/awt/windows/WDropTargetContextPeerFileStream.freeStgMedium(J)V",
    Any
)]
#[async_method]
pub async fn free_stg_medium<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _stgmedium = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WDropTargetContextPeerFileStream.freeStgMedium(J)V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_free_stg_medium() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = free_stg_medium(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/windows/WDropTargetContextPeerFileStream.freeStgMedium(J)V",
            result.unwrap_err().to_string()
        );
    }
}
