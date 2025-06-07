use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_17;
use ristretto_classfile::VersionSpecification::{Any, GreaterThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "com/apple/eawt/_AppMenuBarHandler.nativeActivateDefaultMenuBar(J)V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn native_activate_default_menu_bar(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.apple.eawt._AppMenuBarHandler.nativeActivateDefaultMenuBar(J)V")
}

#[intrinsic_method("com/apple/eawt/_AppMenuBarHandler.nativeSetDefaultMenuBar(J)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn native_set_default_menu_bar(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.apple.eawt._AppMenuBarHandler.nativeSetDefaultMenuBar(J)V")
}

#[intrinsic_method("com/apple/eawt/_AppMenuBarHandler.nativeSetMenuState(IZZ)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn native_set_menu_state(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("com.apple.eawt._AppMenuBarHandler.nativeSetMenuState(IZZ)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.eawt._AppMenuBarHandler.nativeActivateDefaultMenuBar(J)V"
    )]
    async fn test_native_activate_default_menu_bar() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_activate_default_menu_bar(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.eawt._AppMenuBarHandler.nativeSetDefaultMenuBar(J)V"
    )]
    async fn test_native_set_default_menu_bar() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_default_menu_bar(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: com.apple.eawt._AppMenuBarHandler.nativeSetMenuState(IZZ)V"
    )]
    async fn test_native_set_menu_state() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_menu_state(thread, Parameters::default()).await;
    }
}
