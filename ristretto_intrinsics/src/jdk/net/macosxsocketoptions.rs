use ristretto_classfile::VersionSpecification::{Equal, GreaterThanOrEqual};
use ristretto_classfile::{JAVA_11, JAVA_17, JAVA_21};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "jdk/net/MacOSXSocketOptions.getIpDontFragment0(IZ)Z",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn get_ip_dont_fragment_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _is_ipv6 = parameters.pop_bool()?;
    let _fd = parameters.pop_int()?;
    Ok(Some(Value::from(false)))
}

#[intrinsic_method(
    "jdk/net/MacOSXSocketOptions.getSoPeerCred0(I)J",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn get_so_peer_cred_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _fd = parameters.pop_int()?;
    Ok(Some(Value::Long(0)))
}

#[intrinsic_method(
    "jdk/net/MacOSXSocketOptions.getTcpKeepAliveIntvl0(I)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_tcp_keep_alive_intvl_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _fd = parameters.pop_int()?;
    Ok(Some(Value::Int(75)))
}

#[intrinsic_method(
    "jdk/net/MacOSXSocketOptions.getTcpKeepAliveProbes0(I)I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn get_tcp_keep_alive_probes_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _fd = parameters.pop_int()?;
    Ok(Some(Value::Int(8)))
}

#[intrinsic_method(
    "jdk/net/MacOSXSocketOptions.getTcpKeepAliveTime0(I)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_tcp_keep_alive_time_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _fd = parameters.pop_int()?;
    Ok(Some(Value::Int(7200)))
}

#[intrinsic_method(
    "jdk/net/MacOSXSocketOptions.getTcpkeepAliveProbes0(I)I",
    Equal(JAVA_11)
)]
#[async_method]
pub async fn get_tcpkeep_alive_probes_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _fd = parameters.pop_int()?;
    Ok(Some(Value::Int(8)))
}

#[intrinsic_method(
    "jdk/net/MacOSXSocketOptions.ipDontFragmentSupported0()Z",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn ip_dont_fragment_supported_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::from(false)))
}

#[intrinsic_method(
    "jdk/net/MacOSXSocketOptions.keepAliveOptionsSupported0()Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn keep_alive_options_supported_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::from(true)))
}

#[intrinsic_method(
    "jdk/net/MacOSXSocketOptions.setIpDontFragment0(IZZ)V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn set_ip_dont_fragment_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _dont_fragment = parameters.pop_bool()?;
    let _is_ipv6 = parameters.pop_bool()?;
    let _fd = parameters.pop_int()?;
    Ok(None)
}

#[intrinsic_method(
    "jdk/net/MacOSXSocketOptions.setTcpKeepAliveIntvl0(II)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn set_tcp_keep_alive_intvl_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _value = parameters.pop_int()?;
    let _fd = parameters.pop_int()?;
    Ok(None)
}

#[intrinsic_method(
    "jdk/net/MacOSXSocketOptions.setTcpKeepAliveProbes0(II)V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn set_tcp_keep_alive_probes_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _value = parameters.pop_int()?;
    let _fd = parameters.pop_int()?;
    Ok(None)
}

#[intrinsic_method(
    "jdk/net/MacOSXSocketOptions.setTcpKeepAliveTime0(II)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn set_tcp_keep_alive_time_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _value = parameters.pop_int()?;
    let _fd = parameters.pop_int()?;
    Ok(None)
}

#[intrinsic_method(
    "jdk/net/MacOSXSocketOptions.setTcpkeepAliveProbes0(II)V",
    Equal(JAVA_11)
)]
#[async_method]
pub async fn set_tcpkeep_alive_probes_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _value = parameters.pop_int()?;
    let _fd = parameters.pop_int()?;
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_ip_dont_fragment_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_ip_dont_fragment_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_so_peer_cred_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_so_peer_cred_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_tcp_keep_alive_intvl_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_tcp_keep_alive_intvl_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_tcp_keep_alive_probes_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_tcp_keep_alive_probes_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_tcp_keep_alive_time_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_tcp_keep_alive_time_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_tcpkeep_alive_probes_0() {
        let (_vm, thread) = crate::test::java11_thread().await.expect("thread");
        let result = get_tcpkeep_alive_probes_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_ip_dont_fragment_supported_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = ip_dont_fragment_supported_0(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::from(false)));
        Ok(())
    }

    #[tokio::test]
    async fn test_keep_alive_options_supported_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = keep_alive_options_supported_0(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::from(true)));
        Ok(())
    }

    #[tokio::test]
    async fn test_set_ip_dont_fragment_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_ip_dont_fragment_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_tcp_keep_alive_intvl_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_tcp_keep_alive_intvl_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_tcp_keep_alive_probes_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_tcp_keep_alive_probes_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_tcp_keep_alive_time_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_tcp_keep_alive_time_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_tcpkeep_alive_probes_0() {
        let (_vm, thread) = crate::test::java11_thread().await.expect("thread");
        let result = set_tcpkeep_alive_probes_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }
}
