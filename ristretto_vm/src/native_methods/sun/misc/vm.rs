use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/misc/VM";

/// Register all native methods for `sun.misc.VM`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "initialize", "()V", initialize);
    registry.register(
        CLASS_NAME,
        "latestUserDefinedLoader0",
        "()Ljava/lang/ClassLoader;",
        latest_user_defined_loader_0,
    );
}

#[async_recursion(?Send)]
async fn initialize(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn latest_user_defined_loader_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.misc.VM.latestUserDefinedLoader0()Ljava/lang/ClassLoader;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_initialize() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = initialize(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.misc.VM.latestUserDefinedLoader0()Ljava/lang/ClassLoader;"
    )]
    async fn test_latest_user_defined_loader_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = latest_user_defined_loader_0(thread, Parameters::default()).await;
    }
}
