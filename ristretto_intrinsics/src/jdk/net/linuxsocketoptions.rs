#[cfg(not(target_family = "wasm"))]
use ristretto_classfile::VersionSpecification::Equal;
use ristretto_classfile::VersionSpecification::GreaterThanOrEqual;
use ristretto_classfile::{JAVA_11, JAVA_17, JAVA_21};
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
#[cfg(target_os = "linux")]
use ristretto_types::JavaError;
use ristretto_types::Thread;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "jdk/net/LinuxSocketOptions.getIpDontFragment0(IZ)Z",
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
    "jdk/net/LinuxSocketOptions.getQuickAck0(I)Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_quick_ack_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _fd = parameters.pop_int()?;
    Ok(Some(Value::from(false)))
}

#[intrinsic_method(
    "jdk/net/LinuxSocketOptions.getSoPeerCred0(I)J",
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
    "jdk/net/LinuxSocketOptions.getTcpKeepAliveIntvl0(I)I",
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

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "jdk/net/LinuxSocketOptions.getTcpKeepAliveProbes0(I)I",
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
    "jdk/net/LinuxSocketOptions.getTcpKeepAliveTime0(I)I",
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
    "jdk/net/LinuxSocketOptions.keepAliveOptionsSupported0()Z",
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
    "jdk/net/LinuxSocketOptions.quickAckSupported0()Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn quick_ack_supported_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::from(true)))
}

#[intrinsic_method(
    "jdk/net/LinuxSocketOptions.setIpDontFragment0(IZZ)V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_method]
pub async fn set_ip_dont_fragment_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _is_ipv6 = parameters.pop_bool()?;
    let _dont_fragment = parameters.pop_bool()?;
    let _fd = parameters.pop_int()?;
    Ok(None)
}

#[intrinsic_method(
    "jdk/net/LinuxSocketOptions.setQuickAck0(IZ)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn set_quick_ack_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _on = parameters.pop_bool()?;
    let _fd = parameters.pop_int()?;
    Ok(None)
}

#[intrinsic_method(
    "jdk/net/LinuxSocketOptions.setTcpKeepAliveIntvl0(II)V",
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

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "jdk/net/LinuxSocketOptions.setTcpKeepAliveProbes0(II)V",
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
    "jdk/net/LinuxSocketOptions.setTcpKeepAliveTime0(II)V",
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

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "jdk/net/LinuxSocketOptions.incomingNapiIdSupported0()Z",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn incoming_napi_id_supported_0<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::from(false)))
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "jdk/net/LinuxSocketOptions.getIncomingNapiId0(I)I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_method]
pub async fn get_incoming_napi_id_0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _fd = parameters.pop_int()?;
    Ok(Some(Value::Int(0)))
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "jdk/net/LinuxSocketOptions.getTcpkeepAliveProbes0(I)I",
    Equal(JAVA_11)
)]
#[async_method]
pub async fn get_tcpkeep_alive_probes0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "jdk/net/LinuxSocketOptions.getTcpkeepAliveProbes0(I)I".to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "jdk/net/LinuxSocketOptions.setTcpkeepAliveProbes0(II)V",
    Equal(JAVA_11)
)]
#[async_method]
pub async fn set_tcpkeep_alive_probes0<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _optval = parameters.pop_int()?;
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "jdk/net/LinuxSocketOptions.setTcpkeepAliveProbes0(II)V".to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method("jdk/net/LinuxSocketOptions.getIncomingNapiId0(I)I", Equal(JAVA_17))]
#[async_method]
pub async fn get_incoming_napi_id0_linux_v17<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "jdk/net/LinuxSocketOptions.getIncomingNapiId0(I)I".to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "jdk/net/LinuxSocketOptions.getTcpkeepAliveProbes0(I)I",
    Equal(JAVA_11)
)]
#[async_method]
pub async fn get_tcpkeep_alive_probes0_linux_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "jdk/net/LinuxSocketOptions.getTcpkeepAliveProbes0(I)I".to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "jdk/net/LinuxSocketOptions.incomingNapiIdSupported0()Z",
    Equal(JAVA_17)
)]
#[async_method]
pub async fn incoming_napi_id_supported0_linux_v17<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Err(JavaError::UnsatisfiedLinkError(
        "jdk/net/LinuxSocketOptions.incomingNapiIdSupported0()Z".to_string(),
    )
    .into())
}

#[cfg(target_os = "linux")]
#[intrinsic_method(
    "jdk/net/LinuxSocketOptions.setTcpkeepAliveProbes0(II)V",
    Equal(JAVA_11)
)]
#[async_method]
pub async fn set_tcpkeep_alive_probes0_linux_v11<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let _optval = parameters.pop_int()?;
    let _fd = parameters.pop_int()?;
    Err(JavaError::UnsatisfiedLinkError(
        "jdk/net/LinuxSocketOptions.setTcpkeepAliveProbes0(II)V".to_string(),
    )
    .into())
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
    async fn test_get_quick_ack_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_quick_ack_0(thread, Parameters::default()).await;
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
    async fn test_keep_alive_options_supported_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = keep_alive_options_supported_0(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::from(true)));
        Ok(())
    }

    #[tokio::test]
    async fn test_quick_ack_supported_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = quick_ack_supported_0(thread, Parameters::default()).await?;
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
    async fn test_set_quick_ack_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_quick_ack_0(thread, Parameters::default()).await;
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
    async fn test_incoming_napi_id_supported_0() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = incoming_napi_id_supported_0(thread, Parameters::default()).await?;
        assert_eq!(result, Some(Value::from(false)));
        Ok(())
    }

    #[tokio::test]
    async fn test_get_incoming_napi_id_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_incoming_napi_id_0(thread, Parameters::default()).await;
        assert!(result.is_err());
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_get_tcpkeep_alive_probes0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = get_tcpkeep_alive_probes0(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "jdk/net/LinuxSocketOptions.getTcpkeepAliveProbes0(I)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_set_tcpkeep_alive_probes0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            set_tcpkeep_alive_probes0(thread, Parameters::new(vec![Value::Int(0), Value::Int(0)]))
                .await;
        assert_eq!(
            "jdk/net/LinuxSocketOptions.setTcpkeepAliveProbes0(II)V",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_get_incoming_napi_id0_linux_v17() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            get_incoming_napi_id0_linux_v17(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "jdk/net/LinuxSocketOptions.getIncomingNapiId0(I)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_get_tcpkeep_alive_probes0_linux_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result =
            get_tcpkeep_alive_probes0_linux_v11(thread, Parameters::new(vec![Value::Int(0)])).await;
        assert_eq!(
            "jdk/net/LinuxSocketOptions.getTcpkeepAliveProbes0(I)I",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_incoming_napi_id_supported0_linux_v17() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = incoming_napi_id_supported0_linux_v17(thread, Parameters::default()).await;
        assert_eq!(
            "jdk/net/LinuxSocketOptions.incomingNapiIdSupported0()Z",
            result.unwrap_err().to_string()
        );
    }

    #[cfg(target_os = "linux")]
    #[tokio::test]
    async fn test_set_tcpkeep_alive_probes0_linux_v11() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let result = set_tcpkeep_alive_probes0_linux_v11(
            thread,
            Parameters::new(vec![Value::Int(0), Value::Int(0)]),
        )
        .await;
        assert_eq!(
            "jdk/net/LinuxSocketOptions.setTcpkeepAliveProbes0(II)V",
            result.unwrap_err().to_string()
        );
    }
}
