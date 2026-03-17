use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/lwawt/macosx/CMenuBar.nativeCreateMenuBar()J", Any)]
#[async_method]
pub async fn native_create_menu_bar<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CMenuBar.nativeCreateMenuBar()J".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CMenuBar.nativeDelMenu(JI)V", Any)]
#[async_method]
pub async fn native_del_menu<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun.lwawt.macosx.CMenuBar.nativeDelMenu(JI)V".to_string())
            .into(),
    )
}

#[intrinsic_method("sun/lwawt/macosx/CMenuBar.nativeSetHelpMenu(JJ)V", Any)]
#[async_method]
pub async fn native_set_help_menu<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CMenuBar.nativeSetHelpMenu(JJ)V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_native_create_menu_bar() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_create_menu_bar(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_del_menu() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_del_menu(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_native_set_help_menu() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_set_help_menu(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
