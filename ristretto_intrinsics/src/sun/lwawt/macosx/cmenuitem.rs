use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/lwawt/macosx/CMenuItem.nativeCreate(JZ)J", Any)]
#[async_method]
pub async fn native_create<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun.lwawt.macosx.CMenuItem.nativeCreate(JZ)J".to_string())
            .into(),
    )
}

#[intrinsic_method("sun/lwawt/macosx/CMenuItem.nativeSetEnabled(JZ)V", Any)]
#[async_method]
pub async fn native_set_enabled<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CMenuItem.nativeSetEnabled(JZ)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CMenuItem.nativeSetImage(JJ)V", Any)]
#[async_method]
pub async fn native_set_image<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CMenuItem.nativeSetImage(JJ)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/lwawt/macosx/CMenuItem.nativeSetLabel(JLjava/lang/String;CII)V",
    Any
)]
#[async_method]
pub async fn native_set_label<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CMenuItem.nativeSetLabel(JLjava/lang/String;CII)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/lwawt/macosx/CMenuItem.nativeSetTooltip(JLjava/lang/String;)V",
    Any
)]
#[async_method]
pub async fn native_set_tooltip<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CMenuItem.nativeSetTooltip(JLjava/lang/String;)V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_native_create() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_create(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_set_enabled() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_set_enabled(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_set_image() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_set_image(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_set_label() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_set_label(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_set_tooltip() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_set_tooltip(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
