use crate::Result;
use crate::intrinsic_methods::registry::{JAVA_21, MethodRegistry};
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "java/net/InetAddress";

/// Register all intrinsic methods for `java.net.InetAddress`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() >= JAVA_21 {
        registry.register(CLASS_NAME, "isIPv4Available", "()Z", is_ipv_4_available);
        registry.register(CLASS_NAME, "isIPv6Supported", "()Z", is_ipv_6_supported);
    }

    registry.register(CLASS_NAME, "init", "()V", init);
}

#[async_recursion(?Send)]
async fn init(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[async_recursion(?Send)]
async fn is_ipv_4_available(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.net.InetAddress.isIPv4Available()Z")
}

#[async_recursion(?Send)]
async fn is_ipv_6_supported(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.net.InetAddress.isIPv6Supported()Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_init() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init(thread, Parameters::default()).await?;
        assert_eq!(None, result);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.net.InetAddress.isIPv4Available()Z")]
    async fn test_is_ipv_4_available() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_ipv_4_available(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: java.net.InetAddress.isIPv6Supported()Z")]
    async fn test_is_ipv_6_supported() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = is_ipv_6_supported(thread, Parameters::default()).await;
    }
}
