use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::{Any, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "sun/net/dns/ResolverConfigurationImpl.fallbackDomain0()Ljava/lang/String;",
    Any
)]
#[async_recursion(?Send)]
pub(crate) async fn fallback_domain_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.net.dns.ResolverConfigurationImpl.fallbackDomain0()Ljava/lang/String;")
}

#[intrinsic_method(
    "sun/net/dns/ResolverConfigurationImpl.localDomain0()Ljava/lang/String;",
    LessThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn local_domain_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
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
        let _ = fallback_domain_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.net.dns.ResolverConfigurationImpl.localDomain0()Ljava/lang/String;"
    )]
    async fn test_local_domain_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = local_domain_0(thread, Parameters::default()).await;
    }
}
