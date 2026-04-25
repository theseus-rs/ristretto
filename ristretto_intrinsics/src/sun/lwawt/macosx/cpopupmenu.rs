use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/lwawt/macosx/CPopupMenu.nativeCreatePopupMenu()J", Any)]
#[async_method]
pub async fn native_create_popup_menu<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CPopupMenu.nativeCreatePopupMenu()J".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CPopupMenu.nativeShowPopupMenu(JII)J", Any)]
#[async_method]
pub async fn native_show_popup_menu<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _y = parameters.pop_int()?;
    let _x = parameters.pop_int()?;
    let _model_ptr = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CPopupMenu.nativeShowPopupMenu(JII)J".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_native_create_popup_menu() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_create_popup_menu(thread, Parameters::default()).await;
        assert_eq!(
            "sun.lwawt.macosx.CPopupMenu.nativeCreatePopupMenu()J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_show_popup_menu() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_show_popup_menu(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Int(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun.lwawt.macosx.CPopupMenu.nativeShowPopupMenu(JII)J",
            result.unwrap_err().to_string()
        );
    }
}
