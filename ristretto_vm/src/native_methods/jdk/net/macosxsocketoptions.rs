use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_11: Version = Version::Java11 { minor: 0 };
const JAVA_17: Version = Version::Java17 { minor: 0 };
const JAVA_18: Version = Version::Java18 { minor: 0 };
const JAVA_19: Version = Version::Java19 { minor: 0 };
const JAVA_20: Version = Version::Java20 { minor: 0 };
const JAVA_21: Version = Version::Java21 { minor: 0 };

/// Register all native methods for `jdk.net.MacOSXSocketOptions`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "jdk/net/MacOSXSocketOptions";
    let java_version = registry.java_version().clone();

    if java_version >= JAVA_17 {
        registry.register(class_name, "getSoPeerCred0", "(I)J", get_so_peer_cred_0);
    }

    if java_version == JAVA_19 {
        registry.register(
            class_name,
            "getIpDontFragment0",
            "(I)Z",
            get_ip_dont_fragment_0,
        );
        registry.register(
            class_name,
            "setIpDontFragment0",
            "(IZ)V",
            set_ip_dont_fragment_0,
        );
    }
    if java_version >= JAVA_19 {
        registry.register(
            class_name,
            "ipDontFragmentSupported0",
            "()Z",
            ip_dont_fragment_supported_0,
        );
    }

    if java_version >= JAVA_20 {
        registry.register(
            class_name,
            "getIpDontFragment0",
            "(IZ)Z",
            get_ip_dont_fragment_0,
        );
        registry.register(
            class_name,
            "setIpDontFragment0",
            "(IZZ)V",
            set_ip_dont_fragment_0,
        );
    }

    if java_version <= JAVA_11 || (java_version >= JAVA_18 && java_version <= JAVA_20) {
        registry.register(
            class_name,
            "getTcpkeepAliveProbes0",
            "(I)I",
            get_tcpkeep_alive_probes_0,
        );
        registry.register(
            class_name,
            "setTcpkeepAliveProbes0",
            "(II)V",
            set_tcpkeep_alive_probes_0,
        );
    }

    if java_version == JAVA_17 || java_version >= JAVA_21 {
        registry.register(
            class_name,
            "getTcpKeepAliveProbes0",
            "(I)I",
            get_tcp_keep_alive_probes_0,
        );
        registry.register(
            class_name,
            "setTcpKeepAliveProbes0",
            "(II)V",
            set_tcp_keep_alive_probes_0,
        );
    }

    registry.register(
        class_name,
        "getTcpKeepAliveIntvl0",
        "(I)I",
        get_tcp_keep_alive_intvl_0,
    );
    registry.register(
        class_name,
        "getTcpKeepAliveTime0",
        "(I)I",
        get_tcp_keep_alive_time_0,
    );
    registry.register(
        class_name,
        "keepAliveOptionsSupported0",
        "()Z",
        keep_alive_options_supported_0,
    );
    registry.register(
        class_name,
        "setTcpKeepAliveIntvl0",
        "(II)V",
        set_tcp_keep_alive_intvl_0,
    );
}

#[async_recursion(?Send)]
async fn get_ip_dont_fragment_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.net.MacOSXSocketOptions.getIpDontFragment0(I)Z")
}

#[async_recursion(?Send)]
async fn get_so_peer_cred_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!("jdk.net.MacOSXSocketOptions.getSoPeerCred0(I)J")
}

#[async_recursion(?Send)]
async fn get_tcp_keep_alive_intvl_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.net.MacOSXSocketOptions.getTcpKeepAliveIntvl0(I)I")
}

#[async_recursion(?Send)]
async fn get_tcp_keep_alive_probes_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.net.MacOSXSocketOptions.getTcpKeepAliveProbes0(I)I")
}

#[async_recursion(?Send)]
async fn get_tcp_keep_alive_time_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.net.MacOSXSocketOptions.getTcpKeepAliveTime0(I)I")
}

#[async_recursion(?Send)]
async fn get_tcpkeep_alive_probes_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.net.MacOSXSocketOptions.getTcpkeepAliveProbes0(I)I")
}

#[async_recursion(?Send)]
async fn ip_dont_fragment_supported_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.net.MacOSXSocketOptions.ipDontFragmentSupported0()Z")
}

#[async_recursion(?Send)]
async fn keep_alive_options_supported_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.net.MacOSXSocketOptions.keepAliveOptionsSupported0()Z")
}

#[async_recursion(?Send)]
async fn set_ip_dont_fragment_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.net.MacOSXSocketOptions.setIpDontFragment0(IZ)V")
}

#[async_recursion(?Send)]
async fn set_tcp_keep_alive_intvl_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.net.MacOSXSocketOptions.setTcpKeepAliveIntvl0(II)V")
}

#[async_recursion(?Send)]
async fn set_tcp_keep_alive_probes_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.net.MacOSXSocketOptions.setTcpKeepAliveProbes0(II)V")
}

