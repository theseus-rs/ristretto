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
