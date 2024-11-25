use crate::arguments::Arguments;
use crate::native_methods::registry::MethodRegistry;
use crate::thread::Thread;
use crate::Result;
use async_recursion::async_recursion;
use ristretto_classfile::Version;
use ristretto_classloader::Value;
use std::sync::Arc;

const JAVA_11: Version = Version::Java11 { minor: 0 };
const JAVA_18: Version = Version::Java18 { minor: 0 };

/// Register all native methods for `jdk.net.MacOSXSocketOptions`.
pub(crate) fn register(registry: &mut MethodRegistry) {
    let class_name = "jdk/net/MacOSXSocketOptions";
    let java_version = registry.java_version().clone();

    if java_version <= JAVA_11 || java_version >= JAVA_18 {
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
    } else {
        registry.register(class_name, "getSoPeerCred0", "(I)J", get_so_peer_cred_0);
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

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_so_peer_cred_0(_thread: Arc<Thread>, _arguments: Arguments) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_tcp_keep_alive_intvl_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_tcp_keep_alive_probes_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_tcp_keep_alive_time_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn get_tcpkeep_alive_probes_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn keep_alive_options_supported_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn set_tcp_keep_alive_intvl_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn set_tcp_keep_alive_probes_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn set_tcp_keep_alive_time_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}

#[expect(clippy::needless_pass_by_value)]
#[async_recursion(?Send)]
async fn set_tcpkeep_alive_probes_0(
    _thread: Arc<Thread>,
    _arguments: Arguments,
) -> Result<Option<Value>> {
    todo!()
}
