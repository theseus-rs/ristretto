use crate::arguments::Arguments;
use crate::native_methods::registry::{MethodRegistry, JAVA_18, JAVA_19};
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "java/net/InetAddress";

/// Register all native methods for `java.net.InetAddress`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() >= JAVA_18 {
        registry.register(CLASS_NAME, "isIPv4Available", "()Z", is_ipv_4_available);
    }
    if registry.java_major_version() >= JAVA_19 {
        registry.register(CLASS_NAME, "isIPv6Supported", "()Z", is_ipv_6_supported);
    }

    registry.register(CLASS_NAME, "init", "()V", init);
}

#[async_recursion(?Send)]
async fn init(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn is_ipv_4_available(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.net.InetAddress.isIPv4Available()Z")
}

#[async_recursion(?Send)]
async fn is_ipv_6_supported(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("java.net.InetAddress.isIPv6Supported()Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_init() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init(thread, Arguments::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.net.InetAddress.isIPv4Available()Z")]
    async fn test_is_ipv_4_available() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_ipv_4_available(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.net.InetAddress.isIPv6Supported()Z")]
    async fn test_is_ipv_6_supported() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_ipv_6_supported(thread, Arguments::default()).await;
    }
}
