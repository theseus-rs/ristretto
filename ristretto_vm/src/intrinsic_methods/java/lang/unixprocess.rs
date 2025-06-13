use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("java/lang/UNIXProcess.destroyProcess(IZ)V", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn destroy_process(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.UNIXProcess.destroyProcess(IZ)V")
}

#[intrinsic_method(
    "java/lang/UNIXProcess.forkAndExec(I[B[B[BI[BI[B[IZ)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn fork_and_exec(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.UNIXProcess.forkAndExec(I[B[B[BI[BI[B[IZ)I")
}

#[intrinsic_method("java/lang/UNIXProcess.init()V", LessThanOrEqual(JAVA_8))]
#[async_recursion(?Send)]
pub(crate) async fn init(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "java/lang/UNIXProcess.waitForProcessExit(I)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn wait_for_process_exit(
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
    async fn test_init() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.lang.UNIXProcess.waitForProcessExit(I)I")]
    async fn test_wait_for_process_exit() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = wait_for_process_exit(thread, Parameters::default()).await;
    }
}
