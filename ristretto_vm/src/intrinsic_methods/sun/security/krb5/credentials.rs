use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/security/krb5/Credentials.acquireDefaultNativeCreds([I)Lsun/security/krb5/Credentials;",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn acquire_default_native_creds(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.security.krb5.Credentials.acquireDefaultNativeCreds([I)Lsun/security/krb5/Credentials;"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.security.krb5.Credentials.acquireDefaultNativeCreds([I)Lsun/security/krb5/Credentials;"
    )]
    async fn test_acquire_default_native_creds() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = acquire_default_native_creds(thread, Parameters::default()).await;
    }
}
