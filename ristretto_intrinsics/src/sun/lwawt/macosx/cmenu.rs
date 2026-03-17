use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::{Any, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/lwawt/macosx/CMenu.nativeAddSeparator(J)V",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn native_add_separator<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CMenu.nativeAddSeparator(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CMenu.nativeCreateMenu(JZI)J", Any)]
#[async_method]
pub async fn native_create_menu<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CMenu.nativeCreateMenu(JZI)J".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CMenu.nativeCreateSubMenu(J)J", Any)]
#[async_method]
pub async fn native_create_sub_menu<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CMenu.nativeCreateSubMenu(J)J".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CMenu.nativeDeleteItem(JI)V", Any)]
#[async_method]
pub async fn native_delete_item<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun.lwawt.macosx.CMenu.nativeDeleteItem(JI)V".to_string())
            .into(),
    )
}

#[intrinsic_method("sun/lwawt/macosx/CMenu.nativeGetNSMenu(J)J", Any)]
#[async_method]
pub async fn native_get_ns_menu<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun.lwawt.macosx.CMenu.nativeGetNSMenu(J)J".to_string())
            .into(),
    )
}

#[intrinsic_method("sun/lwawt/macosx/CMenu.nativeSetMenuTitle(JLjava/lang/String;)V", Any)]
#[async_method]
pub async fn native_set_menu_title<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CMenu.nativeSetMenuTitle(JLjava/lang/String;)V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_native_add_separator() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_add_separator(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_create_menu() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_create_menu(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_create_sub_menu() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_create_sub_menu(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_delete_item() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_delete_item(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_get_ns_menu() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_get_ns_menu(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_set_menu_title() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_set_menu_title(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
