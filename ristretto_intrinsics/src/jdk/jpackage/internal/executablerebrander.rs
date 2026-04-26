use ristretto_classfile::VersionSpecification::{Between, Equal, GreaterThanOrEqual};
use ristretto_classfile::{JAVA_17, JAVA_21, JAVA_25};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "jdk/jpackage/internal/ExecutableRebrander.iconSwap(JLjava/lang/String;)I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn icon_swap<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _j_icon_target = parameters.pop_reference()?;
    let _j_resource_lock = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "jdk/jpackage/internal/ExecutableRebrander.iconSwap(JLjava/lang/String;)I".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "jdk/jpackage/internal/ExecutableRebrander.lockResource(Ljava/lang/String;)J",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn lock_resource<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _j_executable = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "jdk/jpackage/internal/ExecutableRebrander.lockResource(Ljava/lang/String;)J".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "jdk/jpackage/internal/ExecutableRebrander.unlockResource(J)V",
    Between(JAVA_17, JAVA_21)
)]
#[async_method]
pub async fn unlock_resource<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _j_resource_lock = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "jdk/jpackage/internal/ExecutableRebrander.unlockResource(J)V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "jdk/jpackage/internal/ExecutableRebrander.unlockResource(J)Z",
    Equal(JAVA_25)
)]
#[async_method]
pub async fn unlock_resource_windows_v25<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _j_resource_lock = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "jdk/jpackage/internal/ExecutableRebrander.unlockResource(J)Z".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "jdk/jpackage/internal/ExecutableRebrander.versionSwap(J[Ljava/lang/String;)I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn version_swap<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _j_executable_properties = parameters.pop_reference()?;
    let _j_resource_lock = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "jdk/jpackage/internal/ExecutableRebrander.versionSwap(J[Ljava/lang/String;)I".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_icon_swap() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = icon_swap(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "jdk/jpackage/internal/ExecutableRebrander.iconSwap(JLjava/lang/String;)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_lock_resource() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = lock_resource(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "jdk/jpackage/internal/ExecutableRebrander.lockResource(Ljava/lang/String;)J",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_unlock_resource() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = unlock_resource(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "jdk/jpackage/internal/ExecutableRebrander.unlockResource(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_unlock_resource_windows_v25() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            unlock_resource_windows_v25(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "jdk/jpackage/internal/ExecutableRebrander.unlockResource(J)Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_version_swap() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = version_swap(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "jdk/jpackage/internal/ExecutableRebrander.versionSwap(J[Ljava/lang/String;)I",
            result.unwrap_err().to_string()
        );
    }
}
