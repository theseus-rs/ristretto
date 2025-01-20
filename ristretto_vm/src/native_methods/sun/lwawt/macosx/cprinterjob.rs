use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/lwawt/macosx/CPrinterJob";

/// Register all native methods for `sun.lwawt.macosx.CPrinterJob`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "_safePrintLoop", "(JJ)V", safe_print_loop);
    registry.register(CLASS_NAME, "abortDoc", "()V", abort_doc);
    registry.register(CLASS_NAME, "createNSPrintInfo", "()J", create_ns_print_info);
    registry.register(CLASS_NAME, "dispose", "(J)V", dispose);
    registry.register(
        CLASS_NAME,
        "getDefaultPage",
        "(Ljava/awt/print/PageFormat;)V",
        get_default_page,
    );
    registry.register(CLASS_NAME, "printLoop", "(ZII)Z", print_loop);
    registry.register(
        CLASS_NAME,
        "validatePaper",
        "(Ljava/awt/print/Paper;Ljava/awt/print/Paper;)V",
        validate_paper,
    );
}

#[async_recursion(?Send)]
async fn safe_print_loop(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPrinterJob._safePrintLoop(JJ)V")
}

#[async_recursion(?Send)]
async fn abort_doc(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPrinterJob.abortDoc()V")
}

#[async_recursion(?Send)]
async fn create_ns_print_info(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPrinterJob.createNSPrintInfo()J")
}

#[async_recursion(?Send)]
async fn dispose(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPrinterJob.dispose(J)V")
}

#[async_recursion(?Send)]
async fn get_default_page(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPrinterJob.getDefaultPage(Ljava/awt/print/PageFormat;)V")
}

#[async_recursion(?Send)]
async fn print_loop(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPrinterJob.printLoop(ZII)Z")
}

#[async_recursion(?Send)]
async fn validate_paper(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
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
