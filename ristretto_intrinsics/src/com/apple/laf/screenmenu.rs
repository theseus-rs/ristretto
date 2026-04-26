use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "com/apple/laf/ScreenMenu.addMenuListeners(Lcom/apple/laf/ScreenMenu;J)J",
    Any
)]
#[async_method]
pub async fn add_menu_listeners<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _native_menu = parameters.pop_long()?;
    let _listener = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.apple.laf.ScreenMenu.addMenuListeners(Lcom/apple/laf/ScreenMenu;J)J".to_string(),
    )
    .into())
}

#[intrinsic_method("com/apple/laf/ScreenMenu.removeMenuListeners(J)V", Any)]
#[async_method]
pub async fn remove_menu_listeners<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _model_ptr = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.apple.laf.ScreenMenu.removeMenuListeners(J)V".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_menu_listeners() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = add_menu_listeners(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "com.apple.laf.ScreenMenu.addMenuListeners(Lcom/apple/laf/ScreenMenu;J)J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_remove_menu_listeners() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = remove_menu_listeners(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "com.apple.laf.ScreenMenu.removeMenuListeners(J)V",
            result.unwrap_err().to_string()
        );
    }
}
