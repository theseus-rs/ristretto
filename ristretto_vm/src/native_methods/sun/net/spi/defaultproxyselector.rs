use crate::native_methods::registry::{MethodRegistry, JAVA_8};
use crate::parameters::Parameters;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/net/spi/DefaultProxySelector";

/// Register all native methods for `sun.net.spi.DefaultProxySelector`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= JAVA_8 {
        registry.register(
            CLASS_NAME,
            "getSystemProxy",
            "(Ljava/lang/String;Ljava/lang/String;)Ljava/net/Proxy;",
            get_system_proxy,
        );
    } else {
        registry.register(
            CLASS_NAME,
            "getSystemProxies",
            "(Ljava/lang/String;Ljava/lang/String;)[Ljava/net/Proxy;",
            get_system_proxies,
        );
    }

    registry.register(CLASS_NAME, "init", "()Z", init);
}

#[async_recursion(?Send)]
async fn get_system_proxies(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.net.spi.DefaultProxySelector.getSystemProxies(Ljava/lang/String;Ljava/lang/String;)[Ljava/net/Proxy;")
}

#[async_recursion(?Send)]
async fn get_system_proxy(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
    todo!("sun.net.spi.DefaultProxySelector.getSystemProxy(Ljava/lang/String;Ljava/lang/String;)Ljava/net/Proxy;")
}

#[async_recursion(?Send)]
async fn init(_thread: Arc<Thread>, _parameters: Parameters) -> Result<Option<Value>> {
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
