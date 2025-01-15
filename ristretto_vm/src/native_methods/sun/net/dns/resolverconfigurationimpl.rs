use crate::arguments::Arguments;
use crate::native_methods::registry::{MethodRegistry, JAVA_11};
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "sun/net/dns/ResolverConfigurationImpl";

/// Register all native methods for `sun.net.dns.ResolverConfigurationImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= JAVA_11 {
        registry.register(
            CLASS_NAME,
            "localDomain0",
            "()Ljava/lang/String;",
            local_domain_0,
        );
    }

    registry.register(
        CLASS_NAME,
        "fallbackDomain0",
        "()Ljava/lang/String;",
        fallback_domain_0,
    );
}

#[async_recursion(?Send)]
async fn fallback_domain_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.net.dns.ResolverConfigurationImpl.fallbackDomain0()Ljava/lang/String;")
}

#[async_recursion(?Send)]
async fn local_domain_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.net.dns.ResolverConfigurationImpl.localDomain0()Ljava/lang/String;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.net.dns.ResolverConfigurationImpl.fallbackDomain0()Ljava/lang/String;"
    )]
    async fn test_fallback_domain_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = fallback_domain_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.net.dns.ResolverConfigurationImpl.localDomain0()Ljava/lang/String;"
    )]
    async fn test_local_domain_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = local_domain_0(thread, Arguments::default()).await;
    }
}
