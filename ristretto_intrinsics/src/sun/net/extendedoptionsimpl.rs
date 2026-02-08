use ristretto_classfile::JAVA_8;
use ristretto_classfile::VersionSpecification::LessThanOrEqual;
use ristretto_classloader::Value;
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{Parameters, Result};
use std::sync::Arc;

#[intrinsic_method(
    "sun/net/ExtendedOptionsImpl.flowSupported()Z",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn flow_supported<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.net.ExtendedOptionsImpl.flowSupported()Z")
}

#[intrinsic_method(
    "sun/net/ExtendedOptionsImpl.getFlowOption(Ljava/io/FileDescriptor;Ljdk/net/SocketFlow;)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_flow_option<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.net.ExtendedOptionsImpl.getFlowOption(Ljava/io/FileDescriptor;Ljdk/net/SocketFlow;)V"
    )
}

#[intrinsic_method(
    "sun/net/ExtendedOptionsImpl.getTcpKeepAliveIntvl(Ljava/io/FileDescriptor;)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_tcp_keep_alive_intvl<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.net.ExtendedOptionsImpl.getTcpKeepAliveIntvl(Ljava/io/FileDescriptor;)I")
}

#[intrinsic_method(
    "sun/net/ExtendedOptionsImpl.getTcpKeepAliveProbes(Ljava/io/FileDescriptor;)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_tcp_keep_alive_probes<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.net.ExtendedOptionsImpl.getTcpKeepAliveProbes(Ljava/io/FileDescriptor;)I")
}

#[intrinsic_method(
    "sun/net/ExtendedOptionsImpl.getTcpKeepAliveTime(Ljava/io/FileDescriptor;)I",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn get_tcp_keep_alive_time<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.net.ExtendedOptionsImpl.getTcpKeepAliveTime(Ljava/io/FileDescriptor;)I")
}

#[intrinsic_method("sun/net/ExtendedOptionsImpl.init()V", LessThanOrEqual(JAVA_8))]
#[async_method]
pub async fn init<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(None)
}

#[intrinsic_method(
    "sun/net/ExtendedOptionsImpl.keepAliveOptionsSupported()Z",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn keep_alive_options_supported<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.net.ExtendedOptionsImpl.keepAliveOptionsSupported()Z")
}

#[intrinsic_method(
    "sun/net/ExtendedOptionsImpl.setFlowOption(Ljava/io/FileDescriptor;Ljdk/net/SocketFlow;)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn set_flow_option<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!(
        "sun.net.ExtendedOptionsImpl.setFlowOption(Ljava/io/FileDescriptor;Ljdk/net/SocketFlow;)V"
    )
}

#[intrinsic_method(
    "sun/net/ExtendedOptionsImpl.setTcpKeepAliveIntvl(Ljava/io/FileDescriptor;I)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn set_tcp_keep_alive_intvl<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.net.ExtendedOptionsImpl.setTcpKeepAliveIntvl(Ljava/io/FileDescriptor;I)V")
}

#[intrinsic_method(
    "sun/net/ExtendedOptionsImpl.setTcpKeepAliveProbes(Ljava/io/FileDescriptor;I)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn set_tcp_keep_alive_probes<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.net.ExtendedOptionsImpl.setTcpKeepAliveProbes(Ljava/io/FileDescriptor;I)V")
}

#[intrinsic_method(
    "sun/net/ExtendedOptionsImpl.setTcpKeepAliveTime(Ljava/io/FileDescriptor;I)V",
    LessThanOrEqual(JAVA_8)
)]
#[async_method]
pub async fn set_tcp_keep_alive_time<T: ristretto_types::Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("sun.net.ExtendedOptionsImpl.setTcpKeepAliveTime(Ljava/io/FileDescriptor;I)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: sun.net.ExtendedOptionsImpl.flowSupported()Z")]
    async fn test_flow_supported() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = flow_supported(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.net.ExtendedOptionsImpl.getFlowOption(Ljava/io/FileDescriptor;Ljdk/net/SocketFlow;)V"
    )]
    async fn test_get_flow_option() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_flow_option(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.net.ExtendedOptionsImpl.getTcpKeepAliveIntvl(Ljava/io/FileDescriptor;)I"
    )]
    async fn test_get_tcp_keep_alive_intvl() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_tcp_keep_alive_intvl(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.net.ExtendedOptionsImpl.getTcpKeepAliveProbes(Ljava/io/FileDescriptor;)I"
    )]
    async fn test_get_tcp_keep_alive_probes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_tcp_keep_alive_probes(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.net.ExtendedOptionsImpl.getTcpKeepAliveTime(Ljava/io/FileDescriptor;)I"
    )]
    async fn test_get_tcp_keep_alive_time() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_tcp_keep_alive_time(thread, Parameters::default()).await;
    }

    #[tokio::test]
    async fn test_init() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init(thread, Parameters::default()).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.net.ExtendedOptionsImpl.keepAliveOptionsSupported()Z"
    )]
    async fn test_keep_alive_options_supported() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = keep_alive_options_supported(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.net.ExtendedOptionsImpl.setFlowOption(Ljava/io/FileDescriptor;Ljdk/net/SocketFlow;)V"
    )]
    async fn test_set_flow_option() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_flow_option(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.net.ExtendedOptionsImpl.setTcpKeepAliveIntvl(Ljava/io/FileDescriptor;I)V"
    )]
    async fn test_set_tcp_keep_alive_intvl() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_tcp_keep_alive_intvl(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.net.ExtendedOptionsImpl.setTcpKeepAliveProbes(Ljava/io/FileDescriptor;I)V"
    )]
    async fn test_set_tcp_keep_alive_probes() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_tcp_keep_alive_probes(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: sun.net.ExtendedOptionsImpl.setTcpKeepAliveTime(Ljava/io/FileDescriptor;I)V"
    )]
    async fn test_set_tcp_keep_alive_time() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_tcp_keep_alive_time(thread, Parameters::default()).await;
    }
}
