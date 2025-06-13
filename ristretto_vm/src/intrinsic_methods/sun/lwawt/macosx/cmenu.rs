use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::{Any, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/lwawt/macosx/CMenu.nativeAddSeparator(J)V",
    LessThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn native_add_separator(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CMenu.nativeAddSeparator(J)V")
}

#[intrinsic_method("sun/lwawt/macosx/CMenu.nativeCreateMenu(JZI)J", Any)]
#[async_recursion(?Send)]
pub(crate) async fn native_create_menu(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CMenu.nativeCreateMenu(JZI)J")
}

#[intrinsic_method("sun/lwawt/macosx/CMenu.nativeCreateSubMenu(J)J", Any)]
#[async_recursion(?Send)]
pub(crate) async fn native_create_sub_menu(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CMenu.nativeCreateSubMenu(J)J")
}

#[intrinsic_method("sun/lwawt/macosx/CMenu.nativeDeleteItem(JI)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn native_delete_item(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CMenu.nativeDeleteItem(JI)V")
}

#[intrinsic_method("sun/lwawt/macosx/CMenu.nativeGetNSMenu(J)J", Any)]
#[async_recursion(?Send)]
pub(crate) async fn native_get_ns_menu(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CMenu.nativeGetNSMenu(J)J")
}

#[intrinsic_method("sun/lwawt/macosx/CMenu.nativeSetMenuTitle(JLjava/lang/String;)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn native_set_menu_title(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CMenu.nativeSetMenuTitle(JLjava/lang/String;)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.lwawt.macosx.CMenu.nativeAddSeparator(J)V")]
    async fn test_native_add_separator() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_add_separator(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.lwawt.macosx.CMenu.nativeCreateMenu(JZI)J")]
    async fn test_native_create_menu() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_create_menu(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CMenu.nativeCreateSubMenu(J)J"
    )]
    async fn test_native_create_sub_menu() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_create_sub_menu(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.lwawt.macosx.CMenu.nativeDeleteItem(JI)V")]
    async fn test_native_delete_item() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_delete_item(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.lwawt.macosx.CMenu.nativeGetNSMenu(J)J")]
    async fn test_native_get_ns_menu() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_get_ns_menu(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CMenu.nativeSetMenuTitle(JLjava/lang/String;)V"
    )]
    async fn test_native_set_menu_title() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = native_set_menu_title(thread, Parameters::default()).await;
    }
}
