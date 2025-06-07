use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::Any;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("sun/lwawt/macosx/CPrinterJob._safePrintLoop(JJ)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn safe_print_loop(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPrinterJob._safePrintLoop(JJ)V")
}

#[intrinsic_method("sun/lwawt/macosx/CPrinterJob.abortDoc()V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn abort_doc(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPrinterJob.abortDoc()V")
}

#[intrinsic_method("sun/lwawt/macosx/CPrinterJob.createNSPrintInfo()J", Any)]
#[async_recursion(?Send)]
pub(crate) async fn create_ns_print_info(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPrinterJob.createNSPrintInfo()J")
}

#[intrinsic_method("sun/lwawt/macosx/CPrinterJob.dispose(J)V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn dispose(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPrinterJob.dispose(J)V")
}

#[intrinsic_method(
    "sun/lwawt/macosx/CPrinterJob.getDefaultPage(Ljava/awt/print/PageFormat;)V",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn get_default_page(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPrinterJob.getDefaultPage(Ljava/awt/print/PageFormat;)V")
}

#[intrinsic_method("sun/lwawt/macosx/CPrinterJob.printLoop(ZII)Z", Any)]
#[async_recursion(?Send)]
pub(crate) async fn print_loop(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPrinterJob.printLoop(ZII)Z")
}

#[intrinsic_method(
    "sun/lwawt/macosx/CPrinterJob.validatePaper(Ljava/awt/print/Paper;Ljava/awt/print/Paper;)V",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn validate_paper(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.lwawt.macosx.CPrinterJob.validatePaper(Ljava/awt/print/Paper;Ljava/awt/print/Paper;)V"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CPrinterJob._safePrintLoop(JJ)V"
    )]
    async fn test_safe_print_loop() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = safe_print_loop(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.lwawt.macosx.CPrinterJob.abortDoc()V")]
    async fn test_abort_doc() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = abort_doc(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CPrinterJob.createNSPrintInfo()J"
    )]
    async fn test_create_ns_print_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create_ns_print_info(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.lwawt.macosx.CPrinterJob.dispose(J)V")]
    async fn test_dispose() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = dispose(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CPrinterJob.getDefaultPage(Ljava/awt/print/PageFormat;)V"
    )]
    async fn test_get_default_page() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_default_page(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.lwawt.macosx.CPrinterJob.printLoop(ZII)Z")]
    async fn test_print_loop() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = print_loop(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.lwawt.macosx.CPrinterJob.validatePaper(Ljava/awt/print/Paper;Ljava/awt/print/Paper;)V"
    )]
    async fn test_validate_paper() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = validate_paper(thread, Parameters::default()).await;
    }
}
