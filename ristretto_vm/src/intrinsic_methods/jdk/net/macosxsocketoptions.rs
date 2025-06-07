use crate::Result;
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classfile::VersionSpecification::{Equal, GreaterThanOrEqual};
use ristretto_classfile::{JAVA_11, JAVA_17, JAVA_21};
use ristretto_classloader::Value;
use ristretto_macros::intrinsic_method;
use std::sync::Arc;

#[intrinsic_method(
    "jdk/net/MacOSXSocketOptions.getIpDontFragment0(IZ)Z",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_ip_dont_fragment_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.net.MacOSXSocketOptions.getIpDontFragment0(IZ)Z")
}

#[intrinsic_method(
    "jdk/net/MacOSXSocketOptions.getSoPeerCred0(I)J",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_so_peer_cred_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.net.MacOSXSocketOptions.getSoPeerCred0(I)J")
}

#[intrinsic_method(
    "jdk/net/MacOSXSocketOptions.getTcpKeepAliveIntvl0(I)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_tcp_keep_alive_intvl_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.net.MacOSXSocketOptions.getTcpKeepAliveIntvl0(I)I")
}

#[intrinsic_method(
    "jdk/net/MacOSXSocketOptions.getTcpKeepAliveProbes0(I)I",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_tcp_keep_alive_probes_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.net.MacOSXSocketOptions.getTcpKeepAliveProbes0(I)I")
}

#[intrinsic_method(
    "jdk/net/MacOSXSocketOptions.getTcpKeepAliveTime0(I)I",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_tcp_keep_alive_time_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.net.MacOSXSocketOptions.getTcpKeepAliveTime0(I)I")
}

#[intrinsic_method(
    "jdk/net/MacOSXSocketOptions.getTcpkeepAliveProbes0(I)I",
    Equal(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn get_tcpkeep_alive_probes_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.net.MacOSXSocketOptions.getTcpkeepAliveProbes0(I)I")
}

#[intrinsic_method(
    "jdk/net/MacOSXSocketOptions.ipDontFragmentSupported0()Z",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn ip_dont_fragment_supported_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.net.MacOSXSocketOptions.ipDontFragmentSupported0()Z")
}

#[intrinsic_method(
    "jdk/net/MacOSXSocketOptions.keepAliveOptionsSupported0()Z",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn keep_alive_options_supported_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.net.MacOSXSocketOptions.keepAliveOptionsSupported0()Z")
}

#[intrinsic_method(
    "jdk/net/MacOSXSocketOptions.setIpDontFragment0(IZZ)V",
    GreaterThanOrEqual(JAVA_21)
)]
#[async_recursion(?Send)]
pub(crate) async fn set_ip_dont_fragment_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.net.MacOSXSocketOptions.setIpDontFragment0(IZZ)V")
}

#[intrinsic_method(
    "jdk/net/MacOSXSocketOptions.setTcpKeepAliveIntvl0(II)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn set_tcp_keep_alive_intvl_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.net.MacOSXSocketOptions.setTcpKeepAliveIntvl0(II)V")
}

#[intrinsic_method(
    "jdk/net/MacOSXSocketOptions.setTcpKeepAliveProbes0(II)V",
    GreaterThanOrEqual(JAVA_17)
)]
#[async_recursion(?Send)]
pub(crate) async fn set_tcp_keep_alive_probes_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.net.MacOSXSocketOptions.setTcpKeepAliveProbes0(II)V")
}

#[intrinsic_method(
    "jdk/net/MacOSXSocketOptions.setTcpKeepAliveTime0(II)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn set_tcp_keep_alive_time_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.net.MacOSXSocketOptions.setTcpKeepAliveTime0(II)V")
}

#[intrinsic_method(
    "jdk/net/MacOSXSocketOptions.setTcpkeepAliveProbes0(II)V",
    Equal(JAVA_11)
)]
#[async_recursion(?Send)]
pub(crate) async fn set_tcpkeep_alive_probes_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.net.MacOSXSocketOptions.setTcpkeepAliveProbes0(II)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.net.MacOSXSocketOptions.getIpDontFragment0(IZ)Z"
    )]
    async fn test_get_ip_dont_fragment_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_ip_dont_fragment_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.net.MacOSXSocketOptions.getSoPeerCred0(I)J"
    )]
    async fn test_get_so_peer_cred_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_so_peer_cred_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.net.MacOSXSocketOptions.getTcpKeepAliveIntvl0(I)I"
    )]
    async fn test_get_tcp_keep_alive_intvl_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_tcp_keep_alive_intvl_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.net.MacOSXSocketOptions.getTcpKeepAliveProbes0(I)I"
    )]
    async fn test_get_tcp_keep_alive_probes_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_tcp_keep_alive_probes_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.net.MacOSXSocketOptions.getTcpKeepAliveTime0(I)I"
    )]
    async fn test_get_tcp_keep_alive_time_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_tcp_keep_alive_time_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.net.MacOSXSocketOptions.getTcpkeepAliveProbes0(I)I"
    )]
    async fn test_get_tcpkeep_alive_probes_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_tcpkeep_alive_probes_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.net.MacOSXSocketOptions.ipDontFragmentSupported0()Z"
    )]
    async fn test_ip_dont_fragment_supported_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ip_dont_fragment_supported_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.net.MacOSXSocketOptions.keepAliveOptionsSupported0()Z"
    )]
    async fn test_keep_alive_options_supported_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = keep_alive_options_supported_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.net.MacOSXSocketOptions.setIpDontFragment0(IZZ)V"
    )]
    async fn test_set_ip_dont_fragment_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_ip_dont_fragment_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.net.MacOSXSocketOptions.setTcpKeepAliveIntvl0(II)V"
    )]
    async fn test_set_tcp_keep_alive_intvl_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_tcp_keep_alive_intvl_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.net.MacOSXSocketOptions.setTcpKeepAliveProbes0(II)V"
    )]
    async fn test_set_tcp_keep_alive_probes_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_tcp_keep_alive_probes_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.net.MacOSXSocketOptions.setTcpKeepAliveTime0(II)V"
    )]
    async fn test_set_tcp_keep_alive_time_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_tcp_keep_alive_time_0(thread, Parameters::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.net.MacOSXSocketOptions.setTcpkeepAliveProbes0(II)V"
    )]
    async fn test_set_tcpkeep_alive_probes_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_tcpkeep_alive_probes_0(thread, Parameters::default()).await;
    }
}
