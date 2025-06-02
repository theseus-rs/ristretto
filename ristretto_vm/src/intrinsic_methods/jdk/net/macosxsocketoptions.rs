use crate::Result;
use crate::intrinsic_methods::registry::{JAVA_11, JAVA_17, JAVA_21, MethodRegistry};
use crate::parameters::Parameters;
use crate::thread::Thread;
use async_recursion::async_recursion;
use ristretto_classloader::Value;
use std::sync::Arc;

const CLASS_NAME: &str = "jdk/net/MacOSXSocketOptions";

/// Register all intrinsic methods for `jdk.net.MacOSXSocketOptions`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    if registry.java_major_version() <= JAVA_11 {
        registry.register(
            CLASS_NAME,
            "getTcpkeepAliveProbes0",
            "(I)I",
            get_tcpkeep_alive_probes_0,
        );
        registry.register(
            CLASS_NAME,
            "setTcpkeepAliveProbes0",
            "(II)V",
            set_tcpkeep_alive_probes_0,
        );
    }

    if registry.java_major_version() >= JAVA_17 {
        registry.register(CLASS_NAME, "getSoPeerCred0", "(I)J", get_so_peer_cred_0);
        registry.register(
            CLASS_NAME,
            "getTcpKeepAliveProbes0",
            "(I)I",
            get_tcp_keep_alive_probes_0,
        );
        registry.register(
            CLASS_NAME,
            "setTcpKeepAliveProbes0",
            "(II)V",
            set_tcp_keep_alive_probes_0,
        );
    }

    if registry.java_major_version() >= JAVA_21 {
        registry.register(
            CLASS_NAME,
            "getIpDontFragment0",
            "(IZ)Z",
            get_ip_dont_fragment_0,
        );
        registry.register(
            CLASS_NAME,
            "ipDontFragmentSupported0",
            "()Z",
            ip_dont_fragment_supported_0,
        );
        registry.register(
            CLASS_NAME,
            "setIpDontFragment0",
            "(IZZ)V",
            set_ip_dont_fragment_0,
        );
    }

    registry.register(
        CLASS_NAME,
        "getTcpKeepAliveIntvl0",
        "(I)I",
        get_tcp_keep_alive_intvl_0,
    );
    registry.register(
        CLASS_NAME,
        "getTcpKeepAliveTime0",
        "(I)I",
        get_tcp_keep_alive_time_0,
    );
    registry.register(
        CLASS_NAME,
        "keepAliveOptionsSupported0",
        "()Z",
        keep_alive_options_supported_0,
    );
    registry.register(
        CLASS_NAME,
        "setTcpKeepAliveIntvl0",
        "(II)V",
        set_tcp_keep_alive_intvl_0,
    );
    registry.register(
        CLASS_NAME,
        "setTcpKeepAliveTime0",
        "(II)V",
        set_tcp_keep_alive_time_0,
    );
}

#[async_recursion(?Send)]
async fn get_ip_dont_fragment_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.net.MacOSXSocketOptions.getIpDontFragment0(I)Z")
}

#[async_recursion(?Send)]
async fn get_so_peer_cred_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.net.MacOSXSocketOptions.getSoPeerCred0(I)J")
}

#[async_recursion(?Send)]
async fn get_tcp_keep_alive_intvl_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.net.MacOSXSocketOptions.getTcpKeepAliveIntvl0(I)I")
}

#[async_recursion(?Send)]
async fn get_tcp_keep_alive_probes_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.net.MacOSXSocketOptions.getTcpKeepAliveProbes0(I)I")
}

#[async_recursion(?Send)]
async fn get_tcp_keep_alive_time_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.net.MacOSXSocketOptions.getTcpKeepAliveTime0(I)I")
}

#[async_recursion(?Send)]
async fn get_tcpkeep_alive_probes_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.net.MacOSXSocketOptions.getTcpkeepAliveProbes0(I)I")
}

#[async_recursion(?Send)]
async fn ip_dont_fragment_supported_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.net.MacOSXSocketOptions.ipDontFragmentSupported0()Z")
}

#[async_recursion(?Send)]
async fn keep_alive_options_supported_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.net.MacOSXSocketOptions.keepAliveOptionsSupported0()Z")
}

#[async_recursion(?Send)]
async fn set_ip_dont_fragment_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.net.MacOSXSocketOptions.setIpDontFragment0(IZ)V")
}

#[async_recursion(?Send)]
async fn set_tcp_keep_alive_intvl_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.net.MacOSXSocketOptions.setTcpKeepAliveIntvl0(II)V")
}

#[async_recursion(?Send)]
async fn set_tcp_keep_alive_probes_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.net.MacOSXSocketOptions.setTcpKeepAliveProbes0(II)V")
}

#[async_recursion(?Send)]
async fn set_tcp_keep_alive_time_0(
    _thread: Arc<Thread>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    todo!("jdk.net.MacOSXSocketOptions.setTcpKeepAliveTime0(II)V")
}

#[async_recursion(?Send)]
async fn set_tcpkeep_alive_probes_0(
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
        expected = "not yet implemented: jdk.net.MacOSXSocketOptions.getIpDontFragment0(I)Z"
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
        expected = "not yet implemented: jdk.net.MacOSXSocketOptions.setIpDontFragment0(IZ)V"
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
