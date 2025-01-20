use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "jdk/internal/vm/ForeignLinkerSupport";

/// Register all native methods for `jdk.internal.vm.ForeignLinkerSupport`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "isSupported0", "()Z", is_supported_0);
}

#[async_recursion(?Send)]
async fn is_supported_0(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("jdk.internal.vm.ForeignLinkerSupport.isSupported0()Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.internal.vm.ForeignLinkerSupport.isSupported0()Z"
    )]
    async fn test_is_supported_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_supported_0(thread, Parameters::default()).await;
    }
}
