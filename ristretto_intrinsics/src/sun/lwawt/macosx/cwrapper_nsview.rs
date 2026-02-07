use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/lwawt/macosx/CWrapper$NSView.addSubview(JJ)V", Any)]
#[async_method]
pub async fn add_subview<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSView.addSubview(JJ)V")
}

#[intrinsic_method("sun/lwawt/macosx/CWrapper$NSView.removeFromSuperview(J)V", Any)]
#[async_method]
pub async fn remove_from_superview<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSView.removeFromSuperview(J)V")
}

#[intrinsic_method("sun/lwawt/macosx/CWrapper$NSView.setFrame(JIIII)V", Any)]
#[async_method]
pub async fn set_frame<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSView.setFrame(JIIII)V")
}

#[intrinsic_method("sun/lwawt/macosx/CWrapper$NSView.setHidden(JZ)V", Any)]
#[async_method]
pub async fn set_hidden<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSView.setHidden(JZ)V")
}

#[intrinsic_method(
    "sun/lwawt/macosx/CWrapper$NSView.setToolTip(JLjava/lang/String;)V",
    Any
)]
#[async_method]
pub async fn set_tool_tip<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSView.setToolTip(JLjava/lang/String;)V")
}

#[intrinsic_method("sun/lwawt/macosx/CWrapper$NSView.window(J)J", Any)]
#[async_method]
pub async fn window<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CWrapper$NSView.window(J)J")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CWrapper$NSView.addSubview(JJ)V"
    )]
    async fn test_add_subview() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = add_subview(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CWrapper$NSView.removeFromSuperview(J)V"
    )]
    async fn test_remove_from_superview() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = remove_from_superview(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CWrapper$NSView.setFrame(JIIII)V"
    )]
    async fn test_set_frame() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_frame(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CWrapper$NSView.setHidden(JZ)V"
    )]
    async fn test_set_hidden() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_hidden(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CWrapper$NSView.setToolTip(JLjava/lang/String;)V"
    )]
    async fn test_set_tool_tip() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_tool_tip(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.lwawt.macosx.CWrapper$NSView.window(J)J")]
    async fn test_window() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = window(thread, Parameters::default()).await;
    }
}
