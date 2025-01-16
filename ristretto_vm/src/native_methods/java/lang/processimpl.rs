use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "java/lang/ProcessImpl";

/// Register all native methods for `java.lang.ProcessImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(
        CLASS_NAME,
        "forkAndExec",
        "(I[B[B[BI[BI[B[IZ)I",
        fork_and_exec,
    );
    registry.register(CLASS_NAME, "init", "()V", init);
}

#[async_recursion(?Send)]
async fn fork_and_exec(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.lang.ProcessImpl.forkAndExec(I[B[B[BI[BI[B[IZ)I")
}

#[async_recursion(?Send)]
async fn init(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
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
        let _ = fork_and_exec(thread, Arguments::default()).await;
    }

    #[tokio::test]
    async fn test_init() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init(thread, Arguments::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }
}
