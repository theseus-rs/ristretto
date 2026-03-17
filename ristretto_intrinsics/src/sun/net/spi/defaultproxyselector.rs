use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::{Any, GreaterThan, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/net/spi/DefaultProxySelector.getSystemProxies(Ljava/lang/String;Ljava/lang/String;)[Ljava/net/Proxy;",
    GreaterThan(JAVA_8)
)]
#[async_method]
pub async fn get_system_proxies<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.net.spi.DefaultProxySelector.getSystemProxies(Ljava/lang/String;Ljava/lang/String;)[Ljava/net/Proxy;".to_string()).into())
}

#[intrinsic_method(
    "sun/net/spi/DefaultProxySelector.getSystemProxy(Ljava/lang/String;Ljava/lang/String;)Ljava/net/Proxy;",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_system_proxy<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError("sun.net.spi.DefaultProxySelector.getSystemProxy(Ljava/lang/String;Ljava/lang/String;)Ljava/net/Proxy;".to_string()).into())
}

#[intrinsic_method("sun/net/spi/DefaultProxySelector.init()Z", Any)]
#[async_method]
pub async fn init<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_system_proxies() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_system_proxies(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_system_proxy() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_system_proxy(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_init() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }
}
