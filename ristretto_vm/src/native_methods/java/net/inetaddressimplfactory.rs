use crate::Result;
use crate::native_methods::registry::MethodRegistry;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "java/net/InetAddressImplFactory";

/// Register all native methods for `java.net.InetAddressImplFactory`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    registry.register(CLASS_NAME, "isIPv6Supported", "()Z", is_ipv_6_supported);
}

#[async_recursion(?Send)]
async fn is_ipv_6_supported(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.net.InetAddressImplFactory.isIPv6Supported()Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: java.net.InetAddressImplFactory.isIPv6Supported()Z"
    )]
    async fn test_is_ipv_6_supported() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_ipv_6_supported(thread, Parameters::default()).await;
    }
}
