use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "com/sun/security/auth/module/NTSystem.getCurrent(Z)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn get_current(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.security.auth.module.NTSystem.getCurrent(Z)V")
}

#[intrinsic_method(
    "com/sun/security/auth/module/NTSystem.getImpersonationToken0()J",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub(crate) async fn get_impersonation_token_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.sun.security.auth.module.NTSystem.getImpersonationToken0()J")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.security.auth.module.NTSystem.getCurrent(Z)V"
    )]
    async fn test_get_current() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_current(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.sun.security.auth.module.NTSystem.getImpersonationToken0()J"
    )]
    async fn test_get_impersonation_token_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_impersonation_token_0(thread, Parameters::default()).await;
    }
}
