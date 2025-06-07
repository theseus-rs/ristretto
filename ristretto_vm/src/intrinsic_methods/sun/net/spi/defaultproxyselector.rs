use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::{Any, GreaterThan, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/net/spi/DefaultProxySelector.getSystemProxies(Ljava/lang/String;Ljava/lang/String;)[Ljava/net/Proxy;",
    GreaterThan(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_system_proxies(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.net.spi.DefaultProxySelector.getSystemProxies(Ljava/lang/String;Ljava/lang/String;)[Ljava/net/Proxy;"
    )
}

#[intrinsic_method(
    "sun/net/spi/DefaultProxySelector.getSystemProxy(Ljava/lang/String;Ljava/lang/String;)Ljava/net/Proxy;",
    LessThanOrEqual(JAVA_8)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_system_proxy(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.net.spi.DefaultProxySelector.getSystemProxy(Ljava/lang/String;Ljava/lang/String;)Ljava/net/Proxy;"
    )
}

#[intrinsic_method("sun/net/spi/DefaultProxySelector.init()Z", Any)]
#[async_recursion(?Send)]
pub(crate) async fn init(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.net.spi.DefaultProxySelector.getSystemProxies(Ljava/lang/String;Ljava/lang/String;)[Ljava/net/Proxy;"
    )]
    async fn test_get_system_proxies() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_system_proxies(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.net.spi.DefaultProxySelector.getSystemProxy(Ljava/lang/String;Ljava/lang/String;)Ljava/net/Proxy;"
    )]
    async fn test_get_system_proxy() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_system_proxy(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_init() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }
}
