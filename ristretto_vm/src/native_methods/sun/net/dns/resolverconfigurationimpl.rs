use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

/// Register all native methods for `sun.net.dns.ResolverConfigurationImpl`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "sun/net/dns/ResolverConfigurationImpl";
    registry.register(
        class_name,
        "fallbackDomain0",
        "()Ljava/lang/String;",
        fallback_domain_0,
    );
}

#[async_recursion(?Send)]
async fn fallback_domain_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("sun.net.dns.ResolverConfigurationImpl.fallbackDomain0()Ljava/lang/String;")
}
