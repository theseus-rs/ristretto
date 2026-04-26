use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/awt/windows/WTrayIconPeer._displayMessage(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V",
    Any
)]
#[async_method]
pub async fn display_message<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg2 = parameters.pop_reference()?;
    let _arg1 = parameters.pop_reference()?;
    let _arg0 = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun/awt/windows/WTrayIconPeer._displayMessage(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V".to_string()).into())
}
#[intrinsic_method("sun/awt/windows/WTrayIconPeer._dispose()V", Any)]
#[async_method]
pub async fn dispose<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WTrayIconPeer._dispose()V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/windows/WTrayIconPeer.create()V", Any)]
#[async_method]
pub async fn create<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WTrayIconPeer.create()V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/windows/WTrayIconPeer.setNativeIcon([I[BIII)V", Any)]
#[async_method]
pub async fn set_native_icon<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _n_h = parameters.pop_int()?;
    let _n_w = parameters.pop_int()?;
    let _n_ss = parameters.pop_int()?;
    let _and_mask = parameters.pop_reference()?;
    let _int_raster_data = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WTrayIconPeer.setNativeIcon([I[BIII)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WTrayIconPeer.setToolTip(Ljava/lang/String;)V", Any)]
#[async_method]
pub async fn set_tool_tip<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _tooltip = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WTrayIconPeer.setToolTip(Ljava/lang/String;)V".to_string(),
    )
    .into())
}
#[intrinsic_method("sun/awt/windows/WTrayIconPeer.updateNativeIcon(Z)V", Any)]
#[async_method]
pub async fn update_native_icon<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _do_update = parameters.pop_bool()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WTrayIconPeer.updateNativeIcon(Z)V".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_display_message() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = display_message(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Object(None),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/WTrayIconPeer._displayMessage(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_dispose() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = dispose(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WTrayIconPeer._dispose()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_create() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WTrayIconPeer.create()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_native_icon() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_native_icon(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/WTrayIconPeer.setNativeIcon([I[BIII)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_set_tool_tip() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_tool_tip(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/awt/windows/WTrayIconPeer.setToolTip(Ljava/lang/String;)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_update_native_icon() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = update_native_icon(thread, Parameters::new(vec![Value::from(false)])).await;
        assert_eq!(
            "sun/awt/windows/WTrayIconPeer.updateNativeIcon(Z)V",
            result.unwrap_err().to_string()
        );
    }
}
