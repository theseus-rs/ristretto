use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "java/lang/ProcessImpl.forkAndExec(I[B[B[BI[BI[B[IZ)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn fork_and_exec(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.lang.ProcessImpl.forkAndExec(I[B[B[BI[BI[B[IZ)I")
}

#[intrinsic_method("java/lang/ProcessImpl.init()V", GreaterThanOrEqual(JAVA_11))]
#[async_recursion(?Send)]
pub(crate) async fn init(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.lang.ProcessImpl.forkAndExec(I[B[B[BI[BI[B[IZ)I"
    )]
    async fn test_fork_and_exec() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = fork_and_exec(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_init() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init(thread, Parameters::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }
}
