use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method("sun/lwawt/macosx/CPrinterJob._safePrintLoop(JJ)V", Any)]
#[async_method]
pub async fn safe_print_loop<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _arg = parameters.pop_long()?;
    let _target = parameters.pop_long()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CPrinterJob._safePrintLoop(JJ)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CPrinterJob.abortDoc()V", Any)]
#[async_method]
pub async fn abort_doc<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(
        JavaError::UnsatisfiedLinkError("sun.lwawt.macosx.CPrinterJob.abortDoc()V".to_string())
            .into(),
    )
}

#[intrinsic_method("sun/lwawt/macosx/CPrinterJob.createNSPrintInfo()J", Any)]
#[async_method]
pub async fn create_ns_print_info<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CPrinterJob.createNSPrintInfo()J".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CPrinterJob.dispose(J)V", Any)]
#[async_method]
pub async fn dispose<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _print_info = parameters.pop_long()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun.lwawt.macosx.CPrinterJob.dispose(J)V".to_string())
            .into(),
    )
}

#[intrinsic_method(
    "sun/lwawt/macosx/CPrinterJob.getDefaultPage(Ljava/awt/print/PageFormat;)V",
    Any
)]
#[async_method]
pub async fn get_default_page<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _page = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CPrinterJob.getDefaultPage(Ljava/awt/print/PageFormat;)V".to_string(),
    )
    .into())
}

#[intrinsic_method("sun/lwawt/macosx/CPrinterJob.printLoop(ZII)Z", Any)]
#[async_method]
pub async fn print_loop<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _last_page = parameters.pop_int()?;
    let _first_page = parameters.pop_int()?;
    let _wait_until_done = parameters.pop_bool()?;
    Err(
        JavaError::UnsatisfiedLinkError("sun.lwawt.macosx.CPrinterJob.printLoop(ZII)Z".to_string())
            .into(),
    )
}

#[intrinsic_method(
    "sun/lwawt/macosx/CPrinterJob.validatePaper(Ljava/awt/print/Paper;Ljava/awt/print/Paper;)V",
    Any
)]
#[async_method]
pub async fn validate_paper<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _newpaper = parameters.pop_reference()?;
    let _origpaper = parameters.pop_reference()?;
    Err(JavaError::UnsatisfiedLinkError(
        "sun.lwawt.macosx.CPrinterJob.validatePaper(Ljava/awt/print/Paper;Ljava/awt/print/Paper;)V"
            .to_string(),
    )
    .into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_safe_print_loop() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = safe_print_loop(
            thread,
            Parameters::new(vec![Value::Long(0), Value::Long(0)]),
        )
        .await;
        assert_eq!(
            "sun.lwawt.macosx.CPrinterJob._safePrintLoop(JJ)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_abort_doc() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = abort_doc(thread, Parameters::default()).await;
        assert_eq!(
            "sun.lwawt.macosx.CPrinterJob.abortDoc()V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_create_ns_print_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = create_ns_print_info(thread, Parameters::default()).await;
        assert_eq!(
            "sun.lwawt.macosx.CPrinterJob.createNSPrintInfo()J",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_dispose() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = dispose(thread, Parameters::new(vec![Value::Long(0)])).await;
        assert_eq!(
            "sun.lwawt.macosx.CPrinterJob.dispose(J)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_get_default_page() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_default_page(thread, Parameters::new(vec![Value::Object(None)])).await;
        assert_eq!(
            "sun.lwawt.macosx.CPrinterJob.getDefaultPage(Ljava/awt/print/PageFormat;)V",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_print_loop() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = print_loop(
            thread,
            Parameters::new(vec![Value::from(false), Value::Int(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "sun.lwawt.macosx.CPrinterJob.printLoop(ZII)Z",
            result.unwrap_err().to_string()
        );
    }

    #[tokio::test]
    async fn test_validate_paper() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = validate_paper(
            thread,
            Parameters::new(vec![Value::Object(None), Value::Object(None)]),
        )
        .await;
        assert_eq!(
            "sun.lwawt.macosx.CPrinterJob.validatePaper(Ljava/awt/print/Paper;Ljava/awt/print/Paper;)V",
            result.unwrap_err().to_string()
        );
    }
}
