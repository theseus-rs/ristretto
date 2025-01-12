use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_18: Version = Version::Java18 { minor: 0 };
const JAVA_19: Version = Version::Java19 { minor: 0 };

/// Register all native methods for `java.net.InetAddress`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "java/net/InetAddress";
    let java_version = registry.java_version().clone();

    if java_version >= JAVA_18 {
        registry.register(class_name, "isIPv4Available", "()Z", is_ipv_4_available);
    }
    if java_version >= JAVA_19 {
        registry.register(class_name, "isIPv6Supported", "()Z", is_ipv_6_supported);
    }

    registry.register(class_name, "init", "()V", init);
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

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::new(&Version::Java18 { minor: 0 }, true);
        register(&mut registry);
        let class_name = "java/net/InetAddress";
        assert!(registry
            .method(class_name, "isIPv4Available", "()Z")
            .is_some());
        assert!(registry.method(class_name, "init", "()V").is_some());
    }

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
