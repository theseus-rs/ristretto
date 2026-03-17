use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "com/sun/security/auth/module/NTSystem.getCurrent(Z)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_current<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.security.auth.module.NTSystem.getCurrent(Z)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "com/sun/security/auth/module/NTSystem.getImpersonationToken0()J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_impersonation_token_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com.sun.security.auth.module.NTSystem.getImpersonationToken0()J".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_current() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_current(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_impersonation_token_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_impersonation_token_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
