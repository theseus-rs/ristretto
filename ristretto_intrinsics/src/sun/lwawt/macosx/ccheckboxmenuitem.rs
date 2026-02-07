use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/lwawt/macosx/CCheckboxMenuItem.nativeSetIsCheckbox(J)V", Any)]
#[async_method]
pub async fn native_set_is_checkbox<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CCheckboxMenuItem.nativeSetIsCheckbox(J)V")
}

#[intrinsic_method("sun/lwawt/macosx/CCheckboxMenuItem.nativeSetState(JZ)V", Any)]
#[async_method]
pub async fn native_set_state<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CCheckboxMenuItem.nativeSetState(JZ)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CCheckboxMenuItem.nativeSetIsCheckbox(J)V"
    )]
    async fn test_native_set_is_checkbox() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_is_checkbox(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CCheckboxMenuItem.nativeSetState(JZ)V"
    )]
    async fn test_native_set_state() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_state(thread, Parameters::default()).await;
    }
}
