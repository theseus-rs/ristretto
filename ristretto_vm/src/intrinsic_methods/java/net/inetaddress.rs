use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_21;
use ristretto_classfile::VersionSpecification::{Any, GreaterThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method("java/net/InetAddress.init()V", Any)]
#[async_recursion(?Send)]
pub(crate) async fn init(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method("java/net/InetAddress.isIPv4Available()Z", GreaterThanOrEqual(JAVA_21))]
#[async_recursion(?Send)]
pub(crate) async fn is_ipv_4_available(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("java.net.InetAddress.isIPv4Available()Z")
}

#[intrinsic_method("java/net/InetAddress.isIPv6Supported()Z", GreaterThanOrEqual(JAVA_21))]
#[async_recursion(?Send)]
pub(crate) async fn is_ipv_6_supported(
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
