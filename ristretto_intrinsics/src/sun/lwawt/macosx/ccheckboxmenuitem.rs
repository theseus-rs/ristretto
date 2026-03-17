use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/lwawt/macosx/CCheckboxMenuItem.nativeSetIsCheckbox(J)V", Any)]
#[async_method]
pub async fn native_set_is_checkbox<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CCheckboxMenuItem.nativeSetIsCheckbox(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CCheckboxMenuItem.nativeSetState(JZ)V", Any)]
#[async_method]
pub async fn native_set_state<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CCheckboxMenuItem.nativeSetState(JZ)V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_native_set_is_checkbox() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_set_is_checkbox(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_set_state() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_set_state(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
