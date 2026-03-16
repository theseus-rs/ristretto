use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("com/sun/security/auth/module/UnixSystem.getUnixInfo()V", Any)]
#[async_method]
pub async fn get_unix_info<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.security.auth.module.UnixSystem.getUnixInfo()V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_unix_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_unix_info(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
