use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("sun/lwawt/macosx/CMenuBar.nativeCreateMenuBar()J", Any)]
#[async_method]
pub(crate) async fn native_create_menu_bar(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CMenuBar.nativeCreateMenuBar()J")
}

#[intrinsic_method("sun/lwawt/macosx/CMenuBar.nativeDelMenu(JI)V", Any)]
#[async_method]
pub(crate) async fn native_del_menu(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CMenuBar.nativeDelMenu(JI)V")
}

#[intrinsic_method("sun/lwawt/macosx/CMenuBar.nativeSetHelpMenu(JJ)V", Any)]
#[async_method]
pub(crate) async fn native_set_help_menu(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CMenuBar.nativeSetHelpMenu(JJ)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CMenuBar.nativeCreateMenuBar()J"
    )]
    async fn test_native_create_menu_bar() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_create_menu_bar(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.lwawt.macosx.CMenuBar.nativeDelMenu(JI)V")]
    async fn test_native_del_menu() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_del_menu(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CMenuBar.nativeSetHelpMenu(JJ)V"
    )]
    async fn test_native_set_help_menu() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_help_menu(thread, Parameters::default()).await;
    }
}
