use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/awt/windows/WEmbeddedFrame.initIDs()V", Any)]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/awt/windows/WEmbeddedFrame.initIDs()V".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/awt/windows/WEmbeddedFrame.isPrinterDC(J)Z", Any)]
#[async_method]
pub async fn is_printer_dc<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _hdc = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WEmbeddedFrame.isPrinterDC(J)Z".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/awt/windows/WEmbeddedFrame.notifyModalBlockedImpl(Lsun/awt/windows/WEmbeddedFramePeer;Lsun/awt/windows/WWindowPeer;Z)V",
    Any
)]
#[async_method]
pub async fn notify_modal_blocked_impl<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _blocked = parameters.pop_bool()?;
    let _blocker_peer = parameters.pop_reference()?;
    let _peer = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun/awt/windows/WEmbeddedFrame.notifyModalBlockedImpl(Lsun/awt/windows/WEmbeddedFramePeer;Lsun/awt/windows/WWindowPeer;Z)V".to_string()).into())
}
#[intrinsic_method("sun/awt/windows/WEmbeddedFrame.printBand(J[BIIIIIIIII)V", Any)]
#[async_method]
pub async fn print_band<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _dest_height = parameters.pop_int()?;
    let _dest_width = parameters.pop_int()?;
    let _dest_y = parameters.pop_int()?;
    let _dest_x = parameters.pop_int()?;
    let _src_height = parameters.pop_int()?;
    let _src_width = parameters.pop_int()?;
    let _src_y = parameters.pop_int()?;
    let _src_x = parameters.pop_int()?;
    let _offset = parameters.pop_int()?;
    let _image_array = parameters.pop_reference()?;
    let _the_hdc = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/awt/windows/WEmbeddedFrame.printBand(J[BIIIIIIIII)V".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_init_ids() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init_ids(thread, Parameters::default()).await;
        assert_eq!(
            "sun/awt/windows/WEmbeddedFrame.initIDs()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_is_printer_dc() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = is_printer_dc(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun/awt/windows/WEmbeddedFrame.isPrinterDC(J)Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_notify_modal_blocked_impl() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = notify_modal_blocked_impl(
            thread,
            Parameters::new(vec![
                Value::Object(None),
                Value::Object(None),
                Value::from(false),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/WEmbeddedFrame.notifyModalBlockedImpl(Lsun/awt/windows/WEmbeddedFramePeer;Lsun/awt/windows/WWindowPeer;Z)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_print_band() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = print_band(
            thread,
            Parameters::new(vec![
                Value::Long(0),
                Value::Object(None),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
                Value::Int(0),
            ]),
        )
        .await;
        assert_eq!(
            "sun/awt/windows/WEmbeddedFrame.printBand(J[BIIIIIIIII)V",
            result.unwrap_err().to_string()
        );
    }
}
