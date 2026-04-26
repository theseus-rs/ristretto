#[cfg(target_os = "macos")]
use ristretto_classfile::JAVA_17;
#[cfg(target_os = "windows")]
use ristretto_classfile::JAVA_25;
#[cfg(target_os = "windows")]
use ristretto_classfile::VersionSpecification::Equal;
#[cfg(target_os = "macos")]
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
#[cfg(any(target_os = "macos", target_os = "windows"))]
use ristretto_classloader::Value;
#[cfg(any(target_os = "macos", target_os = "windows"))]
use ristretto_macros::async_method;
#[cfg(any(target_os = "macos", target_os = "windows"))]
use ristretto_macros::intrinsic_method;
#[cfg(any(target_os = "macos", target_os = "windows"))]
use ristretto_types::JavaError;
#[cfg(any(target_os = "macos", target_os = "windows"))]
use ristretto_types::Thread;
#[cfg(any(target_os = "macos", target_os = "windows"))]
use ristretto_types::{Parameters, Result};
#[cfg(any(target_os = "macos", target_os = "windows"))]
use std::sync::Arc;

#[cfg(target_os = "macos")]
#[intrinsic_method(
    "sun/awt/PlatformGraphicsInfo.isInAquaSession()Z",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn is_in_aqua_session<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.awt.PlatformGraphicsInfo.isInAquaSession()Z".to_string(),
    )
    .into())
}

#[cfg(target_os = "windows")]
#[intrinsic_method("sun/awt/PlatformGraphicsInfo.hasDisplays0()Z", Equal(JAVA_25))]
#[async_method]
pub async fn has_displays0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/PlatformGraphicsInfo.hasDisplays0()Z".to_string())
            .into(),
    )
}

#[cfg(target_os = "windows")]
#[intrinsic_method("sun/awt/PlatformGraphicsInfo.hasDisplays0()Z", Equal(JAVA_25))]
#[async_method]
pub async fn has_displays0_windows_v25<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/PlatformGraphicsInfo.hasDisplays0()Z".to_string())
            .into(),
    )
}

#[cfg(all(test, any(target_os = "macos", target_os = "windows")))]
mod tests {
    use super::*;

    #[cfg(target_os = "macos")]
    #[tokio::test]
    async fn test_is_in_aqua_session() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = is_in_aqua_session(thread, Parameters::default()).await;
        assert_eq!(
            "sun.awt.PlatformGraphicsInfo.isInAquaSession()Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_has_displays0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = has_displays0(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/PlatformGraphicsInfo.hasDisplays0()Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_has_displays0_windows_v25() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = has_displays0_windows_v25(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/PlatformGraphicsInfo.hasDisplays0()Z",
            result.unwrap_err().to_string()
        );
    }
}
