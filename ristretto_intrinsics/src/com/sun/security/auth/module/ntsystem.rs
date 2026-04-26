#[cfg(target_os = "windows")]
use ristretto_classfile::JAVA_8;
use ristretto_classfile::JAVA_11;
#[cfg(target_os = "windows")]
use ristretto_classfile::VersionSpecification::Equal;
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
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _debug = parameters.pop_bool()?;
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
#[cfg(target_os = "windows")]
#[intrinsic_method("com/sun/security/auth/module/NTSystem.getCurrent(Z)V", Equal(JAVA_8))]
#[async_method]
pub async fn get_current_windows_v8<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _debug = parameters.pop_bool()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com/sun/security/auth/module/NTSystem.getCurrent(Z)V".to_string(),
    )
    .into())
}
#[cfg(target_os = "windows")]
#[intrinsic_method(
    "com/sun/security/auth/module/NTSystem.getImpersonationToken0()J",
    Equal(JAVA_8)
)]
#[async_method]
pub async fn get_impersonation_token0_windows_v8<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "com/sun/security/auth/module/NTSystem.getImpersonationToken0()J".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_current() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_current(thread, Parameters::new(vec![Value::from(false)])).await;
        assert_eq!(
            "com.sun.security.auth.module.NTSystem.getCurrent(Z)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_impersonation_token_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_impersonation_token_0(thread, Parameters::default()).await;
        assert_eq!(
            "com.sun.security.auth.module.NTSystem.getImpersonationToken0()J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_current_windows_v8() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            get_current_windows_v8(thread, Parameters::new(vec![Value::from(false)])).await;
        assert_eq!(
            "com/sun/security/auth/module/NTSystem.getCurrent(Z)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_impersonation_token0_windows_v8() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_impersonation_token0_windows_v8(thread, Parameters::default()).await;
        assert_eq!(
            "com/sun/security/auth/module/NTSystem.getImpersonationToken0()J",
            result.unwrap_err().to_string()
        );
    }
}
