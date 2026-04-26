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
    "jdk/internal/agent/FileSystemImpl.isAccessUserOnly0(Ljava/lang/String;)Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn is_access_user_only_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _str = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "jdk.internal.agent.FileSystemImpl.isAccessUserOnly0(Ljava/lang/String;)Z".to_string(),
    )
    .into())
}

#[cfg(target_os = "windows")]
#[intrinsic_method(
    "jdk/internal/agent/FileSystemImpl.init0()V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn init0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("jdk/internal/agent/FileSystemImpl.init0()V".to_string())
            .into(),
    )
}

#[cfg(target_os = "windows")]
#[intrinsic_method(
    "jdk/internal/agent/FileSystemImpl.isSecuritySupported0(Ljava/lang/String;)Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn is_security_supported0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _str = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "jdk/internal/agent/FileSystemImpl.isSecuritySupported0(Ljava/lang/String;)Z".to_string(),
    )
    .into())
}

#[cfg(target_os = "windows")]
#[intrinsic_method(
    "jdk/internal/agent/FileSystemImpl.init0()V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn init0_windows_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("jdk/internal/agent/FileSystemImpl.init0()V".to_string())
            .into(),
    )
}

#[cfg(target_os = "windows")]
#[intrinsic_method(
    "jdk/internal/agent/FileSystemImpl.isSecuritySupported0(Ljava/lang/String;)Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn is_security_supported0_windows_ge_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _str = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "jdk/internal/agent/FileSystemImpl.isSecuritySupported0(Ljava/lang/String;)Z".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_is_access_user_only_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            is_access_user_only_0(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "jdk.internal.agent.FileSystemImpl.isAccessUserOnly0(Ljava/lang/String;)Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_init0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init0(thread, Parameters::default()).await;
        assert_eq!(
            "jdk/internal/agent/FileSystemImpl.init0()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_is_security_supported0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            is_security_supported0(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "jdk/internal/agent/FileSystemImpl.isSecuritySupported0(Ljava/lang/String;)Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_init0_windows_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init0_windows_ge_v11(thread, Parameters::default()).await;
        assert_eq!(
            "jdk/internal/agent/FileSystemImpl.init0()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_is_security_supported0_windows_ge_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = is_security_supported0_windows_ge_v11(
            thread,
            Parameters::new(vec![Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "jdk/internal/agent/FileSystemImpl.isSecuritySupported0(Ljava/lang/String;)Z",
            result.unwrap_err().to_string()
        );
    }
}
