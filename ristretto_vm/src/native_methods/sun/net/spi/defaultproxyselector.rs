use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_8: Version = Version::Java8 { minor: 0 };

/// Register all native methods for `sun.net.spi.DefaultProxySelector`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/net/spi/DefaultProxySelector";
    let java_version = registry.java_version();

    if java_version <= &JAVA_8 {
        registry.register(
            class_name,
            "getSystemProxy",
            "(Ljava/lang/String;Ljava/lang/String;)Ljava/net/Proxy;",
            get_system_proxy,
        );
    } else {
        registry.register(
            class_name,
            "getSystemProxies",
            "(Ljava/lang/String;Ljava/lang/String;)[Ljava/net/Proxy;",
            get_system_proxies,
        );
    }

    registry.register(class_name, "init", "()Z", init);
}

#[async_recursion(?Send)]
async fn get_system_proxies(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.net.spi.DefaultProxySelector.getSystemProxies(Ljava/lang/String;Ljava/lang/String;)[Ljava/net/Proxy;")
}

#[async_recursion(?Send)]
async fn get_system_proxy(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.net.spi.DefaultProxySelector.getSystemProxy(Ljava/lang/String;Ljava/lang/String;)Ljava/net/Proxy;")
}

#[async_recursion(?Send)]
async fn init(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_classfile::Version::Java9;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::new(&Java9 { minor: 0 }, true);
        register(&mut registry);
        let class_name = "sun/net/spi/DefaultProxySelector";
        assert!(registry
            .method(
                class_name,
                "getSystemProxies",
                "(Ljava/lang/String;Ljava/lang/String;)[Ljava/net/Proxy;"
            )
            .is_some());
        assert!(registry.method(class_name, "init", "()Z").is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.net.spi.DefaultProxySelector.getSystemProxies(Ljava/lang/String;Ljava/lang/String;)[Ljava/net/Proxy;"
    )]
    async fn test_get_system_proxies() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_system_proxies(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "sun.net.spi.DefaultProxySelector.getSystemProxy(Ljava/lang/String;Ljava/lang/String;)Ljava/net/Proxy;"
    )]
    async fn test_get_system_proxy() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_system_proxy(thread, Arguments::default()).await;
    }

    #[tokio::test]
    async fn test_init() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init(thread, Arguments::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }
}
