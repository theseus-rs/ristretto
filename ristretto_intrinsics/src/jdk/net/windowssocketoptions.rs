use ristretto_classfile::JAVA_11;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "jdk/net/WindowsSocketOptions.getIpDontFragment0(IZ)Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_ip_dont_fragment_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _is_ipv6 = parameters.pop_bool()?;
    let _fd = parameters.pop_int()?;
    Ok(Some(Value::from(false)))
}

#[intrinsic_method(
    "jdk/net/WindowsSocketOptions.getTcpKeepAliveIntvl0(I)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_tcp_keep_alive_intvl_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _fd = parameters.pop_int()?;
    Ok(Some(Value::Int(1)))
}

#[intrinsic_method(
    "jdk/net/WindowsSocketOptions.getTcpKeepAliveProbes0(I)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_tcp_keep_alive_probes_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _fd = parameters.pop_int()?;
    Ok(Some(Value::Int(10)))
}

#[intrinsic_method(
    "jdk/net/WindowsSocketOptions.getTcpKeepAliveTime0(I)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_tcp_keep_alive_time_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _fd = parameters.pop_int()?;
    Ok(Some(Value::Int(7200)))
}

#[intrinsic_method(
    "jdk/net/WindowsSocketOptions.keepAliveOptionsSupported0()Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn keep_alive_options_supported_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::from(true)))
}

#[intrinsic_method(
    "jdk/net/WindowsSocketOptions.setIpDontFragment0(IZZ)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn set_ip_dont_fragment_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _dont_fragment = parameters.pop_bool()?;
    let _is_ipv6 = parameters.pop_bool()?;
    let _fd = parameters.pop_int()?;
    Ok(None)
}

#[intrinsic_method(
    "jdk/net/WindowsSocketOptions.setTcpKeepAliveIntvl0(II)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn set_tcp_keep_alive_intvl_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _value = parameters.pop_int()?;
    let _fd = parameters.pop_int()?;
    Ok(None)
}

#[intrinsic_method(
    "jdk/net/WindowsSocketOptions.setTcpKeepAliveProbes0(II)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn set_tcp_keep_alive_probes_0<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _value = parameters.pop_int()?;
    let _fd = parameters.pop_int()?;
    Ok(None)
}

#[intrinsic_method(
    "jdk/net/WindowsSocketOptions.setTcpKeepAliveTime0(II)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn set_tcp_keep_alive_time_0<T: ristretto_types::Thread + 'static>(
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
}