#[async_recursion(?Send)]
async fn set_tcp_keep_alive_time_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.net.MacOSXSocketOptions.setTcpKeepAliveTime0(II)V")
}

#[async_recursion(?Send)]
async fn set_tcpkeep_alive_probes_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!("jdk.net.MacOSXSocketOptions.setTcpkeepAliveProbes0(II)V")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register() {
        let mut registry = MethodRegistry::new(&Version::Java21 { minor: 0 }, true);
        register(&mut registry);
        let class_name = "jdk/net/MacOSXSocketOptions";
        assert!(registry
            .method(class_name, "getSoPeerCred0", "(I)J")
            .is_some());
        assert!(registry
            .method(class_name, "ipDontFragmentSupported0", "()Z")
            .is_some());
        assert!(registry
            .method(class_name, "getTcpKeepAliveProbes0", "(I)I")
            .is_some());
        assert!(registry
            .method(class_name, "setTcpKeepAliveProbes0", "(II)V")
            .is_some());
        assert!(registry
            .method(class_name, "getTcpKeepAliveIntvl0", "(I)I")
            .is_some());
        assert!(registry
            .method(class_name, "getTcpKeepAliveTime0", "(I)I")
            .is_some());
        assert!(registry
            .method(class_name, "keepAliveOptionsSupported0", "()Z")
            .is_some());
        assert!(registry
            .method(class_name, "setTcpKeepAliveIntvl0", "(II)V")
            .is_some());
    }

    #[test]
    fn test_register_java_19() {
        let mut registry = MethodRegistry::new(&Version::Java19 { minor: 0 }, true);
        register(&mut registry);
        let class_name = "jdk/net/MacOSXSocketOptions";
        assert!(registry
            .method(class_name, "getIpDontFragment0", "(I)Z")
            .is_some());
        assert!(registry
            .method(class_name, "setIpDontFragment0", "(IZ)V")
            .is_some());
    }

    #[test]
    fn test_register_java_20() {
        let mut registry = MethodRegistry::new(&Version::Java20 { minor: 0 }, true);
        register(&mut registry);
        let class_name = "jdk/net/MacOSXSocketOptions";
        assert!(registry
            .method(class_name, "getIpDontFragment0", "(IZ)Z")
            .is_some());
        assert!(registry
            .method(class_name, "setIpDontFragment0", "(IZZ)V")
            .is_some());
        assert!(registry
            .method(class_name, "getTcpkeepAliveProbes0", "(I)I")
            .is_some());
        assert!(registry
            .method(class_name, "setTcpkeepAliveProbes0", "(II)V")
            .is_some());
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.net.MacOSXSocketOptions.getIpDontFragment0(I)Z"
    )]
    async fn test_get_ip_dont_fragment_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_ip_dont_fragment_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.net.MacOSXSocketOptions.getSoPeerCred0(I)J"
    )]
    async fn test_get_so_peer_cred_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_so_peer_cred_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.net.MacOSXSocketOptions.getTcpKeepAliveIntvl0(I)I"
    )]
    async fn test_get_tcp_keep_alive_intvl_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_tcp_keep_alive_intvl_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.net.MacOSXSocketOptions.getTcpKeepAliveProbes0(I)I"
    )]
    async fn test_get_tcp_keep_alive_probes_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_tcp_keep_alive_probes_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.net.MacOSXSocketOptions.getTcpKeepAliveTime0(I)I"
    )]
    async fn test_get_tcp_keep_alive_time_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_tcp_keep_alive_time_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.net.MacOSXSocketOptions.getTcpkeepAliveProbes0(I)I"
    )]
    async fn test_get_tcpkeep_alive_probes_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = get_tcpkeep_alive_probes_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.net.MacOSXSocketOptions.ipDontFragmentSupported0()Z"
    )]
    async fn test_ip_dont_fragment_supported_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = ip_dont_fragment_supported_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.net.MacOSXSocketOptions.keepAliveOptionsSupported0()Z"
    )]
    async fn test_keep_alive_options_supported_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = keep_alive_options_supported_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.net.MacOSXSocketOptions.setIpDontFragment0(IZ)V"
    )]
    async fn test_set_ip_dont_fragment_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_ip_dont_fragment_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.net.MacOSXSocketOptions.setTcpKeepAliveIntvl0(II)V"
    )]
    async fn test_set_tcp_keep_alive_intvl_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_tcp_keep_alive_intvl_0(thread, Arguments::default()).await;
    }

    #[tokio::test]
    #[should_panic(
        expected = "not yet implemented: jdk.net.MacOSXSocketOptions.setTcpKeepAliveProbes0(II)V"
    )]
    async fn test_set_tcp_keep_alive_probes_0() {
        let (_vm, thread) = crate::test::thread().await.expect("thread");
        let _ = set_tcp_keep_alive_probes_0(thread, Arguments::default()).await;
    }
}
