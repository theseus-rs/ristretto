use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/management/FileSystemImpl.isAccessUserOnly0(Ljava/lang/String;)Z",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn is_access_user_only_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.management.FileSystemImpl.isAccessUserOnly0(Ljava/lang/String;)Z".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_is_access_user_only_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = is_access_user_only_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
