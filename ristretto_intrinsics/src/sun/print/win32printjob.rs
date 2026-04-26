use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/print/Win32PrintJob.endPrintRawData()Z", Any)]
#[async_method]
pub async fn end_print_raw_data<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun/print/Win32PrintJob.endPrintRawData()Z".to_string())
            .into(),
    )
}
#[intrinsic_method("sun/print/Win32PrintJob.printRawData([BI)Z", Any)]
#[async_method]
pub async fn print_raw_data<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _count = parameters.pop_int()?;
    let _data_array = parameters.pop_reference()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun/print/Win32PrintJob.printRawData([BI)Z".to_string())
            .into(),
    )
}
#[intrinsic_method(
    "sun/print/Win32PrintJob.startPrintRawData(Ljava/lang/String;Ljava/lang/String;)Z",
    Any
)]
#[async_method]
pub async fn start_print_raw_data<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _jobname = parameters.pop_reference()?;
    let _printer = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun/print/Win32PrintJob.startPrintRawData(Ljava/lang/String;Ljava/lang/String;)Z"
            .to_string(),
    )
    .into())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_end_print_raw_data() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = end_print_raw_data(thread, Parameters::default()).await;
        assert_eq!(
            "sun/print/Win32PrintJob.endPrintRawData()Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_print_raw_data() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = print_raw_data(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun/print/Win32PrintJob.printRawData([BI)Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_start_print_raw_data() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = start_print_raw_data(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun/print/Win32PrintJob.startPrintRawData(Ljava/lang/String;Ljava/lang/String;)Z",
            result.unwrap_err().to_string()
        );
    }
}
