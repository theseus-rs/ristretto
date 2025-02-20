use crate::Result;
use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "java/lang/UNIXProcess";

/// Register all native methods for `java.lang.UNIXProcess`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "destroyProcess", "(IZ)V", destroy_process);
    registry.register(
        CLASS_NAME,
        "forkAndExec",
        "(I[B[B[BI[BI[B[IZ)I",
        fork_and_exec,
    );
    registry.register(CLASS_NAME, "init", "()V", init);
    registry.register(
        CLASS_NAME,
        "waitForProcessExit",
        "(I)I",
        wait_for_process_exit,
    );
}

#[async_recursion(?Send)]
async fn destroy_process(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.UNIXProcess.destroyProcess(IZ)V")
}

#[async_recursion(?Send)]
async fn fork_and_exec(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.UNIXProcess.forkAndExec(I[B[B[BI[BI[B[IZ)I")
}

#[async_recursion(?Send)]
async fn init(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.UNIXProcess.init()V")
}

#[async_recursion(?Send)]
async fn wait_for_process_exit(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.UNIXProcess.waitForProcessExit(I)I")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.lang.UNIXProcess.destroyProcess(IZ)V")]
    async fn test_destroy_process() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = destroy_process(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.UNIXProcess.forkAndExec(I[B[B[BI[BI[B[IZ)I"
    )]
    async fn test_fork_and_exec() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = fork_and_exec(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.lang.UNIXProcess.init()V")]
    async fn test_init() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = init(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.lang.UNIXProcess.waitForProcessExit(I)I")]
    async fn test_wait_for_process_exit() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = wait_for_process_exit(thread, Parameters::default()).await;
    }
}
