use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/awt/X11/XInputMethod.adjustStatusWindow(J)V", Any)]
#[async_method]
pub async fn adjust_status_window<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _window = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XInputMethod.adjustStatusWindow(J)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/X11/XInputMethod.createXICNative(J)Z", Any)]
#[async_method]
pub async fn create_xicnative<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _window = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XInputMethod.createXICNative(J)Z".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XInputMethod.openXIMNative(J)Z", Any)]
#[async_method]
pub async fn open_ximnative<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _display = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/X11/XInputMethod.openXIMNative(J)Z".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/X11/XInputMethod.setXICFocusNative(JZZ)V", Any)]
#[async_method]
pub async fn set_xicfocus_native<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _active = parameters.pop_bool()?;
    let _value = parameters.pop_bool()?;
    let _window = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/X11/XInputMethod.setXICFocusNative(JZZ)V".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_adjust_status_window() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = adjust_status_window(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/X11/XInputMethod.adjustStatusWindow(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_create_xicnative() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create_xicnative(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/X11/XInputMethod.createXICNative(J)Z",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_open_ximnative() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = open_ximnative(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/X11/XInputMethod.openXIMNative(J)Z",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_set_xicfocus_native() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_xicfocus_native(
            thread,
            Parameters::new(vec![Value::Long(0), Value::from(false), Value::from(false)]),
        )
        .await;
        assert_eq!(
            "sun/awt/X11/XInputMethod.setXICFocusNative(JZZ)V",
            result.unwrap_err().to_string()
        );
    }
}
