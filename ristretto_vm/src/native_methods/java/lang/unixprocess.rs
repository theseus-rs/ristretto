use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.lang.UNIXProcess`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/lang/UNIXProcess";
    registry.register(class_name, "destroyProcess", "(IZ)V", destroy_process);
    registry.register(
        class_name,
        "forkAndExec",
        "(I[B[B[BI[BI[B[IZ)I",
        fork_and_exec,
    );
    registry.register(class_name, "init", "()V", init);
    registry.register(
        class_name,
        "waitForProcessExit",
        "(I)I",
        wait_for_process_exit,
    );
}

#[async_recursion(?Send)]
async fn destroy_process(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.UNIXProcess.destroyProcess(IZ)V")
}

#[async_recursion(?Send)]
async fn fork_and_exec(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.UNIXProcess.forkAndExec(I[B[B[BI[BI[B[IZ)I")
}

#[async_recursion(?Send)]
async fn init(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.UNIXProcess.init()V")
}

#[async_recursion(?Send)]
async fn wait_for_process_exit(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("java.lang.UNIXProcess.waitForProcessExit(I)I")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "java/lang/UNIXProcess";
        assert!(registry
            .method(class_name, "destroyProcess", "(IZ)V")
            .is_some());
        assert!(registry
            .method(class_name, "forkAndExec", "(I[B[B[BI[BI[B[IZ)I")
            .is_some());
        assert!(registry.method(class_name, "init", "()V").is_some());
        assert!(registry
            .method(class_name, "waitForProcessExit", "(I)I")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.lang.UNIXProcess.destroyProcess(IZ)V")]
    async fn test_destroy_process() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = destroy_process(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.UNIXProcess.forkAndExec(I[B[B[BI[BI[B[IZ)I"
    )]
    async fn test_fork_and_exec() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = fork_and_exec(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.lang.UNIXProcess.init()V")]
    async fn test_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.lang.UNIXProcess.waitForProcessExit(I)I")]
    async fn test_wait_for_process_exit() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = wait_for_process_exit(thread, Arguments::default()).await;
    }
}
