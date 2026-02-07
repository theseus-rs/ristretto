use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::{Any, LessThanOrEqual};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/net/dns/ResolverConfigurationImpl.fallbackDomain0()Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn fallback_domain_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.net.dns.ResolverConfigurationImpl.fallbackDomain0()Ljava/lang/String;")
}

#[intrinsic_method(
    "sun/net/dns/ResolverConfigurationImpl.localDomain0()Ljava/lang/String;",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn local_domain_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
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
