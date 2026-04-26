#[cfg(target_family = "unix")]
use ristretto_classfile::JAVA_11;
#[cfg(not(target_family = "wasm"))]
use ristretto_classfile::VersionSpecification::Any;
#[cfg(target_family = "unix")]
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
#[cfg(not(target_family = "wasm"))]
use ristretto_classloader::Value;
#[cfg(not(target_family = "wasm"))]
use ristretto_macros::async_method;
#[cfg(not(target_family = "wasm"))]
use ristretto_macros::intrinsic_method;
#[cfg(not(target_family = "wasm"))]
use ristretto_types::JavaError;
#[cfg(not(target_family = "wasm"))]
use ristretto_types::Thread;
#[cfg(not(target_family = "wasm"))]
use ristretto_types::{Parameters, Result};
#[cfg(not(target_family = "wasm"))]
use std::sync::Arc;

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "sun/net/dns/ResolverConfigurationImpl.fallbackDomain0()Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn fallback_domain_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.net.dns.ResolverConfigurationImpl.fallbackDomain0()Ljava/lang/String;".to_string(),
    )
    .into())
}

#[cfg(target_family = "unix")]
#[intrinsic_method(
    "sun/net/dns/ResolverConfigurationImpl.localDomain0()Ljava/lang/String;",
    LessThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn local_domain_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun.net.dns.ResolverConfigurationImpl.localDomain0()Ljava/lang/String;".to_string(),
    )
    .into())
}

#[cfg(target_os = "windows")]
#[intrinsic_method("sun/net/dns/ResolverConfigurationImpl.init0()V", Any)]
#[async_method]
pub async fn init0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/net/dns/ResolverConfigurationImpl.init0()V".to_string(),
    )
    .into())
}

#[cfg(target_os = "windows")]
#[intrinsic_method("sun/net/dns/ResolverConfigurationImpl.loadDNSconfig0()V", Any)]
#[async_method]
pub async fn load_dnsconfig0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/net/dns/ResolverConfigurationImpl.loadDNSconfig0()V".to_string(),
    )
    .into())
}

#[cfg(target_os = "windows")]
#[intrinsic_method("sun/net/dns/ResolverConfigurationImpl.notifyAddrChange0()I", Any)]
#[async_method]
pub async fn notify_addr_change0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/net/dns/ResolverConfigurationImpl.notifyAddrChange0()I".to_string(),
    )
    .into())
}

#[cfg(target_os = "windows")]
#[intrinsic_method("sun/net/dns/ResolverConfigurationImpl.init0()V", Any)]
#[async_method]
pub async fn init0_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/net/dns/ResolverConfigurationImpl.init0()V".to_string(),
    )
    .into())
}

#[cfg(target_os = "windows")]
#[intrinsic_method("sun/net/dns/ResolverConfigurationImpl.loadDNSconfig0()V", Any)]
#[async_method]
pub async fn load_dnsconfig0_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/net/dns/ResolverConfigurationImpl.loadDNSconfig0()V".to_string(),
    )
    .into())
}

#[cfg(target_os = "windows")]
#[intrinsic_method("sun/net/dns/ResolverConfigurationImpl.notifyAddrChange0()I", Any)]
#[async_method]
pub async fn notify_addr_change0_windows<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "sun/net/dns/ResolverConfigurationImpl.notifyAddrChange0()I".to_string(),
    )
    .into())
}

#[cfg(all(test, any(target_family = "unix", target_os = "windows")))]
mod tests {
    use super::*;

    #[cfg(target_family = "unix")]
    #[tokio::test]
    async fn test_fallback_domain_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = fallback_domain_0(thread, Parameters::default()).await;
        assert_eq!(
            "sun.net.dns.ResolverConfigurationImpl.fallbackDomain0()Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_family = "unix")]
    #[tokio::test]
    async fn test_local_domain_0() {
        let (_vm, thread) = crate::test::java11_thread().await.expect("thread");
        let result = local_domain_0(thread, Parameters::default()).await;
        assert_eq!(
            "sun.net.dns.ResolverConfigurationImpl.localDomain0()Ljava/lang/String;",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_init0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init0(thread, Parameters::default()).await;
        assert_eq!(
            "sun/net/dns/ResolverConfigurationImpl.init0()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_load_dnsconfig0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = load_dnsconfig0(thread, Parameters::default()).await;
        assert_eq!(
            "sun/net/dns/ResolverConfigurationImpl.loadDNSconfig0()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_notify_addr_change0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = notify_addr_change0(thread, Parameters::default()).await;
        assert_eq!(
            "sun/net/dns/ResolverConfigurationImpl.notifyAddrChange0()I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_init0_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = init0_windows(thread, Parameters::default()).await;
        assert_eq!(
            "sun/net/dns/ResolverConfigurationImpl.init0()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_load_dnsconfig0_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = load_dnsconfig0_windows(thread, Parameters::default()).await;
        assert_eq!(
            "sun/net/dns/ResolverConfigurationImpl.loadDNSconfig0()V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "windows")]
    #[tokio::test]
    async fn test_notify_addr_change0_windows() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = notify_addr_change0_windows(thread, Parameters::default()).await;
        assert_eq!(
            "sun/net/dns/ResolverConfigurationImpl.notifyAddrChange0()I",
            result.unwrap_err().to_string()
        );
    }
}
