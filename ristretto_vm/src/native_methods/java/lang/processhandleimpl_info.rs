use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "java/lang/ProcessHandleImpl$Info";

/// Register all native methods for `java.lang.ProcessHandleImpl$Info`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "info0", "(J)V", info_0);
    registry.register(CLASS_NAME, "initIDs", "()V", init_ids);
}

#[async_recursion(?Send)]
async fn info_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("java.lang.ProcessHandleImpl$Info.info0(J)V")
}

#[async_recursion(?Send)]
async fn init_ids(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.lang.ProcessHandleImpl$Info.info0(J)V")]
    async fn test_info_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = info_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_init_ids() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Parameters::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }
}
