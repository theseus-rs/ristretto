use ristretto_classfile::JAVA_17;
use ristretto_classfile::VersionSpecification::{Any, GreaterThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "com/apple/eawt/_AppMenuBarHandler.nativeActivateDefaultMenuBar(J)V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn native_activate_default_menu_bar<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _menu_bar_peer = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.apple.eawt._AppMenuBarHandler.nativeActivateDefaultMenuBar(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method("com/apple/eawt/_AppMenuBarHandler.nativeSetDefaultMenuBar(J)V", Any)]
#[async_method]
pub async fn native_set_default_menu_bar<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _menu_bar_peer = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.apple.eawt._AppMenuBarHandler.nativeSetDefaultMenuBar(J)V".to_string(),
    )
    .into())
}

#[intrinsic_method("com/apple/eawt/_AppMenuBarHandler.nativeSetMenuState(IZZ)V", Any)]
#[async_method]
pub async fn native_set_menu_state<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _enabled = parameters.pop_bool()?;
    let _visible = parameters.pop_bool()?;
    let _menu = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "com.apple.eawt._AppMenuBarHandler.nativeSetMenuState(IZZ)V".to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_native_activate_default_menu_bar() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            native_activate_default_menu_bar(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "com.apple.eawt._AppMenuBarHandler.nativeActivateDefaultMenuBar(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_set_default_menu_bar() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            native_set_default_menu_bar(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "com.apple.eawt._AppMenuBarHandler.nativeSetDefaultMenuBar(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_native_set_menu_state() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = native_set_menu_state(
            thread,
            Parameters::new(vec![Value::Int(0), Value::from(false), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "com.apple.eawt._AppMenuBarHandler.nativeSetMenuState(IZZ)V",
            result.unwrap_err().to_string()
        );
    }
}
