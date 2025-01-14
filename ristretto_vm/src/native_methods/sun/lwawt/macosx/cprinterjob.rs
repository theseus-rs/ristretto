use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.lwawt.macosx.CPrinterJob`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/lwawt/macosx/CPrinterJob";
    registry.register(class_name, "_safePrintLoop", "(JJ)V", safe_print_loop);
    registry.register(class_name, "abortDoc", "()V", abort_doc);
    registry.register(class_name, "createNSPrintInfo", "()J", create_ns_print_info);
    registry.register(class_name, "dispose", "(J)V", dispose);
    registry.register(
        class_name,
        "getDefaultPage",
        "(Ljava/awt/print/PageFormat;)V",
        get_default_page,
    );
    registry.register(class_name, "printLoop", "(ZII)Z", print_loop);
    registry.register(
        class_name,
        "validatePaper",
        "(Ljava/awt/print/Paper;Ljava/awt/print/Paper;)V",
        validate_paper,
    );
}

#[async_recursion(?Send)]
async fn safe_print_loop(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPrinterJob._safePrintLoop(JJ)V")
}

#[async_recursion(?Send)]
async fn abort_doc(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPrinterJob.abortDoc()V")
}

#[async_recursion(?Send)]
async fn create_ns_print_info(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPrinterJob.createNSPrintInfo()J")
}

#[async_recursion(?Send)]
async fn dispose(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPrinterJob.dispose(J)V")
}

#[async_recursion(?Send)]
async fn get_default_page(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPrinterJob.getDefaultPage(Ljava/awt/print/PageFormat;)V")
}

#[async_recursion(?Send)]
async fn print_loop(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.lwawt.macosx.CPrinterJob.printLoop(ZII)Z")
}

#[async_recursion(?Send)]
async fn validate_paper(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!(
        "sun.lwawt.macosx.CPrinterJob.validatePaper(Ljava/awt/print/Paper;Ljava/awt/print/Paper;)V"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "sun/lwawt/macosx/CPrinterJob";
        assert!(registry
            .method(class_name, "_safePrintLoop", "(JJ)V")
            .is_some());
        assert!(registry.method(class_name, "abortDoc", "()V").is_some());
        assert!(registry
            .method(class_name, "createNSPrintInfo", "()J")
            .is_some());
        assert!(registry.method(class_name, "dispose", "(J)V").is_some());
        assert!(registry
            .method(
                class_name,
                "getDefaultPage",
                "(Ljava/awt/print/PageFormat;)V"
            )
            .is_some());
        assert!(registry.method(class_name, "printLoop", "(ZII)Z").is_some());
        assert!(registry
            .method(
                class_name,
                "validatePaper",
                "(Ljava/awt/print/Paper;Ljava/awt/print/Paper;)V"
            )
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CPrinterJob._safePrintLoop(JJ)V")]
    async fn test_safe_print_loop() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = safe_print_loop(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CPrinterJob.abortDoc()V")]
    async fn test_abort_doc() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = abort_doc(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CPrinterJob.createNSPrintInfo()J")]
    async fn test_create_ns_print_info() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = create_ns_print_info(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CPrinterJob.dispose(J)V")]
    async fn test_dispose() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = dispose(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.lwawt.macosx.CPrinterJob.getDefaultPage(Ljava/awt/print/PageFormat;)V"
    )]
    async fn test_get_default_page() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_default_page(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "sun.lwawt.macosx.CPrinterJob.printLoop(ZII)Z")]
    async fn test_print_loop() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = print_loop(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.lwawt.macosx.CPrinterJob.validatePaper(Ljava/awt/print/Paper;Ljava/awt/print/Paper;)V"
    )]
    async fn test_validate_paper() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = validate_paper(thread, Arguments::default()).await;
    }
}
