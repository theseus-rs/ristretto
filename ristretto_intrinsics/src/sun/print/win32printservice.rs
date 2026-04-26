use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/print/Win32PrintService.getAllMediaIDs(Ljava/lang/String;Ljava/lang/String;)[I",
    Any
)]
#[async_method]
pub async fn get_all_media_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _port = parameters.pop_reference()?;
    let _printer = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/print/Win32PrintService.getAllMediaIDs(Ljava/lang/String;Ljava/lang/String;)[I"
            .to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/print/Win32PrintService.getAllMediaNames(Ljava/lang/String;Ljava/lang/String;)[Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_all_media_names<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _port = parameters.pop_reference()?;
    let _printer = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun/print/Win32PrintService.getAllMediaNames(Ljava/lang/String;Ljava/lang/String;)[Ljava/lang/String;".to_string()).into())
}
#[intrinsic_method(
    "sun/print/Win32PrintService.getAllMediaSizes(Ljava/lang/String;Ljava/lang/String;)[I",
    Any
)]
#[async_method]
pub async fn get_all_media_sizes<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _port = parameters.pop_reference()?;
    let _printer = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/print/Win32PrintService.getAllMediaSizes(Ljava/lang/String;Ljava/lang/String;)[I"
            .to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/print/Win32PrintService.getAllMediaTrayNames(Ljava/lang/String;Ljava/lang/String;)[Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_all_media_tray_names<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _port = parameters.pop_reference()?;
    let _printer = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError("sun/print/Win32PrintService.getAllMediaTrayNames(Ljava/lang/String;Ljava/lang/String;)[Ljava/lang/String;".to_string()).into())
}
#[intrinsic_method(
    "sun/print/Win32PrintService.getAllMediaTrays(Ljava/lang/String;Ljava/lang/String;)[I",
    Any
)]
#[async_method]
pub async fn get_all_media_trays<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _port = parameters.pop_reference()?;
    let _printer = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/print/Win32PrintService.getAllMediaTrays(Ljava/lang/String;Ljava/lang/String;)[I"
            .to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/print/Win32PrintService.getAllResolutions(Ljava/lang/String;Ljava/lang/String;)[I",
    Any
)]
#[async_method]
pub async fn get_all_resolutions<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _port = parameters.pop_reference()?;
    let _printer = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/print/Win32PrintService.getAllResolutions(Ljava/lang/String;Ljava/lang/String;)[I"
            .to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/print/Win32PrintService.getCapabilities(Ljava/lang/String;Ljava/lang/String;)I",
    Any
)]
#[async_method]
pub async fn get_capabilities<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _port = parameters.pop_reference()?;
    let _printer = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/print/Win32PrintService.getCapabilities(Ljava/lang/String;Ljava/lang/String;)I"
            .to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/print/Win32PrintService.getCopiesSupported(Ljava/lang/String;Ljava/lang/String;)I",
    Any
)]
#[async_method]
pub async fn get_copies_supported<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _port = parameters.pop_reference()?;
    let _printer = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/print/Win32PrintService.getCopiesSupported(Ljava/lang/String;Ljava/lang/String;)I"
            .to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/print/Win32PrintService.getDefaultSettings(Ljava/lang/String;Ljava/lang/String;)[I",
    Any
)]
#[async_method]
pub async fn get_default_settings<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _port = parameters.pop_reference()?;
    let _printer = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/print/Win32PrintService.getDefaultSettings(Ljava/lang/String;Ljava/lang/String;)[I"
            .to_string(),
    )
    .into())
}
#[intrinsic_method("sun/print/Win32PrintService.getJobStatus(Ljava/lang/String;I)I", Any)]
#[async_method]
pub async fn get_job_status<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _type_ = parameters.pop_int()?;
    let _printer = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/print/Win32PrintService.getJobStatus(Ljava/lang/String;I)I".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/print/Win32PrintService.getMediaPrintableArea(Ljava/lang/String;I)[F",
    Any
)]
#[async_method]
pub async fn get_media_printable_area<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _papersize = parameters.pop_int()?;
    let _printer = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/print/Win32PrintService.getMediaPrintableArea(Ljava/lang/String;I)[F".to_string(),
    )
    .into())
}
#[intrinsic_method(
    "sun/print/Win32PrintService.getPrinterPort(Ljava/lang/String;)Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_printer_port<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _printer = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/print/Win32PrintService.getPrinterPort(Ljava/lang/String;)Ljava/lang/String;"
            .to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_all_media_ids() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_all_media_ids(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun/print/Win32PrintService.getAllMediaIDs(Ljava/lang/String;Ljava/lang/String;)[I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_all_media_names() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_all_media_names(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun/print/Win32PrintService.getAllMediaNames(Ljava/lang/String;Ljava/lang/String;)[Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_all_media_sizes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_all_media_sizes(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun/print/Win32PrintService.getAllMediaSizes(Ljava/lang/String;Ljava/lang/String;)[I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_all_media_tray_names() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_all_media_tray_names(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun/print/Win32PrintService.getAllMediaTrayNames(Ljava/lang/String;Ljava/lang/String;)[Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_all_media_trays() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_all_media_trays(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun/print/Win32PrintService.getAllMediaTrays(Ljava/lang/String;Ljava/lang/String;)[I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_all_resolutions() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_all_resolutions(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun/print/Win32PrintService.getAllResolutions(Ljava/lang/String;Ljava/lang/String;)[I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_capabilities() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_capabilities(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun/print/Win32PrintService.getCapabilities(Ljava/lang/String;Ljava/lang/String;)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_copies_supported() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_copies_supported(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun/print/Win32PrintService.getCopiesSupported(Ljava/lang/String;Ljava/lang/String;)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_default_settings() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_default_settings(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun/print/Win32PrintService.getDefaultSettings(Ljava/lang/String;Ljava/lang/String;)[I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_job_status() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_job_status(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/print/Win32PrintService.getJobStatus(Ljava/lang/String;I)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_media_printable_area() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_media_printable_area(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/print/Win32PrintService.getMediaPrintableArea(Ljava/lang/String;I)[F",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_get_printer_port() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_printer_port(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun/print/Win32PrintService.getPrinterPort(Ljava/lang/String;)Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }
}
