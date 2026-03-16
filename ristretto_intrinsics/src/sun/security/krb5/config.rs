use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/security/krb5/Config.getWindowsDirectory(Z)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_windows_directory<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.security.krb5.Config.getWindowsDirectory(Z)Ljava/lang/String;".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_windows_directory() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_windows_directory(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
