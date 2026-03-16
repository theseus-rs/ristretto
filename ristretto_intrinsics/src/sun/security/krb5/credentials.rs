use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/security/krb5/Credentials.acquireDefaultNativeCreds([I)Lsun/security/krb5/Credentials;",
    Any
)]
#[async_method]
pub async fn acquire_default_native_creds<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.security.krb5.Credentials.acquireDefaultNativeCreds([I)Lsun/security/krb5/Credentials;".to_string()).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_acquire_default_native_creds() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = acquire_default_native_creds(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
