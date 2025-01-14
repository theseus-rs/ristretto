use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `java.net.InetAddressImplFactory`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/net/InetAddressImplFactory";
    registry.register(class_name, "isIPv6Supported", "()Z", is_ipv_6_supported);
}

#[async_recursion(?Send)]
async fn is_ipv_6_supported(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.net.InetAddressImplFactory.isIPv6Supported()Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::default();
        register(&mut registry);
        let class_name = "java/net/InetAddressImplFactory";
        assert!(registry
            .method(class_name, "isIPv6Supported", "()Z")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.InetAddressImplFactory.isIPv6Supported()Z"
    )]
    async fn test_is_ipv_6_supported() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_ipv_6_supported(thread, Arguments::default()).await;
    }
}
