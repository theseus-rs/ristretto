use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `jdk.internal.vm.ForeignLinkerSupport`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "jdk/internal/vm/ForeignLinkerSupport";
    registry.register(class_name, "isSupported0", "()Z", is_supported_0);
}

#[async_recursion(?Send)]
async fn is_supported_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.internal.vm.ForeignLinkerSupport.isSupported0()Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "jdk/internal/vm/ForeignLinkerSupport";
        assert!(registry.method(class_name, "isSupported0", "()Z").is_some());
    }

    #[tokio::test]
    #[should_panic(expected = "jdk.internal.vm.ForeignLinkerSupport.isSupported0()Z")]
    async fn test_is_supported_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_supported_0(thread, Arguments::default()).await;
    }
}
