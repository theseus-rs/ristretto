use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/print/PrintServiceLookupProvider.getAllPrinterNames()[Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_all_printer_names<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/print/PrintServiceLookupProvider.getAllPrinterNames()[Ljava/lang/String;".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/print/PrintServiceLookupProvider.getDefaultPrinterName()Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_default_printer_name<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/print/PrintServiceLookupProvider.getDefaultPrinterName()Ljava/lang/String;"
            .to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/print/PrintServiceLookupProvider.notifyLocalPrinterChange()V",
    Any
)]
#[async_method]
pub async fn notify_local_printer_change<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/print/PrintServiceLookupProvider.notifyLocalPrinterChange()V".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/print/PrintServiceLookupProvider.notifyRemotePrinterChange()V",
    Any
)]
#[async_method]
pub async fn notify_remote_printer_change<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/print/PrintServiceLookupProvider.notifyRemotePrinterChange()V".to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_all_printer_names() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_all_printer_names(thread, Parameters::default()).await;
        assert_eq!(
            "sun/print/PrintServiceLookupProvider.getAllPrinterNames()[Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_default_printer_name() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_default_printer_name(thread, Parameters::default()).await;
        assert_eq!(
            "sun/print/PrintServiceLookupProvider.getDefaultPrinterName()Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_notify_local_printer_change() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = notify_local_printer_change(thread, Parameters::default()).await;
        assert_eq!(
            "sun/print/PrintServiceLookupProvider.notifyLocalPrinterChange()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_notify_remote_printer_change() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = notify_remote_printer_change(thread, Parameters::default()).await;
        assert_eq!(
            "sun/print/PrintServiceLookupProvider.notifyRemotePrinterChange()V",
            result.unwrap_err().to_string()
        );
    }
}
