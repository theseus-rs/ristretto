use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/lwawt/macosx/CWrapper$NSView.addSubview(JJ)V", Any)]
#[async_method]
pub async fn add_subview<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CWrapper$NSView.addSubview(JJ)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CWrapper$NSView.removeFromSuperview(J)V", Any)]
#[async_method]
pub async fn remove_from_superview<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CWrapper$NSView.removeFromSuperview(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CWrapper$NSView.setFrame(JIIII)V", Any)]
#[async_method]
pub async fn set_frame<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CWrapper$NSView.setFrame(JIIII)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CWrapper$NSView.setHidden(JZ)V", Any)]
#[async_method]
pub async fn set_hidden<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CWrapper$NSView.setHidden(JZ)V".to_string(),
    )
    .into())
}

#[intrinsic_method(
    "sun/lwawt/macosx/CWrapper$NSView.setToolTip(JLjava/lang/String;)V",
    Any
)]
#[async_method]
pub async fn set_tool_tip<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CWrapper$NSView.setToolTip(JLjava/lang/String;)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CWrapper$NSView.window(J)J", Any)]
#[async_method]
pub async fn window<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun.lwawt.macosx.CWrapper$NSView.window(J)J".to_string())
            .into(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_subview() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = add_subview(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_remove_from_superview() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = remove_from_superview(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_frame() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_frame(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_hidden() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_hidden(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_tool_tip() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_tool_tip(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_window() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = window(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
